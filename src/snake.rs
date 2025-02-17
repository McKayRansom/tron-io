use macroquad::prelude::*;

use std::collections::LinkedList;

pub type Point = (i16, i16);

pub const UP: Point = (0, -1);
pub const DOWN: Point = (0, 1);
pub const RIGHT: Point = (1, 0);
pub const LEFT: Point = (-1, 0);

pub const DIRS: &[Point] = &[UP, DOWN, LEFT, RIGHT];

pub const SQUARES: i16 = 80;

pub struct Grid {
    occupied: Vec<Vec<bool>>,

    game_size: f32,
    offset_x: f32,
    offset_y: f32,
    sq_size: f32,
}

impl Grid {
    pub fn new() -> Self {
        let mut grid: Grid = Self {
            occupied: vec![vec![false; SQUARES as usize]; SQUARES as usize],
            game_size: 0.,
            offset_x: 0.,
            offset_y: 0.,
            sq_size: 0.,
        };
        grid.update_size();
        grid
    }

    pub fn update_size(&mut self) {
        self.game_size = screen_width().min(screen_height());
        self.offset_x = (screen_width() - self.game_size) / 2. + 10.;
        self.offset_y = (screen_height() - self.game_size) / 2. + 10.;
        self.sq_size = (screen_height() - self.offset_y * 2.) / SQUARES as f32;
    }

    pub fn draw(&self) {
        draw_rectangle(
            self.offset_x,
            self.offset_y,
            self.game_size - 20.,
            self.game_size - 20.,
            BLACK,
        );

        for i in 0..SQUARES + 1 {
            if i % 4 != 0 {
                continue;
            }
            draw_line(
                self.offset_x,
                self.offset_y + self.sq_size * i as f32,
                screen_width() - self.offset_x,
                self.offset_y + self.sq_size * i as f32,
                2.,
                DARKGRAY,
            );
        }

        for i in 0..SQUARES + 1 {
            if i % 4 != 0 {
                continue;
            }
            draw_line(
                self.offset_x + self.sq_size * i as f32,
                self.offset_y,
                self.offset_x + self.sq_size * i as f32,
                screen_height() - self.offset_y,
                2.,
                DARKGRAY,
            );
        }
    }

    pub fn draw_cell(&self, pos: Point, color: Color) {
        draw_rectangle(
            self.offset_x + pos.0 as f32 * self.sq_size,
            self.offset_y + pos.1 as f32 * self.sq_size,
            self.sq_size,
            self.sq_size,
            color,
        );
    }

    pub fn occupy(&mut self, pos: Point) -> bool {
        if pos.0 < 0 || pos.1 < 0 || pos.0 >= SQUARES || pos.1 >= SQUARES {
            return true;
        }

        if self.occupied[pos.1 as usize][pos.0 as usize] {
            return true;
        }
        self.occupied[pos.1 as usize][pos.0 as usize] = true;
        return false;
    }
}

pub struct Snake {
    pub head: Point,
    pub body: LinkedList<Point>,
    pub dir: Point,
    pub head_color: Color,
    pub body_color: Color,
}

impl Snake {
    pub fn update(&mut self, grid: &mut Grid, is_ai: bool) -> bool {
        self.body.push_front(self.head);

        let new_head = (self.head.0 + self.dir.0, self.head.1 + self.dir.1);

        if grid.occupy(new_head) {

            if is_ai {
                for dir in DIRS {
                    let new_head = (self.head.0 + dir.0, self.head.1 + dir.1);
                    if !grid.occupy(new_head) {
                        self.head = new_head;
                        self.dir = *dir;
                        return false;
                    }
                }
                
            }
            return true; 
        }

        self.head = new_head;
        false
    }

    pub fn draw(&self, grid: &Grid) {
        grid.draw_cell(self.head, self.head_color);

        for pos in &self.body {
            grid.draw_cell(*pos, self.body_color);
        }
    }
}
