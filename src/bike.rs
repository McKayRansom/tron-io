
use std::collections::LinkedList;

use macroquad::{color::Color, rand};

use crate::grid::{Grid, Point};


pub const UP: Point = (0, -1);
pub const DOWN: Point = (0, 1);
pub const RIGHT: Point = (1, 0);
pub const LEFT: Point = (-1, 0);

pub const DIRS: &[Point] = &[UP, DOWN, LEFT, RIGHT];
pub const DIRS_REV: &[Point] = &[RIGHT, LEFT, DOWN, UP];

pub struct Bike {
    pub head: Point,
    pub body: LinkedList<Point>,
    pub dir: Point,
    pub head_color: Color,
    pub body_color: Color,
}

impl Bike {
    pub fn update(&mut self, grid: &mut Grid, is_ai: bool) -> bool {
        self.body.push_front(self.head);

        let new_head = (self.head.0 + self.dir.0, self.head.1 + self.dir.1);

        if grid.occupy(new_head) {
            if is_ai {
                let dirs = if rand::RandomRange::gen_range(0, 2) == 0 {
                    DIRS
                } else {
                    DIRS_REV
                };
                for dir in dirs {
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
