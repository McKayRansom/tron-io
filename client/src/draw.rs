use macroquad::{
    camera::{set_camera, set_default_camera, Camera2D},
    color::{Color, WHITE},
    math::{vec2, Rect, Vec2},
    prelude::{collections::storage, gl_use_default_material, gl_use_material},
    shapes::{draw_line, draw_rectangle, draw_rectangle_lines},
    texture::{
        draw_texture_ex, DrawTextureParams, RenderTarget
    },
    window::{clear_background, screen_height, screen_width},
};
use tron_io_world::grid::{Cell, Grid, Point};

use crate::BACKGROUND_COLOR;

pub fn cell_color(cell: &Cell) -> Color {
    if cell.is_exploded() {
        return macroquad::color::colors::WHITE;
    }
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

const VIEWPORT_SIZE: f32 = 1024.;

impl GridDrawInfo {
    pub fn new(grid: &Grid) -> Self {
        let game_size = VIEWPORT_SIZE - MARGIN * 2.;
        let offset_x = (screen_width() - game_size) / 2.;
        let offset_y = (screen_height() - game_size) / 2.;
        let sq_size = game_size /grid.size().0 as f32;

        Self {
            game_size,
            offset_x,
            offset_y,
            sq_size,
        }
    }

    pub fn grid_to_screen(&self, pos: Point) -> Vec2 {
        Vec2::new(
            MARGIN + pos.0 as f32 * self.sq_size,
            MARGIN + pos.1 as f32 * self.sq_size,
        )
    }
    // pub fn screen_to_grid(&self, pos: Vec2) -> Point {
    //     let x = ((pos.x - self.offset_x) / self.sq_size).round() as i16;
    //     let y = ((pos.y - self.offset_y) / self.sq_size).round() as i16;
    //     (x, y)
    // }
}

pub fn draw_grid(grid: &Grid) {
    // draw with CRT?
    // TODO: Draw entire thing at once to camera buf!
    // gl_use_material(&storage::get::<Material>());

    let draw_info = GridDrawInfo::new(grid);

    let view_area = screen_height().min(screen_width());

    let camera = Camera2D {
        zoom: vec2(2.0 / VIEWPORT_SIZE, 2.0 / VIEWPORT_SIZE),
        target: vec2(VIEWPORT_SIZE / 2., VIEWPORT_SIZE / 2.),
        render_target: Some(storage::get::<RenderTarget>().clone()),
        ..Default::default()
    };
    set_camera(&camera);
    clear_background(BACKGROUND_COLOR);

    // draw_rectangle(
    //     draw_info.offset_x - MARGIN,
    //     draw_info.offset_y - MARGIN,
    //     draw_info.game_size + MARGIN * 2.,
    //     draw_info.game_size + MARGIN * 2.,
    //     // Color::from_hex(0x020a13),
    //     Color {
    //         r: 0.13,
    //         g: 0.13,
    //         b: 0.13,
    //         a: 1.0,
    //     },
    //     // Color { r: 0.30, g: 0.30, b: 0.30, a: 1.0 },
    // );

    const GRID_LINE_COLOR: macroquad::color::Color = macroquad::color::colors::GRAY;
    const GRID_LINE_THICKNESS: f32 = 2.;
    // const GRID_LINE_INTERVAL: i16 = 5;

    // draw lines every 4 squares
    let (size_y, size_x) = grid.size();
    for i in 0..size_x + 1 {
        if i != 0 && i != size_x {
            // if i % GRID_LINE_INTERVAL != 0 {
            continue;
        }
        let point_horix = draw_info.grid_to_screen((0, i));
        draw_line(
            point_horix.x,
            point_horix.y,
            point_horix.x + draw_info.game_size,
            point_horix.y,
            GRID_LINE_THICKNESS,
            GRID_LINE_COLOR,
        );
        let point_vert = draw_info.grid_to_screen((i, 0));
        draw_line(
            point_vert.x,
            point_vert.y,
            point_vert.x,
            point_vert.y + draw_info.game_size,
            GRID_LINE_THICKNESS,
            GRID_LINE_COLOR,
        );
    }
    // Draw bikes
    // TODO: draw player names, idea: use different fonts to show alive/boost/dead
    for y in 0..size_y {
        for x in 0..size_x {
            if grid.occupied.is_occupied((x, y)) {
                let point = draw_info.grid_to_screen((x, y));
                draw_rectangle(
                    point.x,
                    point.y,
                    draw_info.sq_size,
                    draw_info.sq_size,
                    cell_color(grid.occupied.get_cell((x, y)).unwrap()),
                );
            }
        }
    }

    set_default_camera();
    // clear_background(WHITE);

    let material = storage::get();
    gl_use_material(&material);
    draw_texture_ex(
        &camera.render_target.unwrap().texture,
        (screen_width() - view_area) / 2.,
        (screen_height() - view_area) / 2.,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(view_area, view_area)),
            ..Default::default()
        },
    );
    gl_use_default_material();
}

pub fn draw_rect(rect: Rect, color: Color) {
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, color);
}

pub fn draw_rect_lines(rect: Rect, thickness: f32, color: Color) {
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, thickness, color);
}
