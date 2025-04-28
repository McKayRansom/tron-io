use macroquad::color::colors;
use quad_net::quad_socket::client::QuadSocket;
use tron_io::grid::msg::ClientMsg;

use crate::{context::Context, text};

pub struct Lobby {
    socket: Option<QuadSocket>,
    draw_finished: bool,
    error: bool,
}

impl Lobby {
    pub fn new(_context: &Context) -> Self {
        Self {
            socket: None,
            draw_finished: false,
            error: false,
        }
    }

    fn socket_addr(&self) -> String {
        #[cfg(not(target_arch = "wasm32"))]
        return "localhost:8090".to_string();
        #[cfg(target_arch = "wasm32")]
        return "ws://localhost:8091".to_string();
    }

    pub fn update(&mut self, context: &mut Context) {
        if self.socket.is_none() && self.draw_finished && self.error == false {
            match QuadSocket::connect(self.socket_addr()) {
                Ok(mut socket) => {
                    socket.send_bin(&ClientMsg {
                        state: tron_io::grid::msg::WorldState::Waiting,
                        update: None,
                        ready: false,
                    });
                    self.socket = Some(socket);
                    self.draw_finished = false;
                    context.switch_scene_to =
                    Some(crate::scene::EScene::Gameplay(crate::scene::GameOptions {
                        socket: self.socket.take(),
                    }));
                }
                Err(err) => {
                    dbg!(err);
                    self.error = true
                }
            }

            #[cfg(target_arch = "wasm32")]
            {
                while self.socket.is_wasm_websocket_connected() == false {
                    next_frame().await;
                }
            }
        }
    }

    pub fn draw(&mut self, context: &mut Context) {
        // Drawing logic here
        if self.socket.is_none() && !self.error {
            text::draw_text(
                context,
                "Connecting to server...",
                super::X_INSET,
                super::MENU_CONTENT_Y,
                text::Size::Medium,
                colors::WHITE,
            );
        } else {
            if self.error {
                text::draw_text(
                    context,
                    "Error connecting to server",
                    super::X_INSET,
                    super::MENU_CONTENT_Y,
                    text::Size::Medium,
                    colors::RED,
                );
            } else {
                text::draw_text(
                    context,
                    "Connected to server, waiting for players...",
                    super::X_INSET,
                    super::MENU_CONTENT_Y,
                    text::Size::Medium,
                    colors::GREEN,
                );
            }
        }
        self.draw_finished = true;
    }
}
