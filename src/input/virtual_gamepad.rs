use crate::context::Context;
use crate::text;

use macroquad::color::{Color, colors};
use macroquad::input::{MouseButton, is_mouse_button_pressed, mouse_position};
use macroquad::math::{Rect, Vec2};
use macroquad::shapes::draw_rectangle;
use tron_io::world::Action;

const VIRTUAL_BUTTON_COLOR: Color = Color::new(0.5, 0.5, 0.5, 1.0);
const VIRTUAL_BUTTON_SIZE: f32 = 50.0;

#[derive(Clone, Copy)]
pub struct VirtualGamepad {
    is_active: bool,
    pub action: Option<Action>,
    last_action: Option<Action>,
    last_active_time: f64,
}

impl VirtualGamepad {
    pub fn new() -> Self {
        Self {
            is_active: false,
            action: None,
            last_action: None,
            last_active_time: 0.0,
        }
    }

    pub fn virtual_button_rect(&self, context: &Context, action: Action) -> Rect {
        let button_center: Vec2 = Vec2::new(
            context.screen_size.x - VIRTUAL_BUTTON_SIZE * 2.,
            context.screen_size.y / 2.,
        );
        let arrow_center: Vec2 = Vec2::new(
            VIRTUAL_BUTTON_SIZE * 2.,
            context.screen_size.y / 2.,
        );
        let pos = match action {
            Action::Up => arrow_center + Vec2::new(0., -VIRTUAL_BUTTON_SIZE),
            Action::Down => arrow_center + Vec2::new(0., VIRTUAL_BUTTON_SIZE),
            Action::Left => arrow_center + Vec2::new(-VIRTUAL_BUTTON_SIZE, 0.),
            Action::Right => arrow_center + Vec2::new(VIRTUAL_BUTTON_SIZE, 0.),
            Action::Confirm => button_center + Vec2::new(-VIRTUAL_BUTTON_SIZE, 0.),
            Action::Cancel => button_center + Vec2::new(0., -VIRTUAL_BUTTON_SIZE),
            Action::Reset => todo!(),
            Action::Rewind => todo!(),
            Action::Pause => todo!(),
        };
        Rect::new(
            pos.x - VIRTUAL_BUTTON_SIZE / 2.,
            pos.y - VIRTUAL_BUTTON_SIZE / 2.,
            VIRTUAL_BUTTON_SIZE,
            VIRTUAL_BUTTON_SIZE,
        )
    }

    pub fn update(&self, ctx: &Context) -> VirtualGamepad {
        if is_mouse_button_pressed(MouseButton::Left) {
            let mouse_pos = mouse_position().into();
            for action in [Action::Up, Action::Down, Action::Left, Action::Right] {
                let pos = self.virtual_button_rect(ctx, action);
                if pos.contains(mouse_pos) {
                    return VirtualGamepad {
                        is_active: true,
                        action: Some(action),
                        last_action: Some(action),
                        last_active_time: ctx.time,
                    };
                }
            }
            Self {
                is_active: true,
                action: None,
                last_action: None,
                last_active_time: ctx.time,
            }
        } else if self.is_active && ctx.time - self.last_active_time > 10. {
            VirtualGamepad {
                is_active: false,
                action: None,
                last_action: None,
                last_active_time: self.last_active_time,
            }
        } else {
            VirtualGamepad {
                is_active: self.is_active,
                action: None,
                last_action: self.last_action,
                last_active_time: self.last_active_time,
            }
        }
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
        let rect = self.virtual_button_rect(context, action);
        draw_rectangle(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            if self.last_action == Some(action) && context.time - self.last_active_time < 0.5 {
                colors::WHITE
            } else {
                VIRTUAL_BUTTON_COLOR
            },
        );
        text::draw_text_centered_pos(
            context,
            text,
            rect.x + VIRTUAL_BUTTON_SIZE / 2.,                            // + size / 2.0 + padding,
            rect.y + VIRTUAL_BUTTON_SIZE , // + size / 2.0 + padding,
            crate::text::Size::Medium,
            colors::WHITE,
        );
    }
}
