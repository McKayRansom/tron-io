use crate::{ClientMsg, ServerMsg, WorldState};

use super::WorldServer;

pub struct ServerConnectionState {
    // connection_id: Option<u8>,
    player_mappings: Vec<u8>,
    tick: u32,
    state: WorldState,
}

impl ServerConnectionState {
    pub fn new() -> Self {
        ServerConnectionState {
            // connection_id: None,
            player_mappings: Vec::new(),
            tick: 0,
            state: WorldState::Waiting,
        }
    }

    pub fn on_msg(&mut self, msg: &ClientMsg, world: &mut WorldServer) -> Option<ServerMsg> {
        let mut send_response = false;
        // this seems jank...
        let mut send_options = false;

        if self.player_mappings.len() < msg.players.len() {
            for i in self.player_mappings.len()..msg.players.len() {
                let id = world.join(&msg.players[i]);
                self.player_mappings.push(id);
                log::info!("Player {} joined", id);
                send_response = true;
                send_options = true;
            }
        }

        if let Some(update) = &msg.update {
            self.tick = update.tick;
            world.push_update(update.updates.as_slice());
        }
        self.state = msg.state;

        for (i, player) in msg.players.iter().enumerate() {
            send_response |= world.update_player(&mut self.player_mappings[i], player);
        }

        let last_update = world.get_last_update();
        if self.tick != last_update.tick && world.world_state == WorldState::Playing {
            log::debug!("Tick {} != {}", self.tick, last_update.tick);
            send_response = true;
        }

        if self.state != world.world_state {
            log::debug!("State {:?} != {:?}", self.state, world.world_state);
            send_response = true;
        }

        if send_response {
            Some(ServerMsg {
                local_player_ids: self.player_mappings.clone(),
                players: world.players.clone(),
                state: world.world_state,
                grid_update: Some(world.last_update.clone()),
                options: if send_options {
                    Some(world.options)
                } else {
                    None
                },
                score: world.scores.clone(),
            })
        } else {
            None
        }
    }

    pub fn update(&self, world: &WorldServer) -> Option<ServerMsg> {
        let mut response = ServerMsg {
            local_player_ids: self.player_mappings.clone(),
            players: world.players.clone(),
            state: world.world_state,
            grid_update: None,
            options: None,
            score: world.scores.clone(), // unnesscares clones problably
        };
        let last_update = world.get_last_update();
        if self.tick != last_update.tick && world.world_state == WorldState::Playing {
            log::debug!("Tick {} != {}", self.tick, last_update.tick);
            response.grid_update = Some(last_update.clone());
            Some(response)
        } else if self.state != world.world_state {
            log::debug!("State {:?} != {:?}", self.state, world.world_state);
            Some(response)
        } else {
            None
        }
    }
}
