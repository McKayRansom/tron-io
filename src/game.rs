use macroquad::color::colors;
use macroquad::prelude::*;

use tron_io::bike::{Bike, DOWN, LEFT, RIGHT, UP};
use crate::context::Context;
use tron_io::grid::{Grid, SQUARES};

pub struct Game {
    grid: Grid,

    bikes: Vec<Bike>,

    speed: f64,
    last_update: f64,
    navigation_lock: bool,
    game_over: bool,
    pub game_won: bool,
}

impl Game {
    pub fn new() -> Self {
        let mut grid = Grid::new();
        Self {
            bikes: vec![
                Bike::new(&mut grid, 1, (8, SQUARES / 2), RIGHT),
                Bike::new(&mut grid, 2, (SQUARES - 9, SQUARES / 2), LEFT),
                Bike::new(&mut grid, 3, (SQUARES / 2, 11), DOWN),
                Bike::new(&mut grid, 4, (SQUARES / 2, SQUARES - 11), UP),
            ],
            grid,

            // let mut fruit: Point = (rand::gen_range(0, SQUARES), rand::gen_range(0, SQUARES));
            // let mut score = 0;
            speed: 0.05,
            last_update: get_time(),
            navigation_lock: false,
            game_over: false,
            game_won: true,
        }
    }

    pub fn update(&mut self, won: u32, lost: u32, context: &Context) -> bool {
        if !self.game_over {
            if (is_key_down(KeyCode::Right) || is_key_down(KeyCode::D))
                && self.bikes[0].dir != LEFT
                && !self.navigation_lock
            {
                self.bikes[0].dir = RIGHT;
                self.navigation_lock = true;
            } else if (is_key_down(KeyCode::Left) || is_key_down(KeyCode::A))
                && self.bikes[0].dir != RIGHT
                && !self.navigation_lock
            {
                self.bikes[0].dir = LEFT;
                self.navigation_lock = true;
            } else if (is_key_down(KeyCode::Up) || is_key_down(KeyCode::W))
                && self.bikes[0].dir != DOWN
                && !self.navigation_lock
            {
                self.bikes[0].dir = UP;
                self.navigation_lock = true;
            } else if (is_key_down(KeyCode::Down) || is_key_down(KeyCode::S))
                && self.bikes[0].dir != UP
                && !self.navigation_lock
            {
                self.bikes[0].dir = DOWN;
                self.navigation_lock = true;
            }

            if get_time() - self.last_update > self.speed {
                self.last_update = get_time();

                let mut all_snakes_dead = true;
                for (i, bike) in self.bikes.iter_mut().enumerate() {
                    if bike.update(&mut self.grid, i != 0) {
                        if i == 0 {
                            // player died
                            self.game_over = true;
                            self.game_won = false;
                        }
                    } else if i != 0 {
                        all_snakes_dead = false;
                    }
                }
                if all_snakes_dead {
                    self.game_won = true;
                    self.game_over = true;
                }
                self.navigation_lock = false;
            }
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

        if self.game_over {
            // clear_background(BLACK);
            let text = if self.game_won {
                "Game Won! Press [enter] to play agin."
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
