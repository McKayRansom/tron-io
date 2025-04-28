use nanoserde::DeBin;
use quad_net::quad_socket::server::SocketHandle;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tron_io::grid::msg::{ClientMsg, GridUpdateMsg, ServerMsg, WorldState};

#[derive(Default)]
struct ClientState {
    id: Option<u8>,
    tick: u32,
    state: WorldState,
    world: Option<Arc<Mutex<World>>>,
}

impl ClientState {
    fn new() -> Self {
        Self {
            state: WorldState::Waiting,
            id: None,
            tick: 0,
            world: None,
        }
    }

    fn on_msg(
        &mut self,
        msg: ClientMsg,
        _out: &mut SocketHandle,
        waiting_queue: Arc<Mutex<WaitingQueue>>,
    ) {
        match &self.world {
            Some(world) => {
                // if there is a world, just send the message to it
                let mut world = world.lock().unwrap();
                if let Some(update) = msg.update {
                    self.tick = update.tick;
                    world.next_update.updates.extend_from_slice(&update.updates);
                }
                self.state = msg.state;

                if self.id.is_none() {
                    self.id = Some(world.unique_id);
                    world.unique_id += 1;
                }
            }
            None => {
                let waiting_queue = waiting_queue.lock().unwrap();

                if let Some(world_arc) = &waiting_queue.other_waiting {
                    // if there is another waiting player, send them both into the same world
                    let mut world = world_arc.lock().unwrap();

                    world.world_state = WorldState::Playing;
                    println!("Starting game with player {}", world.unique_id);

                    self.world = Some(world_arc.clone());
                } else {
                    // if there is no other waiting player, add this one to the queue
                    let mut world = Some(Arc::new(Mutex::new(World::new())));

                    //TEMP ONE PLAYER:
                    world.as_mut().unwrap().lock().unwrap().world_state = WorldState::Playing;

                    self.world = world.clone();
                    // waiting_queue.other_waiting = world;
                }
            }
        }
    }

    fn on_timer(&self, out: &mut SocketHandle) {
        if let Some(world) = &self.world {
            let mut world = world.lock().unwrap();
            match world.world_state {
                WorldState::Playing => {
                    if self.tick != world.last_update.tick {
                        let msg = ServerMsg {
                            id: self.id.unwrap_or(0),
                            state: world.world_state,
                            grid_update: Some(world.last_update.clone()),
                        };
                        dbg!(&msg);
                        out.send_bin(&msg).unwrap();
                        // println!("Sending update to player {}", self.id.unwrap_or(0));
                        // state.tick = world.update.tick;
                    }
                    // I don't love that any thread can update the world, but it works for now
                    if world.last_update_time.elapsed() > Duration::from_millis(50) {
                        world.last_update_time = Instant::now();
                        world.last_update = world.next_update.clone();
                        world.next_update.updates.clear();
                        world.next_update.tick += 1;
                        // println!("Sending update to player {}", self.id.unwrap_or(0));
                        // out.send_bin(&world.update).unwrap();
                    }
                }
                _ => {
                    // println!("Waiting for other player...");
                    return;
                }
            }
        }
    }

    fn on_disconnect(&self) {
        println!("Client disconnected: {:?}", self.id);
    }
}

struct WaitingQueue {
    other_waiting: Option<Arc<Mutex<World>>>,
}

struct World {
    world_state: WorldState,
    // pos: (f32, f32),
    // last_edit_id: usize,
    unique_id: u8, // doubles as player count I guess...
    last_update_time: Instant,
    next_update: GridUpdateMsg,
    last_update: GridUpdateMsg,
}

impl World {
    fn new() -> Self {
        Self {
            world_state: WorldState::Waiting,
            // pos: (0.0, 0.0),
            // last_edit_id: 0,
            unique_id: 1,
            last_update_time: Instant::now(),
            next_update: GridUpdateMsg {
                tick: 0,
                seed: 0,
                updates: vec![],
            },
            last_update: GridUpdateMsg {
                tick: 0,
                seed: 0,
                updates: vec![],
            },
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
