use crate::{
    ClientPlayer, ServerPlayer,
    grid::{Grid, GridUpdateMsg, UpdateResult, bike::BikeUpdate},
};

use super::WorldState;

pub mod connection;

// const PLAYER_MAX: usize = 4;
// TODO: Server settings of some kind?
const SCORE_WIN: u8 = 3;
const MIN_PLAYERS: usize = 2; // remaining will be filled with AI

pub struct WorldServer {
    grid: Grid,
    players: Vec<ServerPlayer>,
    score_win: u8,

    pub world_state: WorldState,
    last_update_time: f64,
    next_update: GridUpdateMsg,
    last_update: GridUpdateMsg,
}

impl WorldServer {
    pub fn new() -> Self {
        Self {
            grid: Grid::new(),
            players: Vec::new(),
            score_win: SCORE_WIN,

            world_state: WorldState::Waiting,
            last_update_time: 0.,
            next_update: GridUpdateMsg::default(),
            last_update: GridUpdateMsg::default(),
        }
    }

    pub fn join(&mut self, player: &ClientPlayer) -> u8 {
        self.players.push(ServerPlayer {
            score: 0,
            name: player.name.clone(),
            ready: player.ready,
            is_ai: false,
        });
        (self.players.len() - 1) as u8
    }

    pub fn push_update(&mut self, updates: &[BikeUpdate]) {
        if self.world_state == WorldState::Playing {
            self.next_update.updates.extend_from_slice(updates);
        }
    }

    pub fn update_player(&mut self, player_id: u8, client_player: &ClientPlayer) -> bool {
        let Some(server_player) = self.players.get_mut(player_id as usize) else {
            log::warn!("Player {} doesn't exist!", player_id);
            return false;
        };

        let mut changes_made = false;

        if server_player.name != client_player.name {
            server_player.name = client_player.name.clone();
            changes_made = true;
        }
        if server_player.ready != client_player.ready {
            server_player.ready = client_player.ready;
            log::info!("Player {} ready", player_id);
            changes_made = true;
        } 
        changes_made
    }

    pub fn get_last_update(&self) -> &GridUpdateMsg {
        &self.last_update
    }

    pub fn update(&mut self, time: f64) {
        match self.world_state {
            WorldState::Waiting | WorldState::RoundOver(_) | WorldState::GameOver(_) => {
                if self.players.is_empty()
                    || self
                        .players
                        .iter()
                        .any(|player| player.ready == false && player.is_ai == false)
                {
                    return;
                }
                if matches!(self.world_state, WorldState::GameOver(_)) {
                    for player in self.players.iter_mut() {
                        player.score = 0;
                    }
                    self.world_state = WorldState::Waiting;
                } else {
                    self.world_state = WorldState::Playing;
                    // TEMP: create AI players
                    if self.players.len() < MIN_PLAYERS {
                        self.players.push(ServerPlayer {
                            score: 0,
                            name: "AI".into(),
                            ready: false,
                            is_ai: true,
                        });
                    }
                }
                self.grid = Grid::new();
                self.grid.rng.srand(time as u64);
                self.last_update_time = time;
                self.next_update = GridUpdateMsg::default();
                self.last_update = GridUpdateMsg::default();
            }
            WorldState::Playing => {
                if time - self.last_update_time > (1.0 / 60.0) {
                    self.last_update_time = time;
                    self.last_update = self.next_update.clone();
                    self.next_update.updates.clear();
                    self.next_update.tick += 1;

                    for i in 0..self.players.len() {
                        if self.players[i].is_ai {
                            if let Some(update) =
                                self.grid.bikes[i].ai_update(&self.grid.occupied, &self.grid.rng)
                            {
                                self.last_update.updates.push(update);
                            }
                        }
                    }

                    match self.grid.apply_updates(&self.last_update) {
                        UpdateResult::GameOver(winner) => {
                            self.players[winner as usize].score += 1;
                            if self.players[winner as usize].score == self.score_win {
                                self.world_state = WorldState::GameOver(winner);
                                // self.game_won = winner == self.player_id.unwrap();
                            } else {
                                self.world_state = WorldState::RoundOver(winner);
                            }
                            for player in self.players.iter_mut() {
                                player.ready = false;
                            }
                        }
                        UpdateResult::InProgress => {}
                    }
                    self.last_update.hash = self.grid.hash;
                }
            }
        }
    }
}
