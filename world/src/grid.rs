use std::hash::{DefaultHasher, Hash, Hasher};

use bike::{Bike, BikeUpdate};
use nanoserde::{DeBin, SerBin};

use crate::{AiDifficulty, GridOptions, GridSize, client::WorldEvent};

pub mod bike;

pub type Point = (i16, i16);

pub fn point_add(a: Point, b: Point) -> Point {
    (a.0 + b.0, a.1 + b.1)
}

#[derive(Clone)]
pub struct Cell {
    val: u8,
}

type ColorId = u8;
type TeamId = u8;
type PlayerId = u8;
type BikeId = u8;

pub fn color_to_team(color: ColorId) -> TeamId {
    color / 4
}

pub fn team_to_color(team: TeamId, player: PlayerId) -> ColorId {
    team * 4 + player
}

pub fn bike_id(options: &GridOptions, team: TeamId, player: PlayerId) -> BikeId {
    team * options.players + player
}

pub fn team_from_bike(options: &GridOptions, bike_id: BikeId) -> TeamId {
    bike_id / options.players
}

pub fn player_from_bike(options: &GridOptions, bike_id: BikeId) -> PlayerId {
    bike_id % options.players
}

impl Cell {
    const BIKE_MASK: u8 = 0b10000000;
    const BOOST_MASK: u8 = 0b1000000;
    const SPLODE_MASK: u8 = 0b100000;
    const COLOR_MASK: u8 = 0b11111;

    pub fn new() -> Self {
        Self { val: 0 }
    }

    pub fn occupy(&mut self, val: u8, is_bike: bool, is_boost: bool) {
        self.val = val + 1
            | if is_bike { Self::BIKE_MASK } else { 0 }
            | if is_boost { Self::BOOST_MASK } else { 0 };
    }

    pub fn is_occupied(&self) -> bool {
        self.val & Self::COLOR_MASK != 0
    }

    pub fn is_bike(&self) -> bool {
        self.val & Self::BIKE_MASK != 0
    }
    pub fn is_boost(&self) -> bool {
        self.val & Self::BOOST_MASK != 0
    }

    pub fn get_color(&self) -> ColorId {
        if self.val != 0 {
            (self.val & Self::COLOR_MASK) - 1
        } else {
            0
        }
    }

    pub fn explode(&mut self) {
        self.val |= Self::SPLODE_MASK;
        self.val &= !Self::BIKE_MASK;
    }

    pub fn is_exploded(&self) -> bool {
        self.val & Self::SPLODE_MASK != 0
    }

    // returns ok
    fn free(&mut self, _id: u8) -> bool {
        self.val &= !Self::BIKE_MASK;
        self.val & Self::SPLODE_MASK == 0
    }

    fn free_for_real(&mut self) {
        self.val = 0;
    }
}

pub struct Occupied {
    size: Point,
    occupied: Vec<Vec<Cell>>,
}

impl Occupied {
    pub fn new(size: GridSize) -> Self {
        let size = size.dim();
        Self {
            size,
            occupied: vec![vec![Cell::new(); size.1 as usize]; size.0 as usize],
        }
    }

    pub fn is_occupied(&self, pos: Point) -> bool {
        if pos.0 < 0 || pos.1 < 0 || pos.0 >= self.size.0 || pos.1 >= self.size.1 {
            return true;
        }
        self.occupied[pos.1 as usize][pos.0 as usize].is_occupied()
    }

    pub fn get_cell(&self, pos: Point) -> Option<&Cell> {
        self.occupied.get(pos.1 as usize)?.get(pos.0 as usize)
    }

    pub fn occupy(&mut self, pos: Point, id: u8, boost: bool) -> bool {
        if pos.0 < 0 || pos.1 < 0 || pos.0 >= self.size.0 || pos.1 >= self.size.1 {
            return true;
        }
        let cell = &mut self.occupied[pos.1 as usize][pos.0 as usize];

        if cell.is_occupied() {
            if cell.is_bike() {
                cell.explode();
            }
            return true;
        }
        cell.occupy(id, true, boost);
        return false;
    }

    pub(crate) fn free(&mut self, pos: (i16, i16), id: u8) -> bool {
        self.occupied[pos.1 as usize][pos.0 as usize].free(id)
    }
    pub fn explose(&mut self, pos: (i16, i16)) {
        if pos.0 < 0 || pos.1 < 0 || pos.0 >= self.size.0 || pos.1 >= self.size.1 {
            return;
        }
        self.occupied[pos.1 as usize][pos.0 as usize].explode();
    }

