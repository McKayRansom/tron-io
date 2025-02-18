use std::collections::LinkedList;

use macroquad::prelude::*;

use crate::snake::{DOWN, Grid, LEFT, RIGHT, SQUARES, Snake, UP};

pub struct Game {
    grid: Grid,

    snakes: Vec<Snake>,

    speed: f64,
    last_update: f64,
    navigation_lock: bool,
    game_over: bool,
    pub game_won: bool,
}

impl Game {
    pub fn new() -> Self {
        Self {
            grid: Grid::new(),
            snakes: vec![
                Snake {
                    head: (8, SQUARES / 2),
                    dir: (1, 0),
                    body: LinkedList::new(),
                    head_color: SKYBLUE,
                    body_color: DARKBLUE,
                },
                Snake {
                    head: (SQUARES - 9, SQUARES / 2),
                    dir: (-1, 0),
                    body: LinkedList::new(),
                    head_color: PINK,
                    body_color: MAROON,
                },
                // Snake {
                //     head: (SQUARES / 2, 11),
                //     dir: (0, 1),
                //     body: LinkedList::new(),
                //     head_color: LIME,
                //     body_color: DARKGREEN,
                // },
                // Snake {
                //     head: (SQUARES / 2, SQUARES - 11),
                //     dir: (0, -1),
                //     body: LinkedList::new(),
                //     head_color: YELLOW,
                //     body_color: GOLD,
                // },
            ],

            // let mut fruit: Point = (rand::gen_range(0, SQUARES), rand::gen_range(0, SQUARES));
            // let mut score = 0;
            speed: 0.05,
            last_update: get_time(),
            navigation_lock: false,
            game_over: false,
            game_won: true,
        }
    }

    pub fn update(&mut self, won: u32, lost: u32) -> bool {
        if !self.game_over {
            if (is_key_down(KeyCode::Right) || is_key_down(KeyCode::D))
                && self.snakes[0].dir != LEFT
                && !self.navigation_lock
            {
                self.snakes[0].dir = RIGHT;
                self.navigation_lock = true;
            } else if (is_key_down(KeyCode::Left) || is_key_down(KeyCode::A))
                && self.snakes[0].dir != RIGHT
                && !self.navigation_lock
            {
                self.snakes[0].dir = LEFT;
                self.navigation_lock = true;
            } else if (is_key_down(KeyCode::Up) || is_key_down(KeyCode::W))
                && self.snakes[0].dir != DOWN
                && !self.navigation_lock
            {
                self.snakes[0].dir = UP;
                self.navigation_lock = true;
            } else if (is_key_down(KeyCode::Down) || is_key_down(KeyCode::S))
                && self.snakes[0].dir != UP
                && !self.navigation_lock
            {
                self.snakes[0].dir = DOWN;
                self.navigation_lock = true;
            }

            if get_time() - self.last_update > self.speed {
                self.last_update = get_time();

                let mut all_snakes_dead = true;
                for (i, snake) in self.snakes.iter_mut().enumerate() {
                    if snake.update(&mut self.grid, i != 0) {
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

        let mut player_color = self.snakes[0].body_color;
        player_color.r *= 0.5;
        player_color.g *= 0.5;
        player_color.b *= 0.5;

        clear_background(player_color);

        self.grid.update_size();
        self.grid.draw();

        for snake in &self.snakes {
            snake.draw(&self.grid);
        }

        draw_text(format!("Score: Won: {won} Lost: {lost}").as_str(), 10., 20., 20., WHITE);

        if self.game_over {
            // clear_background(BLACK);
            let text = if self.game_won {
                "Game Won! Press [enter] to play agin."
            } else {
                "Game Over. Press [enter] to play again."
            };
            let font_size = 30.;
            let text_size = measure_text(text, None, font_size as _, 1.0);

            draw_text(
                text,
                screen_width() / 2. - text_size.width / 2.,
                screen_height() / 2. + text_size.height / 2.,
                font_size,
                WHITE,
            );

            if is_key_down(KeyCode::Enter) {
                // start new game
                return true;
            }
        }
        false
    }
}
