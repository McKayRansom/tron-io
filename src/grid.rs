use std::hash::{DefaultHasher, Hash, Hasher};

use bike::Bike;
use macroquad::{
    color::{Color, colors},
    math::Vec2,
    shapes::{draw_line, draw_rectangle},
    window::{screen_height, screen_width},
};

pub mod bike;
pub mod msg;

pub const SQUARES: i16 = 80;

pub type Point = (i16, i16);

#[derive(Clone)]
pub struct Cell {
    val: u8,
}

impl Cell {
    pub fn new() -> Self {
        Self { val: 0 }
    }

    pub fn occupy(&mut self, val: u8, is_bike: bool) {
        self.val = val | if is_bike { 0b10000000 } else { 0 };
    }

    pub fn is_occupied(&self) -> bool {
        self.val & 0b01111111 != 0
    }

    pub fn color(&self) -> Color {
        if self.val != 0 {
            let mut color = PLAYER_COLOR_LOOKUP[(self.val & 0b01111111) as usize];
            if self.val & 0b10000000 != 0 {
                color.r += 0.2;
                color.g += 0.2;
                color.b += 0.2;
            }
            color
        } else {
            colors::WHITE
        }
    }

    fn free(&mut self, id: u8) {
        assert_eq!(self.val & 0b01111111, id);
        self.val &= 0b01111111;
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

    pub fn occupy(&mut self, pos: Point, id: u8) -> bool {
        if pos.0 < 0 || pos.1 < 0 || pos.0 >= SQUARES || pos.1 >= SQUARES {
            return true;
        }

        if self.occupied[pos.1 as usize][pos.0 as usize].is_occupied() {
            return true;
        }
        self.occupied[pos.1 as usize][pos.0 as usize].occupy(id, true);
        return false;
    }

    pub(crate) fn free(&mut self, pos: (i16, i16), id: u8) {
        self.occupied[pos.1 as usize][pos.0 as usize].free(id);
    }
}

pub struct Grid {
    pub tick: u32,
    pub bikes: Vec<Bike>,
    pub occupied: Occupied,
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

const PLAYER_COLOR_LOOKUP: &[Color] = &[
    colors::WHITE,
    colors::BLUE,
    colors::RED,
    colors::DARKGREEN,
    colors::GOLD,
];

pub enum UpdateResult {
    GameOver,
    GameWon,
    InProgress,
}

impl Grid {
    pub fn new() -> Self {
        let mut occupied = Occupied::new();
        let bikes = vec![
            Bike::new(&mut occupied, 1, (8, SQUARES / 2), bike::RIGHT),
            Bike::new(&mut occupied, 2, (SQUARES - 9, SQUARES / 2), bike::LEFT),
            Bike::new(&mut occupied, 3, (SQUARES / 2, 11), bike::DOWN),
            Bike::new(&mut occupied, 4, (SQUARES / 2, SQUARES - 11), bike::UP),
        ];
        Self { bikes, occupied, tick: 0 }
    }

    pub fn update(&mut self) -> UpdateResult {
        let mut all_snakes_dead = true;
        let mut hasher = DefaultHasher::new();
        for (i, bike) in self.bikes.iter_mut().enumerate() {
            
            if bike.update(&mut self.occupied, i != 0) {
                if i == 0 {
                    // player died
                    return UpdateResult::GameOver;
                }
            } else if i != 0 {
                all_snakes_dead = false;
            }

            // Compute hash for the bike
            bike.hash(&mut hasher);
            // println!("Bike {} hash: {}", i, bike_hash); // Replace with appropriate logging if needed

        }
        let _bike_hash = hasher.finish();
        if all_snakes_dead {
            UpdateResult::GameWon
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
            colors::BLACK,
        );

        // draw lines every 4 squares
        for i in 0..SQUARES + 1 {
            if i % 4 != 0 {
                continue;
            }
            let point_horix = draw_info.grid_to_screen((0, i));
            draw_line(
                point_horix.x,
                point_horix.y,
                point_horix.x + draw_info.game_size,
                point_horix.y,
                2.,
                colors::GRAY,
            );
            let point_vert = draw_info.grid_to_screen((i, 0));
            draw_line(
                point_vert.x,
                point_vert.y,
                point_vert.x,
                point_vert.y + draw_info.game_size,
                2.,
                colors::GRAY,
            );
        }
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
