use crate::{
    ClientPlayer, ServerPlayer,
    grid::{Grid, bike::BikeUpdate},
};

use super::{ClientConnection, ClientMsg, GridUpdateMsg, WorldState};

pub enum WorldEvent {
    PlayerJoin,
    PlayerReady,
    LocalUpdate(BikeUpdate),
    ServerUpdate(GridUpdateMsg),
    GameState(WorldState),
}

pub struct WorldClient {
    // local
    pub grid: Grid,
    pub score_win: u8,
    pub local_players: Vec<ClientPlayer>,
    pub events: Vec<WorldEvent>, // for sfx and such

    // remote
    pub local_player_ids: Vec<u8>, // translation ClientPlayer index -> ServerPlayer index
    pub server_players: Vec<ServerPlayer>,
    pub game_state: WorldState,
    connection: Box<dyn ClientConnection>,
}

impl WorldClient {
    pub fn new(connection: Box<dyn ClientConnection>) -> Self {
        Self {
            // local
            grid: Grid::new(),
            score_win: super::SCORE_WIN,
            local_players: Vec::new(),
            events: Vec::new(),

            // Remote
            local_player_ids: Vec::new(),
            server_players: Vec::new(),
            game_state: WorldState::Waiting,
            connection,
        }
    }

    fn send_msg(&mut self, update: Option<GridUpdateMsg>) {
        self.connection.send(&ClientMsg {
            // connection_id: self.connection_id,
            players: self.local_players.clone(),
            state: self.game_state,
            update,
        });
    }

    pub fn server_player(&self, local_player: u8) -> Option<u8> {
        self.local_player_ids.get(local_player as usize).copied()
    }

    pub fn local_player(&self, server_player: u8) -> Option<u8> {
        self.local_player_ids
            .iter()
            .position(|id| id == &server_player)
            .map(|index| index as u8)
    }

    pub fn handle_input(
        &mut self,
        local_player_id: Option<u8>,
        action: super::Action,
    ) -> Option<u8> {
        match self.game_state {
            WorldState::Waiting | WorldState::RoundOver(_) | WorldState::GameOver(_) => {
                if action == super::Action::Confirm {
                    // I was wrong, do not ready up on this
                    if local_player_id.is_none() {
                        if self.game_state == WorldState::Waiting {
                            let new_player_id = self.local_players.len() as u8;
                            self.local_players.push(ClientPlayer {
                                name: format!("p{}", self.local_players.len()),
                                ready: false,
                            });
                            // update server with players
                            self.send_msg(None);
                            return Some(new_player_id);
                        } else {
                            log::warn!("Player cannot join duing {:?}", self.game_state);
                            return None;
                        }
                    }

                    if let Some(player) = self
                        .local_players
                        .get_mut(local_player_id.unwrap() as usize)
                    {
                        player.ready = true;
                        log::debug!("Client player {} ready", local_player_id.unwrap());
                        self.send_msg(None);
                    } else {
                        log::warn!("Unknown player: {}", local_player_id.unwrap());
                    }
                }
            }
            WorldState::Playing => {
                let Some(player_id) = local_player_id else {
                    log::warn!("Player cannot join while game is in progress!");
                    return None;
                };
                let Some(server_player_id) = self.local_player_ids.get(player_id as usize) else {
                    log::warn!("Player {} has not yet joined server side!", player_id);
                    return None;
                };
                if let Some(bike_update) =
                    self.grid.bikes[*server_player_id as usize].handle_action(action)
                {
                    self.events.push(WorldEvent::LocalUpdate(bike_update));
                    // TODO: Mark this update for self.grid.tick + 2 and implement rollback!
                    // (and re-send to transition to UDP)
                    self.send_msg(Some(GridUpdateMsg {
                        tick: self.grid.tick,
                        hash: self.grid.hash,
                        updates: vec![bike_update],
                    }));
                }
            }
        }
        local_player_id
    }

    pub fn update(&mut self, time: f64) {
        while let Some(server_msg) = self.connection.try_recv() {
            log::debug!("Received server message: {:?}", server_msg);
            // self.connection_id = Some(server_msg.connection_id);
            self.server_players = server_msg.players.clone();
            self.local_player_ids = server_msg.local_player_ids.clone();
            if self.game_state != server_msg.state {
                self.game_state = server_msg.state;
                for player in self.local_players.iter_mut() {
                    player.ready = false;
                }
                match self.game_state {
                    WorldState::Waiting => {
                        // New game
                        self.grid = Grid::new();
                    }
                    WorldState::Playing => {
                        // New round (potentially)
                        self.grid = Grid::new();
                    }
                    _ => {}
                }
                self.events.push(WorldEvent::GameState(self.game_state));
                log::info!("Game state changed to {:?}", self.game_state);
                self.send_msg(None);
            }
            if self.game_state == WorldState::Playing {
                if let Some(grid_update) = server_msg.grid_update {
                    // This is expected to often not be the case as the server sends out ticks every 10ms
                    //  we may not get back before the server sends a duplicate
                    if grid_update.tick == self.grid.tick + 1 {
                        let _ = self.grid.apply_updates(&grid_update);

                        if self.grid.hash != grid_update.hash {
                            log::error!(
                                "Hash mismatch! {} != {} at tick {}",
                                self.grid.hash,
                                grid_update.hash,
                                grid_update.tick
                            );
                        }
                        self.events.push(WorldEvent::ServerUpdate(grid_update));

                        self.send_msg(Some(GridUpdateMsg {
                            tick: self.grid.tick,
                            hash: self.grid.hash,
                            updates: vec![],
                        }));
                    }
                }
            }
        }

        self.connection.update(time);
    }
}
