use crate::grid::Point;

use super::Occupied;


pub const UP: Point = (0, -1);
pub const DOWN: Point = (0, 1);
pub const RIGHT: Point = (1, 0);
pub const LEFT: Point = (-1, 0);

pub const DIRS: &[Point] = &[UP, DOWN, LEFT, RIGHT];
pub const DIRS_REV: &[Point] = &[RIGHT, LEFT, DOWN, UP];

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Bike {
    pub id: u8,
    pub head: Point,
    pub dir: Point,
}

impl Bike {
    pub fn new(grid: &mut Occupied, id: u8, head: Point, dir: Point) -> Self {
        assert!(!grid.occupy(head, id));
        Self { id, head, dir }
    }

    pub fn update(&mut self, grid: &mut Occupied, is_ai: bool, rng: &macroquad::rand::RandGenerator) -> bool {
        
        grid.free(self.head, self.id);

        let new_head = (self.head.0 + self.dir.0, self.head.1 + self.dir.1);

        if grid.occupy(new_head, self.id) {
            if is_ai {
                let dirs = if rng.gen_range(0, 2) == 0 {
                    DIRS
                } else {
                    DIRS_REV
                };
                for dir in dirs {
                    let new_head = (self.head.0 + dir.0, self.head.1 + dir.1);
                    if !grid.occupy(new_head, self.id) {
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

}
