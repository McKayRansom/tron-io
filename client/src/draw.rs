use macroquad::{color::Color, math::{Rect, Vec2}, shapes::{draw_line, draw_rectangle, draw_rectangle_lines}, window::{screen_height, screen_width}};
use tron_io_world::grid::{Cell, Grid, Point, SQUARES};

pub fn cell_color(cell: &Cell) -> Color {
    let mut color = crate::colors::get_color(cell.get_color());
    if cell.is_bike() {
        color.r += 0.3;
        color.g += 0.3;
        color.b += 0.3;
    }
    if cell.is_boost() {
        color.r -= 0.2;
        color.g -= 0.2;
        color.b -= 0.2;
    }
    color
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

pub fn draw_grid(grid: &Grid) {
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
            if grid.occupied.is_occupied((x, y)) {
                let point = draw_info.grid_to_screen((x, y));
                draw_rectangle(
                    point.x,
                    point.y,
                    draw_info.sq_size,
                    draw_info.sq_size,
                    cell_color(&grid.occupied.get_cell((x, y))),
                );
            }
        }
    }
}

pub fn draw_rect(rect: Rect, color: Color) {
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, color);
}

pub fn draw_rect_lines(rect: Rect, thickness: f32, color: Color) {
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, thickness, color);
}
