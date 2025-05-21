use std::vec;

use crate::{
    grid::{
        bike::BikeUpdate, bike_id, team_from_bike, Grid, GridUpdateMsg, UpdateResult
    }, ClientPlayer, GridOptions, ServerPlayer
};

use super::WorldState;

pub mod connection;

// const PLAYER_MAX: usize = 4;
// TODO: Server settings of some kind?
const SCORE_WIN: u8 = 3;

pub struct WorldServer {
    options: GridOptions,
    grid: Grid,
    players: Vec<ServerPlayer>,
    score_win: u8,
    scores: Vec<u8>,

    pub world_state: WorldState,
    last_update_time: f64,
    next_update: GridUpdateMsg,
    last_update: GridUpdateMsg,
}

impl WorldServer {
    pub fn new(options: GridOptions) -> Self {
        Self {
            options,
            grid: Grid::new(options),
            players: vec![
                ServerPlayer {
                    // score: 0,
                    name: "AI".into(),
                    ready: false,
                    is_ai: true,
                    // team: ,
                };
                (options.teams * options.players) as usize
            ],
            score_win: SCORE_WIN,

            world_state: WorldState::Waiting,
            last_update_time: 0.,
            next_update: GridUpdateMsg::default(),
            last_update: GridUpdateMsg::default(),
            scores: vec![0; options.teams as usize],
        }
    }

    pub fn join(&mut self, client_player: &ClientPlayer) -> u8 {
        for (i, player) in self.players.iter_mut().enumerate() {
            if player.is_ai {
                player.is_ai = false;
                player.ready = client_player.ready;
                player.name = client_player.name.clone();
                return i as u8;
            }
        }
        panic!("No room to join!");
    }

    pub fn push_update(&mut self, updates: &[BikeUpdate]) {
        if self.world_state == WorldState::Playing {
            self.next_update.updates.extend_from_slice(updates);
        }
    }

    pub fn update_player(
        &mut self,
        server_player_id: &mut u8,
        client_player: &ClientPlayer,
    ) -> bool {
        let Some(server_player) = self.players.get_mut(*server_player_id as usize) else {
            log::warn!("Player {} doesn't exist!", server_player_id);
            return false;
        };

        let mut changes_made = false;

        if server_player.name != client_player.name {
            server_player.name = client_player.name.clone();
            changes_made = true;
        }
        if server_player.ready != client_player.ready {
            server_player.ready = client_player.ready;
            log::info!("Player {} ready", server_player_id);
            changes_made = true;
        }
        if !server_player.ready
            && self.world_state == WorldState::Waiting
            && client_player.team_request != team_from_bike(&self.options, *server_player_id)
        {
            // is there space on the new team
            for team_player_index in 0..self.options.players {
                let new_id = bike_id(&self.options, client_player.team_request, team_player_index);
                let player = &mut self.players[new_id as usize];
                if player.is_ai {
                    log::info!(
                        "Player {} changing to team {} player {}",
                        client_player.name,
                        client_player.team_request,
                        new_id
                    );
                    // yay we can move here
                    player.is_ai = false;
                    player.name = client_player.name.clone();
                    player.ready = client_player.ready;

                    // remove the old player
                    let old_player = &mut self.players[*server_player_id as usize];
                    old_player.is_ai = true;
                    old_player.name = "AI".into();

                    // update our index
                    *server_player_id = new_id;

                    changes_made = true;
                    break;
                }
            }
        }
        changes_made
    }

    pub fn get_last_update(&self) -> &GridUpdateMsg {
        &self.last_update
    }

    pub fn update(&mut self, time: f64) {
        match self.world_state {
            WorldState::Waiting | WorldState::RoundOver(_) | WorldState::GameOver(_) => {
                if self
                    .players
                    .iter()
                    .any(|player| player.ready == false && player.is_ai == false)
                    || !self.players.iter().any(|player| player.is_ai == false)
                {
                    return;
                }

                if matches!(self.world_state, WorldState::GameOver(_)) {
                    for player in self.players.iter_mut() {
                        // player.score = 0;
                        player.ready = false;
                    }
                    for score in self.scores.iter_mut() {
                        *score = 0;
                    }
                    self.world_state = WorldState::Waiting;
                } else {
                    self.world_state = WorldState::Playing;
                }
                self.grid = Grid::new(self.options);
                self.grid.rng.srand(time as u64);
                self.last_update_time = time;
                self.next_update = GridUpdateMsg::default();
                self.last_update = GridUpdateMsg::default();
            }
            WorldState::Playing => {
                if time - self.last_update_time > (1.0 / 60.0) {
                    self.last_update_time = time;
                    self.last_update = self.next_update.clone();
                    self.last_update.tick += 1;
                    self.next_update.updates.clear();
                    self.next_update.tick = self.last_update.tick + 1;

                    for i in 0..self.players.len() {
                        if self.players[i].is_ai {
                            if let Some(update) = self.grid.bikes[i].ai_update(&self.grid) {
                                self.last_update.updates.push(update);
                            }
                        }
                    }

                    match self.grid.apply_updates(&self.last_update, None) {
                        UpdateResult::MatchOver(winner) => {
                            for player in self.players.iter_mut() {
                                player.ready = false;
                            }
                            if let Some(winning_team) = winner {
                                self.scores[winning_team as usize] += 1;
                                if self.scores[winning_team as usize] == self.score_win {
                                    self.world_state = WorldState::GameOver(winning_team);
                                } else {
                                    self.world_state = WorldState::RoundOver(Some(winning_team));
                                }
                            } else {
                                self.world_state = WorldState::RoundOver(None);
                            }
                        }
                        UpdateResult::InProgress => {}
                    }
                    log::debug!("Tick: {:?}", &self.last_update);
                    self.last_update.hash = self.grid.hash;
                }
            }
        }
    }
}
