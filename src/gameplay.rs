use macroquad::color::colors;
use macroquad::prelude::*;
use quad_net::quad_socket::client::QuadSocket;
use tron_io::grid::msg::{BikeUpdate, ClientMsg, GridUpdateMsg, ServerMsg, WorldState};

use crate::context::Context;
use crate::scene::{GameOptions, Scene};
use crate::text::draw_text_centered;
use crate::{input, text};
use tron_io::grid::bike::{DOWN, LEFT, RIGHT, UP};
use tron_io::grid::{Grid, PLAYER_COLOR_LOOKUP, Point};

const PLAYER_MAX: usize = 4;

pub struct Gameplay {
    grid: Grid,
    scores: [u8; PLAYER_MAX],
    score_win: u8,

    speed: f64,
    last_update: f64,
    player_id: Option<u8>,
    ready: bool,
    player_update: Option<BikeUpdate>,
    game_state: WorldState,
    pub game_won: bool,
    pub socket: Option<QuadSocket>,
}

impl Gameplay {
    pub fn new(_context: &Context, gameoptions: GameOptions) -> Self {
        Self {
            grid: Grid::new(),
            scores: [0; PLAYER_MAX],
            score_win: 3,
            speed: 0.05,
            player_id: None,
            ready: false,
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
            // wait for players
            if input::action_pressed(input::Action::Confirm, &context.gamepads) {
                if !self.ready {
                    self.ready = true;
                    println!("Ready!");
                    socket.send_bin(&ClientMsg {
                        ready: true,
                        state: self.game_state,
                        update: None,
                    });
                }
            }
            // MULTILAYER
            while let Some(server_msg) = socket.try_recv_bin::<ServerMsg>() {
                // self.grid.apply_updates(&grid_update);
                // dbg!(&server_msg);
                self.player_id = Some(server_msg.id);
                if self.game_state != server_msg.state {
                    self.game_state = server_msg.state;
                    self.ready = false;
                    match self.game_state {
                        WorldState::Waiting => {
                            // New game
                            self.grid = Grid::new();
                            self.scores = [0; PLAYER_MAX];
                        }
                        WorldState::Playing => {
                            // New round (potentially)
                            self.grid = Grid::new();
                        }
                        WorldState::RoundOver(winner) => self.scores[winner as usize] += 1,
                        WorldState::GameOver(winner) => self.scores[winner as usize] += 1,
                    }
                    println!("Game state changed to {:?}", self.game_state);
                }
                if self.game_state == WorldState::Playing {
                    if let Some(grid_update) = server_msg.grid_update {
                        self.player_update = None;
                        let _ = self.grid.apply_updates(&grid_update);

                        if self.grid.hash != grid_update.hash {
                            println!("Hash mismatch! {} != {}", self.grid.hash, grid_update.hash);
                            println!("Tick mismatch! {} != {}", self.grid.tick, grid_update.tick);
                            // context.switch_scene_to = Some(crate::scene::EScene::MainMenu);
                        }

                        socket.send_bin(&ClientMsg {
                            ready: false,
                            state: tron_io::grid::msg::WorldState::Playing,
                            update: Some(GridUpdateMsg {
                                tick: self.grid.tick,
                                hash: self.grid.hash,
                                updates: vec![],
                            }),
                        });
                    }
                }
            }
        } else {
            // SINGLEPLAYER
            match self.game_state {
                WorldState::Waiting | WorldState::RoundOver(_) | WorldState::GameOver(_) => {
                    // wait for players
                    if input::action_pressed(input::Action::Confirm, &context.gamepads) {
                        if matches!(self.game_state, WorldState::GameOver(_)) {
                            self.scores = [0; PLAYER_MAX];
                        }
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
                            hash: 0,
                            updates: if let Some(player_update) = self.player_update.take() {
                                vec![player_update]
                            } else {
                                vec![]
                            },
                        }) {
                            tron_io::grid::UpdateResult::GameOver(winner) => {
                                self.scores[winner as usize] += 1;
                                if self.scores[winner as usize] == self.score_win {
                                    self.game_state = WorldState::GameOver(winner);
                                    self.game_won = winner == self.player_id.unwrap();
                                } else {
                                    self.game_state = WorldState::RoundOver(winner);
                                }
                            }
                            tron_io::grid::UpdateResult::InProgress => {}
                        }
                        self.player_update = None;
                    }
                }
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
                                    ready: self.ready,
                                    update: Some(GridUpdateMsg {
                                        tick: self.grid.tick,
                                        hash: self.grid.hash,
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
        text::draw_text(context, "WN:", 10., 30., text::Size::Medium, colors::WHITE);

        const BOX_POS_ADJUSTMENT: f32 = text::text_size(text::Size::Medium) as f32 / 2.;

        for j in 0..self.score_win {
            draw_rectangle(
                60. + j as f32 * 20.,
                30. - BOX_POS_ADJUSTMENT,
                15.,
                15.,
                colors::WHITE,
            );
        }

        for i in 0..2 {
            let pos = vec2(10., 60. + 30. * i as f32);
            text::draw_text(
                context,
                format!("P{}:", i + 1).as_str(),
                pos.x,
                pos.y,
                text::Size::Medium,
                PLAYER_COLOR_LOOKUP[i],
            );
            for j in 0..self.scores[i] {
                draw_rectangle(
                    pos.x + 50. + j as f32 * 20.,
                    pos.y - BOX_POS_ADJUSTMENT,
                    15.,
                    15.,
                    PLAYER_COLOR_LOOKUP[i],
                );
            }
            // text::draw_text(
            //     context,
            //     format!("{}", self.scores[i]).as_str(),
            //     pos.x + 50.,
            //     pos.y,
            //     text::Size::Medium,
            //     colors::WHITE,
            // );
        }

        if matches!(
            self.game_state,
            WorldState::GameOver(_) | WorldState::Waiting | WorldState::RoundOver(_)
        ) {
            draw_rectangle(0., 0., screen_width(), screen_height(), Color {
                r: 0.,
                g: 0.,
                b: 0.,
                a: 0.5,
            });
            let (text, subtext) = match self.game_state {
                WorldState::GameOver(winner) => {
                    if winner == self.player_id.unwrap() {
                        ("Game Won!", "Press [enter] to play again.")
                    } else {
                        ("Game Lost.", "Press [enter] to play again.")
                    }
                }
                WorldState::Waiting => ("Waiting for players...", "Press [enter] to start."),
                WorldState::RoundOver(winner) => (
                    if winner == self.player_id.unwrap() {
                        "Round Won!"
                    } else {
                        "Round Lost."
                    },
                    "Press [enter] when ready.",
                ),
                _ => unreachable!(),
            };

            draw_text_centered(
                &context,
                text,
                context.screen_size.y / 2.,
                text::Size::Medium,
                colors::WHITE,
            );
            draw_text_centered(
                &context,
                subtext,
                context.screen_size.y / 2. + 100.,
                text::Size::Small,
                colors::WHITE,
            );
        }
    }
}
