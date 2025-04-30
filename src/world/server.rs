use std::time::{Duration, Instant, SystemTime};

use crate::grid::{
    Grid, UpdateResult,
    msg::{BikeUpdate, GridUpdateMsg, WorldState},
};

pub mod connection;

const PLAYER_MAX: usize = 4;
const SCORE_WIN: u8 = 3;

pub struct WorldServer {
    grid: Grid,
    scores: [u8; PLAYER_MAX],
    ready: [bool; PLAYER_MAX],
    score_win: u8,

    pub world_state: WorldState,
    players: u8,
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
            WorldState::Waiting | WorldState::RoundOver(_) | WorldState::GameOver(_) => {
                if self.players == 0 {
                    return;
                }
                for i in 0..self.players as usize {
                    if !self.ready[i] {
                        // dbg!("Waiting for player {}", i);
                        return;
                    }
                }
                if matches!(self.world_state, WorldState::GameOver(_)) {
                    self.scores = [0; PLAYER_MAX];
                    self.world_state = WorldState::Waiting;
                } else {
                    self.world_state = WorldState::Playing;
                }
                self.grid = Grid::new();
                self.grid.rng.srand(
                    SystemTime::now()
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .unwrap()
                        .as_millis() as u64,
                );
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

                    for i in self.players as usize..self.grid.bikes.len() {
                        if let Some(update) =
                            self.grid.bikes[i].ai_update(&self.grid.occupied, &self.grid.rng)
                        {
                            self.last_update.updates.push(update);
                        }
                    }

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
        }
    }
}
