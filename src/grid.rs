use macroquad::{
    color::{colors, Color},
    shapes::{draw_line, draw_rectangle},
    window::{screen_height, screen_width},
};

pub const SQUARES: i16 = 80;

pub type Point = (i16, i16);

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
            colors::BLACK,
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
                colors::GRAY,
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
                colors::GRAY,
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
