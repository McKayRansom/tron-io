use crate::context::Context;
use crate::text;

use macroquad::color::{Color, colors};
use macroquad::input::{MouseButton, is_mouse_button_pressed, mouse_position};
use macroquad::math::{Rect, Vec2, vec2};
use macroquad::shapes::draw_rectangle;
use macroquad::time::get_time;
use macroquad::window::{screen_height, screen_width};
use tron_io_world::Action;

const VIRTUAL_BUTTON_COLOR: Color = Color::new(0.5, 0.5, 0.5, 1.0);

#[derive(Clone, Copy)]
pub struct VirtualGamepad {
    virtual_button_size: f32,
    is_active: bool,
    screen_size: Vec2,
    button_center: Vec2,
    arrow_center: Vec2,
    action: Option<Action>,
    last_action: Option<Action>,
    last_active_time: f64,
}

impl VirtualGamepad {
    pub fn new() -> Self {
        Self {
            virtual_button_size: 50.,
            is_active: false,
            screen_size: Vec2::new(0.0, 0.0),
            arrow_center: vec2(0., 0.),
            button_center: vec2(0., 0.),
            action: None,
            last_action: None,
            last_active_time: 0.0,
        }
    }

    pub fn update_pos(&mut self) {
        self.screen_size = Vec2::new(screen_width(), screen_height());
        self.virtual_button_size = self.screen_size.x / 16.;
        self.button_center = vec2(
            self.screen_size.x - self.virtual_button_size * 2.,
            self.screen_size.y / 2.,
        );
        self.arrow_center = vec2(self.virtual_button_size * 2., self.screen_size.y / 2.);
    }

    pub fn virtual_button_rect(&self, action: Action) -> Rect {
        let pos = match action {
            Action::Up => self.arrow_center + Vec2::new(0., -self.virtual_button_size),
            Action::Down => self.arrow_center + Vec2::new(0., self.virtual_button_size),
            Action::Left => self.arrow_center + Vec2::new(-self.virtual_button_size, 0.),
            Action::Right => self.arrow_center + Vec2::new(self.virtual_button_size, 0.),
            Action::Confirm => self.button_center + Vec2::new(self.virtual_button_size, 0.),
            Action::Cancel => self.button_center + Vec2::new(-self.virtual_button_size, 0.),
            Action::Reset => todo!(),
            Action::Rewind => todo!(),
            Action::Pause => todo!(),
        };
        Rect::new(
            pos.x - self.virtual_button_size / 2.,
            pos.y - self.virtual_button_size / 2.,
            self.virtual_button_size,
            self.virtual_button_size,
        )
    }

    pub fn update(&mut self) -> Option<Action> {
        if is_mouse_button_pressed(MouseButton::Left) {
            self.update_pos();
            self.is_active = true;
            self.last_active_time = get_time();
            let mouse_pos: Vec2 = mouse_position().into();
            let action =
                if mouse_pos.distance(self.arrow_center) < mouse_pos.distance(self.button_center) {
                    let diff = mouse_pos - self.arrow_center;
                    if diff.x.abs() > diff.y.abs() {
                        if diff.x > 0. {
                            Action::Right
                        } else {
                            Action::Left
                        }
                    } else {
                        if diff.y > 0. {
                            Action::Down
                        } else {
                            Action::Up
                        }
                    }
                } else {
                    let diff = mouse_pos - self.button_center;
                    // if diff.x.abs() > diff.y.abs() {
                    if diff.x > 0. {
                        Action::Confirm
                    } else {
                        Action::Cancel
                    }
                    // }
                };
            self.action = Some(action);
            self.last_action = Some(action);
            return Some(action);
        } else if self.is_active && get_time() - self.last_active_time > 10. {
            self.is_active = false;
            self.last_action = None;
        }
        None
    }

    pub fn draw(&self, context: &Context) {
        if !self.is_active {
            return;
        }
        // arrows
        self.draw_virtual_button(context, Action::Up, "U");
        self.draw_virtual_button(context, Action::Down, "D");
        self.draw_virtual_button(context, Action::Left, "L");
        self.draw_virtual_button(context, Action::Right, "R");

        // buttons
        self.draw_virtual_button(context, Action::Confirm, "A");
        self.draw_virtual_button(context, Action::Cancel, "B");
    }

    fn draw_virtual_button(&self, context: &Context, action: Action, text: &str) {
        // let color = PLAYER_COLOR_LOOKUP[0];
        let rect = self.virtual_button_rect(action);
        draw_rectangle(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            if self.last_action == Some(action) && get_time() - self.last_active_time < 0.2 {
                colors::WHITE
            } else {
                VIRTUAL_BUTTON_COLOR
            },
        );
        text::draw_text_centered_pos(
            context,
            text,
            rect.x + self.virtual_button_size / 2., // + size / 2.0 + padding,
            rect.y + self.virtual_button_size,      // + size / 2.0 + padding,
            crate::text::Size::Medium,
            colors::WHITE,
        );
    }
}
