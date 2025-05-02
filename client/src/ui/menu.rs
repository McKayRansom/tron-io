use macroquad::{color::colors, math::Vec2};
use tron_io_world::Action;

use crate::{audio::play_sfx, context::Context, text};

pub struct MenuItem<V> {
    pub text: String,
    pub value: V,
}
impl<V> MenuItem<V> {
    pub fn new(text: String, value: V) -> Self {
        Self { text, value }
    }
    fn draw(&self, ctx: &mut Context, pos: Vec2, selected: bool) {
        let color = if selected {
            colors::GREEN
        } else {
            colors::WHITE
        };

        text::draw_text(
            ctx,
            self.text.as_str(),
            pos.x,
            pos.y,
            text::Size::Medium,
            color,
        );
    }
}

const MENU_SPACING: f32 = 40.;

pub struct Menu<V> {
    pub items: Vec<MenuItem<V>>,
    pub selected: usize,
}

impl<V> Menu<V> {
    pub fn new(items: Vec<MenuItem<V>>) -> Self {
        Self { items, selected: 0 }
    }

    pub fn select_next(&mut self) {
        self.selected = (self.selected + 1) % self.items.len();
    }

    pub fn select_previous(&mut self) {
        if self.selected == 0 {
            self.selected = self.items.len() - 1;
        } else {
            self.selected -= 1;
        }
    }

    pub fn draw(&mut self, ctx: &mut Context, mut pos: Vec2) -> Option<&V> {
        for (i, item) in self.items.iter().enumerate() {
            item.draw(ctx, pos, i == self.selected);
            pos.y += MENU_SPACING;
        }

        for action in ctx.input.actions.iter() {
            match action {
                Action::Up => {
                    play_sfx(ctx, &ctx.audio.sfx.menu_move);
                    self.select_previous();
                }
                Action::Down => {
                    play_sfx(ctx, &ctx.audio.sfx.menu_move);
                    self.select_next();
                }
                Action::Confirm => {
                    play_sfx(ctx, &ctx.audio.sfx.menu_select);
                    return Some(&self.items[self.selected].value);
                }
                _ => {}
            }
        }
        None
    }
}
