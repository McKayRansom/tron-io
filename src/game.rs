use std::collections::LinkedList;

use macroquad::prelude::*;

use crate::snake::{DOWN, Grid, LEFT, RIGHT, SQUARES, Snake, UP};

pub struct Game {
    grid: Grid,

    player: Snake,
    enemy: Snake,

    speed: f64,
    last_update: f64,
    navigation_lock: bool,
    game_over: bool,
    game_won: bool,
}

impl Game {
    pub fn new() -> Self {
        Self {
            grid: Grid::new(),
            player: Snake {
                head: (10, SQUARES / 2),
                dir: (1, 0),
                body: LinkedList::new(),
                head_color: SKYBLUE,
                body_color: DARKBLUE,
            },

            enemy: Snake {
                head: (SQUARES - 11, SQUARES / 2),
                dir: (-1, 0),
                body: LinkedList::new(),
                head_color: PINK,
                body_color: MAROON,
            },

            // let mut fruit: Point = (rand::gen_range(0, SQUARES), rand::gen_range(0, SQUARES));
            // let mut score = 0;
            speed: 0.05,
            last_update: get_time(),
            navigation_lock: false,
            game_over: false,
            game_won: true,
        }
    }

    pub fn update(&mut self) -> bool {
        if !self.game_over {
            if (is_key_down(KeyCode::Right) || is_key_down(KeyCode::D))
                && self.player.dir != LEFT
                && !self.navigation_lock
            {
                self.player.dir = RIGHT;
                self.navigation_lock = true;
            } else if (is_key_down(KeyCode::Left) || is_key_down(KeyCode::A))
                && self.player.dir != RIGHT
                && !self.navigation_lock
            {
                self.player.dir = LEFT;
                self.navigation_lock = true;
            } else if (is_key_down(KeyCode::Up) || is_key_down(KeyCode::W))
                && self.player.dir != DOWN
                && !self.navigation_lock
            {
                self.player.dir = UP;
                self.navigation_lock = true;
            } else if (is_key_down(KeyCode::Down) || is_key_down(KeyCode::S))
                && self.player.dir != UP
                && !self.navigation_lock
            {
                self.player.dir = DOWN;
                self.navigation_lock = true;
            }

            if get_time() - self.last_update > self.speed {
                self.last_update = get_time();

                if self.enemy.update(&mut self.grid, true) {
                    self.game_won = true;
                    self.game_over = true;
                }

                if self.player.update(&mut self.grid, false) {
                    self.game_over = true;
                    self.game_won = false;
                }

                self.navigation_lock = false;
            }
        }

        clear_background(self.player.body_color);

        self.grid.update_size();
        self.grid.draw();

        self.player.draw(&self.grid);
        self.enemy.draw(&self.grid);

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
