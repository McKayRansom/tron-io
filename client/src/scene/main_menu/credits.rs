use macroquad::color::colors;

// use crate::audio::play_sfx;
use super::TITLE_Y_INSET;
// use super::VIRTUAL_HEIGHT;
use crate::audio::SoundFx;
use crate::context::VIRTUAL_HEIGHT;
use crate::text::draw_text_screen_centered;
// use crate::input::action_pressed;
use crate::text::Size;
use crate::ui::menu::MENU_SPACING;
use crate::context::Context;

/// sub-scene for displaying who worked on the game
pub struct Credits {
    pub active: bool,
}
impl Credits {
    pub fn new(_ctx: &Context) -> Self {
        Self { active: false }
    }
    pub fn draw(&mut self, ctx: &mut Context) {
        draw_text_screen_centered(
            ctx,
            "Credits",
            // X_INSET,
            TITLE_Y_INSET,
            Size::Large,
            colors::WHITE,
        );

        draw_text_screen_centered(
            ctx,
            "Game by McKay Ransom",
            // X_INSET,
            240.,
            Size::Medium,
            colors::GREEN,
        );
        draw_text_screen_centered(
            ctx,
            "Music by Jared Yelton",
            // X_INSET,
            240. + MENU_SPACING,
            Size::Medium,
            colors::BLUE,
        );

        draw_text_screen_centered(
            ctx,
            "Press any button to return",
            // X_INSET,
            VIRTUAL_HEIGHT - 120.,
            Size::Small,
            colors::WHITE,
        );
        if ctx.input.actions.len() > 0 {
            // Action::Confirm | Action::Cancel => {
            ctx.audio.play_sfx(SoundFx::MenuCancel);
            self.active = false;
            // }
            ctx.input.actions.clear();
        }
    }
}
