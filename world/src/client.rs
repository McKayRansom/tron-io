use crate::{
    grid::{Grid, bike::BikeUpdate},
    PLAYER_MAX,
};

use super::{ClientConnection, ClientMsg, GridUpdateMsg, WorldState};

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

    pub fn handle_input(&mut self, action: super::Action) {
        match self.game_state {
            WorldState::Waiting | WorldState::RoundOver(_) | WorldState::GameOver(_) => {
                if action == super::Action::Confirm && !self.ready {
                    self.ready = true;
                    log::info!("Ready!");
                    self.connection.send(&ClientMsg {
                        ready: true,
                        state: self.game_state,
                        update: None,
                    });
                }
            }
            WorldState::Playing => {
                if let Some(player_id) = self.player_id {
                    if let Some(bike_update) =
                        self.grid.bikes[player_id as usize].handle_action(action)
                    {
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
        }
    }

    pub fn update(&mut self) {
        while let Some(server_msg) = self.connection.try_recv() {
            log::debug!("Received server message: {:?}", server_msg);
            self.player_id = Some(server_msg.id);
            if self.game_state != server_msg.state {
                self.ready = false;
                self.game_state = server_msg.state;
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
                log::info!("Game state changed to {:?}", self.game_state);
                self.connection.send(&ClientMsg {
                    ready: false,
                    state: self.game_state,
                    update: None,
                });
            }
            if self.game_state == WorldState::Playing {
                if let Some(grid_update) = server_msg.grid_update {
                    if grid_update.tick != self.grid.tick + 1 {
                        // unfortuantely, This is expected as the server sends out ticks every 10ms 
                        //  we may not get back before the server sends a duplicate
                        // log::warn!("Tick {} != {} + 1", grid_update.tick, self.grid.tick);
                    } else {
                        self.player_update = None;
                        let _ = self.grid.apply_updates(&grid_update);

                        if self.grid.hash != grid_update.hash {
                            log::error!(
                                "Hash mismatch! {} != {} at tick {}",
                                self.grid.hash,
                                grid_update.hash,
                                grid_update.tick
                            );
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
        }

        self.connection.update();
    }
}
