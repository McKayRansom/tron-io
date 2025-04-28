use std::time::{Duration, Instant};

use crate::grid::{msg::{BikeUpdate, GridUpdateMsg, WorldState}, Grid, UpdateResult};

pub mod connection;

const PLAYER_MAX: usize = 4;
const SCORE_WIN: u8 = 3;

pub struct WorldServer {
    grid: Grid,
    scores: [u8; PLAYER_MAX],
    ready: [bool; PLAYER_MAX],
    score_win: u8,

    pub world_state: WorldState,
    // pos: (f32, f32),
    // last_edit_id: usize,
    players: u8, // doubles as player count I guess...
    last_update_time: Instant,
    next_update: GridUpdateMsg,
    last_update: GridUpdateMsg,
}

impl WorldServer {
    pub fn new() -> Self {
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

    pub fn join(&mut self) -> u8 {
        let id = self.players;
        self.players += 1;
        id
    }

    pub fn push_update(&mut self, updates: &[BikeUpdate]) {
        if self.world_state == WorldState::Playing {
            self.next_update.updates.extend_from_slice(updates);
        }
    }

    pub fn set_ready(&mut self, id: u8, ready: bool) -> bool {
        let old_ready = self.ready[id as usize];
        self.ready[id as usize] = ready;
        old_ready != ready
    }

    pub fn get_last_update(&self) -> &GridUpdateMsg {
        &self.last_update
    }

    pub fn update(&mut self) {
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
                        UpdateResult::GameOver(winner) => {
                            self.scores[winner as usize] += 1;
                            if self.scores[winner as usize] == self.score_win {
                                self.world_state = WorldState::GameOver(winner);
                                // self.game_won = winner == self.player_id.unwrap();
                            } else {
                                self.world_state = WorldState::RoundOver(winner);
                            }
                            self.ready = [false; PLAYER_MAX];
                        }
                        UpdateResult::InProgress => {}
                    }
                    self.last_update.hash = self.grid.hash;
                }
            }
            WorldState::GameOver(_) => {}
        }
    }
}
