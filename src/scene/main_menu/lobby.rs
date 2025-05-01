use macroquad::color::colors;
use quad_net::quad_socket::client::QuadSocket;
use tron_io::world::{client::WorldClient, online::WorldClientOnline, ClientMsg, WorldState};

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
        return "ec2-3-144-4-46.us-east-2.compute.amazonaws.com:8090".to_string();
        #[cfg(target_arch = "wasm32")]
        return "ws://ec2-3-144-4-46.us-east-2.compute.amazonaws.com:8091".to_string();
    }

    pub fn update(&mut self, context: &mut Context) {
        if self.socket.is_none() && self.draw_finished && self.error == false {
            log::info!("Connecting to server {}", self.socket_addr());
            match QuadSocket::connect(self.socket_addr()) {
                Ok(socket) => {
                    self.socket = Some(socket);
                    self.draw_finished = false;

                }
                Err(err) => {
                    log::error!("Connecting to server: {:?}", err);
                    self.error = true
                }
            }
        }

        if self.socket.is_some() && self.draw_finished {
            if let Some(socket) = &mut self.socket {
                #[cfg(target_arch = "wasm32")]
                {
                    if socket.is_wasm_websocket_connected() == false {
                        return;
                    }
                }
                socket.send_bin(&ClientMsg {
                    state: WorldState::Waiting,
                    update: None,
                    ready: false,
                });
                context.switch_scene_to =
                Some(crate::scene::EScene::Gameplay(crate::scene::GameOptions {
                    client: WorldClient::new(Box::new(WorldClientOnline::new(
                        self.socket.take().unwrap(),
                    ))),
                }));
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
