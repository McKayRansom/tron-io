use macroquad::{color::colors, math::vec2};

use crate::{
    context::Context,
    settings::{SOUND_MAX, SOUND_MIN},
    text::{self, draw_text},
    ui::menu::{Menu, MenuAction},
};

use super::{MENU_CONTENT_Y, TITLE_Y_INSET, X_INSET};

pub struct SettingsScene {
    // menu: Menu<SettingOption>,
    pub selected: usize,
    pub active: bool,
}

impl SettingsScene {
    pub fn new(_ctx: &mut Context) -> Self {
        Self {
            active: false,
            selected: 0,
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) {
        draw_text(
            ctx,
            "Tron-IO/Menu/Settings",
            X_INSET,
            TITLE_Y_INSET,
            text::Size::Large,
            colors::WHITE,
        );

        let mut menu = Menu::new(vec2(X_INSET, MENU_CONTENT_Y), self.selected);

        match menu.option_ex(format!("<Sound: {}>", ctx.settings.sound).as_str(), ctx) {
            Some(MenuAction::Enter) => {
                if ctx.settings.sound == 0 {
                    ctx.settings.sound = SOUND_MAX;
                } else {
                    ctx.settings.sound = SOUND_MIN;
                }
            }
            Some(MenuAction::Right) => {
                if ctx.settings.sound < SOUND_MAX {
                    ctx.settings.sound += 1;
                }
            }
            Some(MenuAction::Left) => {
                if ctx.settings.sound > SOUND_MIN {
                    ctx.settings.sound -= 1;
                }
            }
            _ => {}
        }

        match menu.option_ex(format!("<Music: {}>", ctx.settings.music).as_str(), ctx) {
            Some(MenuAction::Enter) => {
                if ctx.settings.music == 0 {
                    ctx.settings.music = SOUND_MAX;
                } else {
                    ctx.settings.music = SOUND_MIN;
                }
            }
            Some(MenuAction::Right) => {
                if ctx.settings.music < SOUND_MAX {
                    ctx.settings.music += 1;
                }
            }
            Some(MenuAction::Left) => {
                if ctx.settings.music > SOUND_MIN {
                    ctx.settings.music -= 1;
                }
            }
            _ => {}
        }

        match menu.option_ex(format!("<Fullscreen: {}>", if ctx.settings.fullscreen {"On" } else {"Off"}).as_str(), ctx) {
            Some(_) => ctx.settings.fullscreen = !ctx.settings.fullscreen,
            None => {}
        }

        if menu.option("Save and exit", ctx) {
            ctx.save_settings();
            self.active = false;
        }

        self.selected = menu.finish();
    }
}
