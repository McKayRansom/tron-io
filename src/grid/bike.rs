use macroquad::color::{self};
use nanoserde::{DeBin, SerBin};

use crate::{grid::Point, world::Action};

use super::Occupied;

mod ai;

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

#[derive(DeBin, SerBin, Debug, Clone, Copy)]
pub struct BikeUpdate {
    pub id: u8,
    pub dir: Point,
    pub boost: bool,
}

impl BikeUpdate {
    pub fn new(id: u8, dir: Point) -> Self {
        Self {
            id,
            dir,
            boost: false,
        }
    }
}

type Ticks = u8;

pub const BOOST_COUNT: u8 = 4;
pub const BOOST_TIME: Ticks = 10;

// TODO: Are these off by one?
const BOOST_SPEED: Ticks = 0;
const NORMAL_SPEED: Ticks = 1;

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Bike {
    pub id: u8,
    color: u8,
    head: Point,
    dir: Point,
    alive: bool,
    speed: Ticks,
    pub boost_time: Ticks,
    pub boost_count: u8,
}

impl Bike {
    pub fn new(grid: &mut Occupied, id: u8, color: u8, head: Point, dir: Point) -> Self {
        assert!(!grid.occupy(head, color, false));
        Self {
            id,
            color,
            head,
            dir,
            alive: true,
            speed: 0,
            boost_time: 0,
            boost_count: BOOST_COUNT,
        }
    }

    pub fn handle_action(&self, action: Action) -> Option<BikeUpdate> {
        let new_dir = match action {
            Action::Left => crate::grid::bike::LEFT,
            Action::Right => crate::grid::bike::RIGHT,
            Action::Up => crate::grid::bike::UP,
            Action::Down => crate::grid::bike::DOWN,
            Action::Confirm => {
                dbg!("applying boost");
                if self.boost_count == 0 {
                    return None;
                }
                return Some(BikeUpdate {
                    id: self.id,
                    dir: self.dir,
                    boost: true,
                });
            }
            _ => return None,
        };

        if new_dir == self.dir || new_dir == crate::grid::bike::invert_dir(self.dir) {
            return None;
        }
        Some(BikeUpdate {
            id: self.id,
            dir: new_dir,
            boost: false,
        })
    }

    pub fn get_color(&self) -> color::Color {
        super::colors::get_color(self.color)
    }

    pub fn apply_update(&mut self, update: &BikeUpdate) {
        self.dir = update.dir;
        if update.boost && self.boost_count > 0 {
            self.boost_time = BOOST_TIME;
            self.boost_count -= 1;
        }
    }

    /// returns true if the bike is alive
    pub fn update(&mut self, grid: &mut Occupied) -> bool {
        if !self.alive {
            return self.alive;
        }
        if self.speed > 0 {
            self.speed -= 1;
            return true;
        } else {
            self.speed = if self.boost_time > 0 {
                self.boost_time -= 1;
                BOOST_SPEED
            } else {
                NORMAL_SPEED
            };
        }
        grid.free(self.head, self.color);

        let new_head = (self.head.0 + self.dir.0, self.head.1 + self.dir.1);

        if grid.occupy(new_head, self.color, self.boost_time > 0) {
            self.alive = false;
            false
        } else {
            self.head = new_head;
            true
        }
    }
}
