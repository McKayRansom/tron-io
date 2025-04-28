use std::collections::VecDeque;

use crate::grid::msg::{ClientMsg, ServerMsg};

use super::{server::{connection::ServerConnectionState, WorldServer}, ClientConnection};


pub struct WorldClientLocal {
    response: VecDeque<ServerMsg>,
    connection: ServerConnectionState,
    world: WorldServer,
}

impl WorldClientLocal {
    pub fn new() -> Self {
        Self {
            response: VecDeque::new(),
            connection: ServerConnectionState::new(),
            world: WorldServer::new(),
        }
    }
}

impl ClientConnection for WorldClientLocal {

    fn send(&mut self, msg: &ClientMsg) {
        self.connection.on_msg(msg, &mut self.world);
    }

    fn try_recv(&mut self) -> Option<ServerMsg> {
        self.response.pop_front()
    }

    fn update(&mut self) {
        if let Some(response) = self.connection.update(&self.world) {
            self.response.push_back(response);
        }
        self.world.update();
    }

}