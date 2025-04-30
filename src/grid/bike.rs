use crate::grid::Point;

use super::Occupied;

pub const UP: Point = (0, -1);
pub const DOWN: Point = (0, 1);
pub const RIGHT: Point = (1, 0);
pub const LEFT: Point = (-1, 0);

pub fn invert_dir(dir: Point) -> Point {
    match dir {
        UP => DOWN,
        DOWN => UP,
        LEFT => RIGHT,
        RIGHT => LEFT,
        _ => panic!("Invalid direction"),
    }
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Bike {
    pub id: u8,
    pub color: u8,
    pub head: Point,
    pub dir: Point,
}

impl Bike {
    pub fn new(grid: &mut Occupied, id: u8, color: u8, head: Point, dir: Point) -> Self {
        assert!(!grid.occupy(head, color));
        Self {
            id,
            color,
            head,
            dir,
        }
    }

    pub fn update(&mut self, grid: &mut Occupied) -> bool {
        grid.free(self.head, self.color);

        let new_head = (self.head.0 + self.dir.0, self.head.1 + self.dir.1);

        if grid.occupy(new_head, self.color) {
            return true;
        }

        self.head = new_head;
        false
    }
}
