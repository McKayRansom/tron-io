use std::hash::{DefaultHasher, Hash, Hasher};

use bike::{Bike, BikeUpdate};
use nanoserde::{DeBin, SerBin};

pub mod bike;

pub const SQUARES: i16 = 80;

pub type Point = (i16, i16);

#[derive(Clone)]
pub struct Cell {
    val: u8,
}

impl Cell {
    const BIKE_MASK: u8 = 0b10000000;
    const BOOST_MASK: u8 = 0b1000000;
    const COLOR_MASK: u8 = 0b111111;

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

    pub fn get_color(&self) -> u8 {
        if self.val != 0 {
            (self.val & Self::COLOR_MASK) - 1
        } else {
            0
        }
    }

    fn free(&mut self, id: u8) {
        assert_eq!(self.val & Self::COLOR_MASK, id + 1);
        self.val &= !Self::BIKE_MASK;
    }
}

pub struct Occupied {
    occupied: Vec<Vec<Cell>>,
}

impl Occupied {
    pub fn new() -> Self {
        Self {
            occupied: vec![vec![Cell::new(); SQUARES as usize]; SQUARES as usize],
        }
    }

    pub fn is_occupied(&self, pos: Point) -> bool {
        if pos.0 < 0 || pos.1 < 0 || pos.0 >= SQUARES || pos.1 >= SQUARES {
            return true;
        }
        self.occupied[pos.1 as usize][pos.0 as usize].is_occupied()
    }

    pub fn get_cell(&self, pos: Point) -> Option<&Cell> {
        self.occupied
            .get(pos.1 as usize)?
            .get(pos.0 as usize)
    }

    pub fn occupy(&mut self, pos: Point, id: u8, boost: bool) -> bool {
        if pos.0 < 0 || pos.1 < 0 || pos.0 >= SQUARES || pos.1 >= SQUARES {
            return true;
        }

        if self.occupied[pos.1 as usize][pos.0 as usize].is_occupied() {
            return true;
        }
        self.occupied[pos.1 as usize][pos.0 as usize].occupy(id, true, boost);
        return false;
    }

    pub(crate) fn free(&mut self, pos: (i16, i16), id: u8) {
        self.occupied[pos.1 as usize][pos.0 as usize].free(id);
    }
}

#[derive(DeBin, SerBin, Debug, Clone, Default)]
pub struct GridUpdateMsg {
    pub tick: u32,
    pub hash: u64,
    pub updates: Vec<BikeUpdate>,
}

pub struct Grid {
    pub tick: u32,
    pub hash: u64,
    pub bikes: Vec<Bike>,
    pub occupied: Occupied,
    pub rng: quad_rand::RandGenerator,
}

pub enum UpdateResult {
    GameOver(Option<u8>),
    InProgress,
}

// pub enum GridSize {
//     Small,
//     Medium,
//     Large,
// }

// pub struct GridSettings {
// size: GridSize,
// teams
// }

impl Grid {
    pub fn new() -> Self {
        let mut occupied = Occupied::new();
        let bikes = vec![
            Bike::new(&mut occupied, 0, 6, (8, SQUARES / 2), bike::RIGHT),
            Bike::new(&mut occupied, 1, 10, (SQUARES - 9, SQUARES / 2), bike::LEFT),
            // Bike::new(&mut occupied, 2, 0, (SQUARES / 2 + 1, 11), bike::DOWN),
            // Bike::new(&mut occupied, 3, 4, (SQUARES / 2 - 1, SQUARES - 11), bike::UP),
        ];
        Self {
            hash: 0,
            bikes,
            occupied,
            tick: 0,
            rng: quad_rand::RandGenerator::new(),
        }
    }

    pub fn update(&mut self) -> UpdateResult {
        let mut winning_player: Option<u8> = None;
        let mut alive_players = 0;
        let mut hasher = DefaultHasher::new();
        for (i, bike) in self.bikes.iter_mut().enumerate() {
            if bike.update(&mut self.occupied) {
                winning_player = Some(i as u8);
                alive_players += 1;
            }

            // Compute hash for the bike
            bike.hash(&mut hasher);
        }
        self.hash = hasher.finish();
        if alive_players <= 1 {
            UpdateResult::GameOver(winning_player)
        } else {
            UpdateResult::InProgress
        }
    }
}

impl Grid {
    pub fn apply_updates(&mut self, updates: &GridUpdateMsg) -> UpdateResult {
        // tick and seed?
        for update in updates.updates.iter() {
            let bike = self.bikes.get_mut(update.id as usize).unwrap();
            bike.apply_update(update);
        }
        if self.tick != updates.tick {
            self.tick = updates.tick;
            self.update()
        } else {
            UpdateResult::InProgress
        }
    }
}
