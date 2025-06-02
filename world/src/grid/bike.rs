use std::collections::VecDeque;

use nanoserde::{DeBin, SerBin};

use crate::{
    Action,
    grid::{Point, team_to_color},
};

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

pub fn rotate_right(dir: Point) -> Point {
    match dir {
        UP => RIGHT,
        RIGHT => DOWN,
        DOWN => LEFT,
        LEFT => UP,
        _ => panic!("Invalid direction"),
    }
}

pub fn rotate_left(dir: Point) -> Point {
    match dir {
        UP => LEFT,
        RIGHT => UP,
        DOWN => RIGHT,
        LEFT => DOWN,
        _ => panic!("Invalid direction"),
    }
}

pub fn multiply(dir: Point, val: i16) -> Point {
    (dir.0 * val, dir.1 * val)
}

pub fn add(dir: Point, dir2: Point) -> Point {
    (dir.0 + dir2.0, dir.1 + dir2.1)
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

fn bike_pos(team: u8, player: u8, size: Point) -> (Point, Point) {
    let (pos, dir) = match team {
        0 => ((8, size.1 / 2), RIGHT),
        1 => ((size.0 - 9, size.1 / 2), LEFT),
        2 => ((size.0 / 2, 8), DOWN),
        3 => ((size.0 / 2, size.1 - 8), UP),
        _ => todo!(),
    };

    let player_offset = match player {
        0 => (0, 0),
        1 => add(multiply(rotate_right(dir), 3), invert_dir(dir)),
        2 => add(multiply(rotate_left(dir), 3), invert_dir(dir)),
        3 => add(multiply(rotate_right(dir), 6), multiply(invert_dir(dir), 3)),
        4 => add(multiply(rotate_left(dir), 6), multiply(invert_dir(dir), 3)),
        _ => unimplemented!(),
    };

    (add(pos, player_offset), dir)
}

type Ticks = u8;

pub const BOOST_COUNT: u8 = 3;
pub const BOOST_TIME: Ticks = 20;

// TODO: Are these off by one?
const BOOST_SPEED: Ticks = 0;
const NORMAL_SPEED: Ticks = 1;

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Bike {
    pub id: u8,
    pub team: u8,
    pub player: u8,
    color: u8,
    pub head: Point,
    pub dir: Point,
    pub alive: bool,
    speed: Ticks,
    pub boost_time: Ticks,
    pub boost_count: u8,

    pub length: u8,
    pub segments: VecDeque<Point>,
    // try shooting
}

impl Bike {
    pub fn new(grid: &mut Occupied, id: u8, team: u8, player: u8) -> Self {
        let (head, dir) = bike_pos(team, player, grid.size);
        let color = team_to_color(team, player);
        assert!(!grid.occupy(head, color, false));
        Self {
            id,
            team,
            player,
            color,
            head,
            dir,
            alive: true,
            speed: 0,
            boost_time: 0,
            boost_count: BOOST_COUNT,
            length: 64,
            segments: VecDeque::new(),
        }
    }

    pub fn get_color(&self) -> u8 {
        self.color
    }

    pub fn handle_action(&self, action: Action) -> Option<BikeUpdate> {
        let new_dir = match action {
            Action::Left => crate::grid::bike::LEFT,
            Action::Right => crate::grid::bike::RIGHT,
            Action::Up => crate::grid::bike::UP,
            Action::Down => crate::grid::bike::DOWN,
            Action::Confirm => {
                log::debug!("applying boost");
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

    pub fn apply_update(&mut self, update: &BikeUpdate) {
        self.dir = update.dir;
        // if update.boost && self.boost_count > 0 && self.boost_time == 0 {
        //     self.boost_time = BOOST_TIME;
        //     self.boost_count -= 1;
        // }
    }

    /// returns true if the bike just died
    pub fn update(&mut self, grid: &mut Occupied) -> bool {
        if !self.alive {
            return false;
        }
        if self.speed > 0 {
            self.speed -= 1;
            return false;
        } else {
            self.speed = if self.boost_time > 0 {
                self.boost_time -= 1;
                // BOOST_SPEED
                // hijack boosting to shoot!
                NORMAL_SPEED
            } else {
                NORMAL_SPEED
            };
        }

        // we were ramed :(
        if !grid.free(self.head, self.color) {
            self.alive = false;
            return true;
        }

        let new_head = (self.head.0 + self.dir.0, self.head.1 + self.dir.1);

        if grid.occupy(new_head, self.color, false) {
            self.alive = false;
            // explode the old head so it's clear we died
            grid.explose(self.head);
            true
        } else {
            // self.segments.push_front(self.head);
            // log::info!("Pushing segment {:?}", self.head);
            // while self.segments.len() > self.length as usize {
            //     if let Some(end) = self.segments.pop_back() {
            //         // log::info!("Poping segment {:?}", end);
            //         grid.free_for_read(end);
            //     }
            // }
            self.head = new_head;
            false
        }
    }
}
