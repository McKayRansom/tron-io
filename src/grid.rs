use std::hash::{DefaultHasher, Hash, Hasher};

use bike::{Bike, BikeUpdate};
use colors::{DEFAULT_COLOR, get_color};
use macroquad::{
    math::Vec2,
    shapes::{draw_line, draw_rectangle},
    window::{screen_height, screen_width},
};
use nanoserde::{DeBin, SerBin};

pub mod bike;
pub mod colors;

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

    pub fn color(&self) -> macroquad::prelude::Color {
        if self.val != 0 {
            let mut color = get_color((self.val & Self::COLOR_MASK) - 1);
            if self.val & Self::BIKE_MASK != 0 {
                color.r += 0.3;
                color.g += 0.3;
                color.b += 0.3;
            }
            if self.val & Self::BOOST_MASK != 0 {
                color.r -= 0.2;
                color.g -= 0.2;
                color.b -= 0.2;
            }
            color
        } else {
            macroquad::color::colors::WHITE
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
    pub rng: macroquad::rand::RandGenerator,
}

pub struct GridDrawInfo {
    game_size: f32,
    offset_x: f32,
    offset_y: f32,
    sq_size: f32,
}

const MARGIN: f32 = 10.;

impl GridDrawInfo {
    pub fn new() -> Self {
        let game_size = screen_width().min(screen_height()) - MARGIN * 2.;
        let offset_x = (screen_width() - game_size) / 2.;
        let offset_y = (screen_height() - game_size) / 2.;
        let sq_size = game_size / SQUARES as f32;

        Self {
            game_size,
            offset_x,
            offset_y,
            sq_size,
        }
    }

    pub fn grid_to_screen(&self, pos: Point) -> Vec2 {
        Vec2::new(
            self.offset_x + pos.0 as f32 * self.sq_size,
            self.offset_y + pos.1 as f32 * self.sq_size,
        )
    }
    // pub fn screen_to_grid(&self, pos: Vec2) -> Point {
    //     let x = ((pos.x - self.offset_x) / self.sq_size).round() as i16;
    //     let y = ((pos.y - self.offset_y) / self.sq_size).round() as i16;
    //     (x, y)
    // }
}

pub enum UpdateResult {
    GameOver(u8),
    InProgress,
}

impl Grid {
    pub fn new() -> Self {
        let mut occupied = Occupied::new();
        let bikes = vec![
            Bike::new(
                &mut occupied,
                0,
                DEFAULT_COLOR,
                (8, SQUARES / 2),
                bike::RIGHT,
            ),
            Bike::new(&mut occupied, 1, 10, (SQUARES - 9, SQUARES / 2), bike::LEFT),
            // Bike::new(&mut occupied, 3, (SQUARES / 2, 11), bike::DOWN),
            // Bike::new(&mut occupied, 4, (SQUARES / 2, SQUARES - 11), bike::UP),
        ];
        Self {
            hash: 0,
            bikes,
            occupied,
            tick: 0,
            rng: macroquad::rand::RandGenerator::new(),
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
        if alive_players == 1 {
            UpdateResult::GameOver(winning_player.unwrap())
        } else {
            UpdateResult::InProgress
        }
    }

    pub fn draw(&self) {
        let draw_info = GridDrawInfo::new();
        draw_rectangle(
            draw_info.offset_x,
            draw_info.offset_y,
            draw_info.game_size,
            draw_info.game_size,
            macroquad::color::colors::BLACK,
        );

        const GRID_LINE_COLOR: macroquad::color::Color = macroquad::color::colors::DARKGRAY;
        const GRID_LINE_INTERVAL: i16 = 4;

        // draw lines every 4 squares
        for i in 0..SQUARES + 1 {
            if i % GRID_LINE_INTERVAL != 0 {
                continue;
            }
            let point_horix = draw_info.grid_to_screen((0, i));
            draw_line(
                point_horix.x,
                point_horix.y,
                point_horix.x + draw_info.game_size,
                point_horix.y,
                2.,
                GRID_LINE_COLOR,
            );
            let point_vert = draw_info.grid_to_screen((i, 0));
            draw_line(
                point_vert.x,
                point_vert.y,
                point_vert.x,
                point_vert.y + draw_info.game_size,
                2.,
                GRID_LINE_COLOR,
            );
        }
        // Draw bikes
        // TODO: draw player names, idea: use different fonts to show alive/boost/dead
        for y in 0..SQUARES {
            for x in 0..SQUARES {
                if self.occupied.occupied[y as usize][x as usize].is_occupied() {
                    let point = draw_info.grid_to_screen((x, y));
                    draw_rectangle(
                        point.x,
                        point.y,
                        draw_info.sq_size,
                        draw_info.sq_size,
                        self.occupied.occupied[y as usize][x as usize].color(),
                    );
                }
            }
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
