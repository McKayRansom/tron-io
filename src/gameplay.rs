use macroquad::color::colors;
use macroquad::prelude::*;
use quad_net::quad_socket::client::QuadSocket;
use tron_io::grid::msg::{BikeUpdate, ClientMsg, GridUpdateMsg, ServerMsg, WorldState};

use crate::context::Context;
use crate::scene::{GameOptions, Scene};
use crate::{input, text};
use tron_io::grid::bike::{DOWN, LEFT, RIGHT, UP};
use tron_io::grid::{Grid, Point};

pub struct Gameplay {
    grid: Grid,

    speed: f64,
    last_update: f64,
    player_id: Option<u8>,
    player_update: Option<BikeUpdate>,
    game_state: WorldState,
    pub game_won: bool,
    pub socket: Option<QuadSocket>,
}

impl Gameplay {
    pub fn new(_context: &Context, gameoptions: GameOptions) -> Self {
        Self {
            grid: Grid::new(),
            speed: 0.05,
            player_id: None,
            last_update: get_time(),
            player_update: None,
            game_state: WorldState::Waiting,
            game_won: true,
            socket: gameoptions.socket,
        }
    }

    fn update_player_input(&self, player_id: u8) -> Option<Point> {
        // input::action_pressed(action, gamepads)
        if (is_key_down(KeyCode::Right) || is_key_down(KeyCode::D))
            && self.grid.bikes[player_id as usize].dir != LEFT
        {
            Some(RIGHT)
        } else if (is_key_down(KeyCode::Left) || is_key_down(KeyCode::A))
            && self.grid.bikes[player_id as usize].dir != RIGHT
        {
            Some(LEFT)
        } else if (is_key_down(KeyCode::Up) || is_key_down(KeyCode::W))
            && self.grid.bikes[player_id as usize].dir != DOWN
        {
            Some(UP)
        } else if (is_key_down(KeyCode::Down) || is_key_down(KeyCode::S))
            && self.grid.bikes[player_id as usize].dir != UP
        {
            Some(DOWN)
        } else {
            None
        }
    }
}

impl Scene for Gameplay {
    fn update(&mut self, context: &mut Context) {
        if let Some(socket) = &mut self.socket {
            // MULTILAYER
            while let Some(server_msg) = socket.try_recv_bin::<ServerMsg>() {
                // self.grid.apply_updates(&grid_update);
                dbg!(&server_msg);
                self.player_id = Some(server_msg.id);
                self.game_state = server_msg.state;
                if let Some(grid_update) = server_msg.grid_update {
                    self.player_update = None;
                    let _ = self.grid.apply_updates(&grid_update);

                    socket.send_bin(&ClientMsg {
                        state: tron_io::grid::msg::WorldState::Playing,
                        update: Some(GridUpdateMsg {
                            tick: self.grid.tick,
                            seed: 0,
                            updates: vec![],
                        }),
                    });
                }
            }
        } else {
            // SINGLEPLAYER
            match self.game_state {
                WorldState::Waiting | WorldState::RoundOver | WorldState::GameOver => {
                    // wait for players
                    if input::action_pressed(input::Action::Confirm, &context.gamepads) {
                        self.game_state = WorldState::Playing;
                        self.player_id = Some(0);
                        self.grid = Grid::new();
                        self.last_update = get_time();
                    }
                }
                WorldState::Playing => {
                    if get_time() - self.last_update > self.speed {
                        self.last_update = get_time();
                        match self.grid.apply_updates(&GridUpdateMsg {
                            tick: self.grid.tick + 1,
                            seed: 0,
                            updates: if let Some(player_update) = self.player_update.take() {
                                vec![player_update]
                            } else {
                                vec![]
                            },
                        }) {
                            tron_io::grid::UpdateResult::GameOver(winner) => {
                                dbg!(&winner);
                                if winner == 0 {
                                    self.game_state = WorldState::GameOver;
                                    self.game_won = false;
                                } else {
                                    self.game_state = WorldState::RoundOver;
                                    self.game_won = true;
                                }
                            }
                            tron_io::grid::UpdateResult::InProgress => {}
                        }
                        self.player_update = None;
                    }
                }
                _ => {}
            }
        }

        // }
        if matches!(self.game_state, WorldState::Playing) {
            if self.player_update.is_none() {
                if let Some(player_id) = self.player_id {
                    if let Some(dir) = self.update_player_input(player_id) {
                        let update = BikeUpdate::new(player_id, dir);
                        self.player_update = Some(update.clone());
                        // self.grid.bikes[0].dir = dir;
                        // self.grid.bikes[0].update(&mut self.grid.occupied, false);

                        // if let Some(update) = self.player_update.take() {
                        if self.grid.bikes[player_id as usize].dir != update.dir {
                            if let Some(socket) = &mut self.socket {
                                // send update to server
                                socket.send_bin(&ClientMsg {
                                    update: Some(GridUpdateMsg {
                                        tick: self.grid.tick,
                                        seed: 0,
                                        updates: vec![update],
                                    }),
                                    state: tron_io::grid::msg::WorldState::Playing,
                                });
                            } else {
                                self.grid.apply_update(&update);
                            }
                        }
                    }
                }
            }
        }
    }

    fn draw(&mut self, context: &mut Context) {
        clear_background(BLACK);

        self.grid.draw();

        text::draw_text(
            context,
            format!("Score: Won: X Lost: X").as_str(),
            10.,
            20.,
            text::Size::Medium,
            colors::WHITE,
        );

        if matches!(self.game_state, WorldState::GameOver | WorldState::Waiting | WorldState::RoundOver) {
            draw_rectangle(0., 0., screen_width(), screen_height(), Color {
                r: 0.,
                g: 0.,
                b: 0.,
                a: 0.5,
            });
            let text = match self.game_state {
                WorldState::GameOver => {
                    if self.game_won {
                        "Game Won! Press [enter] to play again."
                    } else {
                        "Game Over. Press [enter] to play again."
                    }
                }
                WorldState::Waiting => "Waiting for players...",
                WorldState::RoundOver => "Round Over, waiting for next round.",
                _ => "Unknown state",
            };
            let font_size = 30.;
            let text_size = measure_text(text, None, font_size as _, 1.0);

            draw_text_ex(
                text,
                screen_width() / 2. - text_size.width / 2.,
                screen_height() / 2. + text_size.height / 2.,
                TextParams {
                    font: Some(&context.font),
                    font_size: font_size as u16,
                    color: colors::WHITE,
                    ..Default::default()
                },
            );
        }
    }
}
