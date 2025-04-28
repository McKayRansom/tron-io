use crate::{
    grid::{
        Grid,
        msg::{BikeUpdate, ClientMsg, GridUpdateMsg, WorldState},
    },
    world::PLAYER_MAX,
};

use super::{Action, ClientConnection};

pub struct WorldClient {
    pub grid: Grid,
    pub scores: [u8; super::PLAYER_MAX],
    pub score_win: u8,
    pub player_id: Option<u8>,
    ready: bool,

    player_update: Option<BikeUpdate>,
    pub game_state: WorldState,

    connection: Box<dyn ClientConnection>,
}

impl WorldClient {
    pub fn new(connection: Box<dyn ClientConnection>) -> Self {
        Self {
            grid: Grid::new(),
            scores: [0; super::PLAYER_MAX],
            score_win: super::SCORE_WIN,
            player_update: None,
            game_state: WorldState::Waiting,
            player_id: None,
            ready: false,
            connection,
        }
    }

    pub fn handle_input(&mut self, action: &super::Action) {
        // wait for players
        match action {
            Action::Confirm => {
                if !self.ready {
                    self.ready = true;
                    println!("Ready!");
                    self.connection.send(&ClientMsg {
                        ready: true,
                        state: self.game_state,
                        update: None,
                    });
                }
            }
            Action::Left | Action::Right | Action::Up | Action::Down => {
                if self.game_state == WorldState::Playing {
                    if let Some(player_id) = self.player_id {
                        let new_dir = match action {
                            Action::Left => crate::grid::bike::LEFT,
                            Action::Right => crate::grid::bike::RIGHT,
                            Action::Up => crate::grid::bike::UP,
                            Action::Down => crate::grid::bike::DOWN,
                            _ => unreachable!(),
                        };

                        let current_dir = self.grid.bikes[player_id as usize].dir;

                        if new_dir == current_dir || new_dir == crate::grid::bike::invert_dir(current_dir) {
                            // println!("Player {} moved {:?}", player_id, dir);
                            return;
                        }
                        let bike_update = BikeUpdate {
                            id: player_id,
                            dir: new_dir,
                        };
                        self.player_update = Some(bike_update.clone());
                        self.connection.send(&ClientMsg {
                            ready: self.ready,
                            state: self.game_state,
                            update: Some(GridUpdateMsg {
                                tick: self.grid.tick,
                                hash: self.grid.hash,
                                updates: vec![bike_update],
                            }),
                        });
                    }
                }
            }
            _ => {
                println!("Unimplemented Action: {:?}", action);
            }
        }
    }

    // pub fn update(&mut self, context: &mut Context)
    pub fn update(&mut self) {
        // MULTILAYER
        while let Some(server_msg) = self.connection.try_recv() {
            // self.grid.apply_updates(&grid_update);
            // dbg!(&server_msg);
            self.player_id = Some(server_msg.id);
            if self.game_state != server_msg.state {
                self.game_state = server_msg.state;
                self.ready = false;
                match self.game_state {
                    WorldState::Waiting => {
                        // New game
                        self.grid = Grid::new();
                        self.scores = [0; PLAYER_MAX];
                    }
                    WorldState::Playing => {
                        // New round (potentially)
                        self.grid = Grid::new();
                    }
                    WorldState::RoundOver(winner) => self.scores[winner as usize] += 1,
                    WorldState::GameOver(winner) => self.scores[winner as usize] += 1,
                }
                println!("Game state changed to {:?}", self.game_state);
            }
            if self.game_state == WorldState::Playing {
                if let Some(grid_update) = server_msg.grid_update {
                    self.player_update = None;
                    let _ = self.grid.apply_updates(&grid_update);

                    if self.grid.hash != grid_update.hash {
                        println!("Hash mismatch! {} != {}", self.grid.hash, grid_update.hash);
                        println!("Tick mismatch! {} != {}", self.grid.tick, grid_update.tick);
                        // context.switch_scene_to = Some(crate::scene::EScene::MainMenu);
                    }

                    self.connection.send(&ClientMsg {
                        ready: false,
                        state: WorldState::Playing,
                        update: Some(GridUpdateMsg {
                            tick: self.grid.tick,
                            hash: self.grid.hash,
                            updates: vec![],
                        }),
                    });
                }
            }
        }

        self.connection.update();
    }
}