    fn free_for_read(&mut self, pos: (i16, i16)) {
        if pos.0 < 0 || pos.1 < 0 || pos.0 >= self.size.0 || pos.1 >= self.size.1 {
            return;
        }
        self.occupied[pos.1 as usize][pos.0 as usize].free_for_real();
    }
}

#[derive(DeBin, SerBin, Debug, Clone, Default)]
pub struct GridUpdateMsg {
    pub tick: u32,
    pub hash: u64,
    pub updates: Vec<BikeUpdate>,
}

const GAME_END_DELAY: u32 = 30;

pub struct Grid {
    pub tick: u32,
    pub hash: u64,
    pub bikes: Vec<Bike>,
    pub occupied: Occupied,
    pub rng: quad_rand::RandGenerator,
    pub delay: Option<u32>, // delay after game end
    pub ai_diff: AiDifficulty,
    pub bullets: Vec<Bullet>,
}

pub enum UpdateResult {
    MatchOver(Option<u8>),
    InProgress,
}

impl Grid {
    pub fn new(options: GridOptions) -> Self {
        let mut occupied = Occupied::new(options.grid_size);
        let mut bikes: Vec<Bike> = Vec::new();
        for team in 0..options.teams {
            for player in 0..options.players {
                bikes.push(Bike::new(&mut occupied, bikes.len() as u8, team, player));
            }
        }
        Self {
            hash: 0,
            bikes,
            occupied,
            tick: 0,
            rng: quad_rand::RandGenerator::new(),
            delay: None,
            ai_diff: options.ai_diff,
            bullets: Vec::new(),
        }
    }

    pub fn get_color(&self, bike_id: u8) -> u8 {
        self.bikes[bike_id as usize].get_color()
    }

    pub fn update(&mut self, mut events: Option<&mut Vec<WorldEvent>>) -> UpdateResult {
        //update bullets first so they can get out of the way of the shooting bike
        self.bullets
            .retain_mut(|bullet| bullet.update(&mut self.occupied));

        let mut winning_team: Option<u8> = None;
        let mut winner = true;
        let mut hasher = DefaultHasher::new();

        for bike in self.bikes.iter_mut() {
            if bike.update(&mut self.occupied) {
                if let Some(events) = &mut events {
                    events.push(WorldEvent::BikeDeath(bike.id, bike.head));
                }
            }

            if bike.alive {
                if winning_team.is_some_and(|team| team != bike.team) {
                    winner = false;
                } else {
                    winning_team = Some(bike.team);
                }
            }

            // Compute hash for the bike
            bike.hash(&mut hasher);
        }
        self.hash = hasher.finish();
        if winner {
            if self.delay.is_none() {
                self.delay = Some(GAME_END_DELAY)
            } else if self.delay.unwrap() > 0 {
                self.delay = Some(self.delay.unwrap() - 1)
            } else {
                return UpdateResult::MatchOver(winning_team);
            }
        }
        UpdateResult::InProgress
    }

    pub fn size(&self) -> Point {
        self.occupied.size
    }
}

impl Grid {
    pub fn apply_updates(
        &mut self,
        updates: &GridUpdateMsg,
        events: Option<&mut Vec<WorldEvent>>,
    ) -> UpdateResult {
        // tick and seed?
        for update in updates.updates.iter() {
            let bike = self.bikes.get_mut(update.id as usize).unwrap();
            bike.apply_update(update);
            if update.boost {
                if let Some(bullet) =
                    Bullet::new(&mut self.occupied, bike.head, bike.dir, bike.get_color())
                {
                    self.bullets.push(bullet);
                }
            }
        }
        if self.tick != updates.tick {
            self.tick = updates.tick;
            self.update(events)
        } else {
            UpdateResult::InProgress
        }
    }
}

pub struct Bullet {
    pos: Point,
    dir: Point,
    color: u8,
}

impl Bullet {
    pub fn new(occupied: &mut Occupied, from_pos: Point, dir: Point, color: u8) -> Option<Self> {
        let mut bullet = Bullet {
            pos: from_pos,
            dir,
            color,
        };
        bullet.pos = point_add(bullet.pos, bullet.dir);
        if occupied.occupy(bullet.pos, bullet.color, true) {
            // laser hit something?
            occupied.free_for_read(bullet.pos);
            return None;
        } else {
            return Some(bullet);
        }
    }

    pub fn update(&mut self, occupied: &mut Occupied) -> bool {
        // self.laser = self.head + self.dir + self.dir;
        occupied.free_for_read(self.pos);
        self.pos = point_add(self.pos, self.dir);
        if occupied.occupy(self.pos, self.color, true) {
            // laser hit something?
            occupied.free_for_read(self.pos);
            false
        } else {
            true
        }
    }
}
