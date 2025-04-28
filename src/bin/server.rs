use nanoserde::DeBin;
use quad_net::quad_socket::server::SocketHandle;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tron_io::grid::Grid;
use tron_io::grid::msg::{ClientMsg, GridUpdateMsg, ServerMsg, WorldState};

#[derive(Default)]
struct ClientState {
    id: Option<u8>,
    tick: u32,
    state: WorldState,
    world: Option<Arc<Mutex<World>>>,
}

impl ClientState {
    fn on_msg(
        &mut self,
        msg: ClientMsg,
        out: &mut SocketHandle,
        waiting_queue: Arc<Mutex<WaitingQueue>>,
    ) {
        if self.world.is_none() {
            let mut waiting_queue = waiting_queue.lock().unwrap();

            if let Some(world_arc) = &waiting_queue.other_waiting {
                // if there is another waiting player, send them both into the same world
                let mut world = world_arc.lock().unwrap();
                if world.world_state == WorldState::Waiting {
                    // if the world is not waiting, we can't add a player

                    if self.id.is_none() {
                        self.id = Some(world.players);
                        world.players += 1;
                    }
                    self.state = world.world_state;

                    world.world_state = WorldState::Playing;
                    println!("Adding player to world");

                    self.world = Some(world_arc.clone());
                }
            }
            if self.world.is_none() {
                // if there is no other waiting player, add this one to the queue
                let mut world = World::new();

                if self.id.is_none() {
                    self.id = Some(world.players);
                    world.players += 1;
                }
                self.state = world.world_state;

                println!("Starting new world");

                let world = Arc::new(Mutex::new(world));
                self.world = Some(world.clone());
                waiting_queue.other_waiting = Some(world);
            }
            out.send_bin(&ServerMsg {
                id: self.id.unwrap_or(0),
                state: self.state,
                grid_update: None,
            })
            .unwrap();
        }
        if let Some(world) = &self.world {
            // if there is a world, just send the message to it
            let mut world = world.lock().unwrap();
            if let Some(update) = msg.update {
                self.tick = update.tick;
                world.next_update.updates.extend_from_slice(&update.updates);
            }
            self.state = msg.state;

            if world.ready[self.id.unwrap() as usize] != msg.ready {
                println!("Player {} is ready: {}", self.id.unwrap(), msg.ready);
                world.ready[self.id.unwrap() as usize] = msg.ready;
                if msg.ready && matches!(world.world_state, WorldState::GameOver(_)) {
                    // if the player is ready and the game is over, find a new game
                    drop(world);
                    *self = ClientState::default();
                    // println!("Player {} is ready, finding new game", self.id.unwrap());
                    out.send_bin(&ServerMsg {
                        id: self.id.unwrap_or(0),
                        state: self.state,
                        grid_update: None,
                    })
                    .unwrap();
                }
            }
        }
    }

    fn on_timer(&self, out: &mut SocketHandle) {
        if let Some(world) = &self.world {
            let mut world = world.lock().unwrap();
            if self.tick != world.last_update.tick || self.state != world.world_state {
                let msg = ServerMsg {
                    id: self.id.unwrap_or(0),
                    state: world.world_state,
                    grid_update: Some(world.last_update.clone()),
                };
                // dbg!(&msg);
                out.send_bin(&msg).unwrap();
                // println!("Sending update to player {}", self.id.unwrap_or(0));
                // state.tick = world.update.tick;
            }
            // I don't love that any thread can update the world, but it works for now
            world.update();
        }
    }

    fn on_disconnect(&self) {
        println!("Client disconnected: {:?}", self.id);
    }
}

struct WaitingQueue {
    other_waiting: Option<Arc<Mutex<World>>>,
}

const PLAYER_MAX: usize = 4;
const SCORE_WIN: u8 = 3;

struct World {
    grid: Grid,
    scores: [u8; PLAYER_MAX],
    ready: [bool; PLAYER_MAX],
    score_win: u8,

    world_state: WorldState,
    // pos: (f32, f32),
    // last_edit_id: usize,
    players: u8, // doubles as player count I guess...
    last_update_time: Instant,
    next_update: GridUpdateMsg,
    last_update: GridUpdateMsg,
}

impl World {
    fn new() -> Self {
        Self {
            grid: Grid::new(),
            scores: [0; PLAYER_MAX],
            ready: [false; PLAYER_MAX],
            score_win: SCORE_WIN,
            world_state: WorldState::Waiting,
            // pos: (0.0, 0.0),
            // last_edit_id: 0,
            players: 0,
            last_update_time: Instant::now(),
            next_update: GridUpdateMsg::default(),
            last_update: GridUpdateMsg::default(),
        }
    }

    fn update(&mut self) {
        match self.world_state {
            WorldState::Waiting | WorldState::RoundOver(_) => {
                if self.players == 0 {
                    return;
                }
                for i in 0..self.players as usize {
                    if !self.ready[i] {
                        // dbg!("Waiting for player {}", i);
                        return;
                    }
                }
                self.world_state = WorldState::Playing;
                self.grid = Grid::new();
                self.last_update_time = Instant::now();
                self.next_update = GridUpdateMsg::default();
                self.last_update = GridUpdateMsg::default();
            }
            WorldState::Playing => {
                if self.last_update_time.elapsed() > Duration::from_millis(50) {
                    self.last_update_time = Instant::now();
                    self.last_update = self.next_update.clone();
                    self.next_update.updates.clear();
                    self.next_update.tick += 1;

                    match self.grid.apply_updates(&self.last_update) {
                        tron_io::grid::UpdateResult::GameOver(winner) => {
                            self.scores[winner as usize] += 1;
                            if self.scores[winner as usize] == self.score_win {
                                self.world_state = WorldState::GameOver(winner);
                                // self.game_won = winner == self.player_id.unwrap();
                            } else {
                                self.world_state = WorldState::RoundOver(winner);
                            }
                            self.ready = [false; PLAYER_MAX];
                        }
                        tron_io::grid::UpdateResult::InProgress => {}
                    }
                    self.last_update.hash = self.grid.hash;
                }
            }
            WorldState::GameOver(_) => {}
        }
    }
}

pub fn main() -> std::io::Result<()> {
    // let world = Arc::new(Mutex::new(World::new()));
    let waiting_queue = Arc::new(Mutex::new(WaitingQueue {
        other_waiting: None,
    }));

    quad_net::quad_socket::server::listen(
        "0.0.0.0:8090",
        "0.0.0.0:8091",
        quad_net::quad_socket::server::Settings {
            on_message: {
                let waiting_queue = waiting_queue.clone();
                move |out, state: &mut ClientState, msg| {
                    let msg: ClientMsg = DeBin::deserialize_bin(&msg).unwrap();
                    state.on_msg(msg, out, waiting_queue.clone());
                }
            },
            on_timer: move |out, state| {
                state.on_timer(out);
            },
            on_disconnect: |state| {
                state.on_disconnect();
            },
            timer: Some(Duration::from_millis(10)),
            _marker: std::marker::PhantomData,
        },
    );
    Ok(())
}
