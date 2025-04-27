use macroquad::color::colors;
use macroquad::prelude::*;
use quad_net::quad_socket::client::QuadSocket;
use tron_io::grid::msg::{BikeUpdate, GridUpdateMsg};

use crate::context::Context;
use tron_io::grid::bike::{DOWN, LEFT, RIGHT, UP};
use tron_io::grid::{Grid, Point};

#[derive(PartialEq, Eq)]
pub enum GameState {
    Lobby,
    Playing,
    GameOver,
}

pub struct Game {
    grid: Grid,

    speed: f64,
    last_update: f64,
    player_update: Option<BikeUpdate>,
    // game_over: bool,
    game_state: GameState,
    pub game_won: bool,
    pub socket: Option<QuadSocket>,
}

impl Game {
    pub fn new(socket: Option<QuadSocket>) -> Self {
        Self {
            grid: Grid::new(),

            // let mut fruit: Point = (rand::gen_range(0, SQUARES), rand::gen_range(0, SQUARES));
            // let mut score = 0;
            speed: 0.05,
            last_update: get_time(),
            player_update: None,
            game_state: GameState::Playing,
            game_won: true,
            socket,
        }
    }

    fn update_player_input(&self) -> Option<Point> {
        if (is_key_down(KeyCode::Right) || is_key_down(KeyCode::D))
            && self.grid.bikes[0].dir != LEFT
        {
            Some(RIGHT)
        } else if (is_key_down(KeyCode::Left) || is_key_down(KeyCode::A))
            && self.grid.bikes[0].dir != RIGHT
        {
            Some(LEFT)
        } else if (is_key_down(KeyCode::Up) || is_key_down(KeyCode::W))
            && self.grid.bikes[0].dir != DOWN
        {
            Some(UP)
        } else if (is_key_down(KeyCode::Down) || is_key_down(KeyCode::S))
            && self.grid.bikes[0].dir != UP
        {
            Some(DOWN)
        } else {
            None
        }
    }

    pub fn update(&mut self, won: u32, lost: u32, context: &Context) -> bool {
        if self.game_state == GameState::Playing {
            if self.player_update.is_none() {
                if let Some(dir) = self.update_player_input() {
                    self.player_update = Some(BikeUpdate::new(1, dir));
                    // self.grid.bikes[0].dir = dir;
                    // self.grid.bikes[0].update(&mut self.grid.occupied, false);
                }
            }

            // if get_time() - self.last_update > self.speed {
            // self.last_update = get_time();

            if let Some(update) = self.player_update.take() {
                if self.grid.bikes[0].dir != update.dir {
                    if let Some(socket) = &mut self.socket {
                        // send update to server
                        socket.send_bin(&GridUpdateMsg {
                            tick: self.grid.tick,
                            seed: 0,
                            updates: vec![update],
                        });
                    } else {
                        self.grid.apply_update(&update);
                    }
                }
            }

            if let Some(socket) = &mut self.socket {
                // send update to server
                // socket.send_bin(&self.player_update.unwrap());
                while let Some(grid_update) = socket.try_recv_bin() {
                    // self.grid.apply_updates(&grid_update);

                    match self.grid.apply_updates(&grid_update) {
                        // GameOver
                        tron_io::grid::UpdateResult::GameOver => {
                            self.game_state = GameState::GameOver;
                            self.game_won = false;
                        }
                        // GameWon
                        tron_io::grid::UpdateResult::GameWon => {
                            self.game_state = GameState::GameOver;
                            self.game_won = true;
                        }
                        // InProgress
                        tron_io::grid::UpdateResult::InProgress => {}
                    }
                    socket.send_bin(&GridUpdateMsg {
                        tick: self.grid.tick,
                        seed: 0,
                        updates: vec![],
                    });
                }
            }

            // }
        }

        // let mut player_color = self.bikes[0].body_color;
        // player_color.r *= 0.5;
        // player_color.g *= 0.5;
        // player_color.b *= 0.5;

        clear_background(BLACK);

        self.grid.draw();

        draw_text_ex(
            format!("Score: Won: {won} Lost: {lost}").as_str(),
            10.,
            20.,
            TextParams {
                font: Some(&context.font),
                font_size: 20,
                color: colors::GREEN,
                ..Default::default()
            },
        );

        if self.game_state == GameState::GameOver {
            draw_rectangle(0., 0., screen_width(), screen_height(), Color {
                r: 0.,
                g: 0.,
                b: 0.,
                a: 0.5,
            });
            let text = if self.game_won {
                "Game Won! Press [enter] to play again."
            } else {
                "Game Over. Press [enter] to play again."
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

            if is_key_down(KeyCode::Enter) {
                // start new game
                return true;
            }
        }
        false
    }
}
