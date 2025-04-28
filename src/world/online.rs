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
        self.socket.try_recv_bin()
    }

    fn update(&mut self) {
        
    }
}