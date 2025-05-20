use macroquad::{color::colors, math::vec2};
use tron_io_world::{GridOptions, GridSize, MAX_TEAMS, MIN_PLAYERS, MIN_TEAMS};

use crate::{
    context::Context,
    scene::{EScene},
    text::{self, draw_text},
    ui::menu::{Menu, MenuAction},
};

use super::{MENU_CONTENT_Y, TITLE_Y_INSET, X_INSET};

pub struct GameOptionsScene {
    // menu: Menu<SettingOption>,
    pub options: GridOptions,
    pub selected: usize,
    pub active: bool,
}

impl GameOptionsScene {
    pub fn new(_ctx: &mut Context) -> Self {
        Self {
            options: GridOptions::default(),
            active: false,
            selected: 0,
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) {
        draw_text(
            ctx,
            "Tron-IO/Menu/GridOptions",
            X_INSET,
            TITLE_Y_INSET,
            text::Size::Large,
            colors::WHITE,
        );

        let mut menu = Menu::new(vec2(X_INSET, MENU_CONTENT_Y), self.selected);


        if menu.option("Start", ctx) {
            ctx.switch_scene_to = Some(EScene::Gameplay(self.options));
        }

        match menu.option_ex(
            format!("<Size: {:?}>", self.options.grid_size).as_str(),
            ctx,
        ) {
            Some(MenuAction::Enter) => self.options.grid_size = GridSize::default(),
            Some(MenuAction::Right) => self.options.grid_size.incr(),
            Some(MenuAction::Left) => self.options.grid_size.decr(),
            _ => {}
        }


        match menu.option_ex(format!("<Teams: {}>", self.options.teams).as_str(), ctx) {
            Some(MenuAction::Enter) => self.options.teams = MIN_TEAMS,
            Some(MenuAction::Right) => {
                if self.options.teams < MAX_TEAMS {
                    self.options.teams += 1;
                }
            }
            Some(MenuAction::Left) => {
                if self.options.teams > MIN_TEAMS {
                    self.options.teams -= 1;
                }
            }
            _ => {}
        }

        match menu.option_ex(format!("<Players: {}>", self.options.players).as_str(), ctx) {
            Some(MenuAction::Enter) => self.options.players = MIN_PLAYERS,
            Some(MenuAction::Right) => {
                if self.options.players < tron_io_world::MAX_PLAYERS {
                    self.options.players += 1;
                }
            }
            Some(MenuAction::Left) => {
                if self.options.players > MIN_PLAYERS {
                    self.options.players -= 1;
                }
            }
            _ => {}
        }


        self.selected = menu.finish();
    }
}
