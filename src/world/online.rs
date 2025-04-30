use macroquad::prelude::info;
use quad_net::quad_socket::client::QuadSocket;

use crate::grid::msg::{ClientMsg, ServerMsg};

use super::ClientConnection;


pub struct WorldClientOnline {
    socket: QuadSocket,
}

impl WorldClientOnline {
    pub fn new(socket: QuadSocket) -> Self {
        Self { socket }
    }
}

impl ClientConnection for WorldClientOnline {

    fn send(&mut self, msg: &ClientMsg) {
        self.socket.send_bin(msg);
    }

    fn try_recv(&mut self) -> Option<ServerMsg> {
        {
            let this = &mut self.socket;
            let bytes = this.try_recv()?;
            // info!("Received bytes: {:?}", bytes);
            if let Ok(data) = nanoserde::DeBin::deserialize_bin(&bytes) {

                Some(data)
            } else {
                info!("Failed to deserialize bytes: {:?}", bytes);
                None
            }

        }
    }

    fn update(&mut self) {
        
    }
}