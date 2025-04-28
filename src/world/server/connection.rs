use crate::grid::msg::{ClientMsg, ServerMsg, WorldState};

use super::WorldServer;



pub struct ServerConnectionState {
    id: Option<u8>,
    tick: u32,
    state: WorldState,
}

impl ServerConnectionState {
    pub fn new() -> Self {
        ServerConnectionState {
            id: None,
            tick: 0,
            state: WorldState::Waiting,
        }
    }

    pub fn on_msg(&mut self, msg: &ClientMsg, world: &mut WorldServer) -> Option<ServerMsg> {
        let mut response = ServerMsg {
            id: self.id.unwrap_or(0),
            state: world.world_state,
            grid_update: None,
        };
        let mut send_response = false;

        if self.id.is_none() {
            self.id = Some(world.join());
        }
        self.state = world.world_state;

        if let Some(update) = &msg.update {
            self.tick = update.tick;
            world.push_update(update.updates.as_slice());
        }
        self.state = msg.state;
        
        let last_update = world.get_last_update();
        if self.tick != last_update.tick {
            response.grid_update = Some(last_update.clone());
            send_response = true;
        }

        if self.state != world.world_state {
            send_response = true;
        }

        if world.set_ready(self.id.unwrap(), msg.ready) {
            println!("Player {} is ready: {}", self.id.unwrap(), msg.ready);
            if msg.ready && matches!(world.world_state, WorldState::GameOver(_)) {
                // if the player is ready and the game is over, find a new game
                // drop(world);
                // *self = ClientState::default();
                println!("Player {} is ready, finding new game", self.id.unwrap());
            }
        }
        if send_response {
            Some(response)
        } else {
            None
        }
    }

    pub fn update(&self, world: &WorldServer) -> Option<ServerMsg> {
        let mut response = ServerMsg {
            id: self.id.unwrap_or(0),
            state: world.world_state,
            grid_update: None,
        };
        let last_update = world.get_last_update();
        if self.tick != last_update.tick {
            response.grid_update = Some(last_update.clone());
            Some(response)
        } else {
            None
        }
    }
}
