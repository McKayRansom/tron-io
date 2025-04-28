// use super::credits::Credits;
// use super::settings::Settings;
use super::{EScene, Scene};
use crate::audio::play_sfx;
// use crate::consts::*;
use crate::context::Context;
use crate::input::{Action, action_pressed};
use crate::text::{self, draw_text};
use macroquad::color::colors;
use tron_io::world::client::WorldClient;
use tron_io::world::local::WorldClientLocal;

mod lobby;

pub struct MainMenu {
    menu_options: Vec<MenuOption>,
    menu_index: usize,
    // settings_subscene: Settings,
    // credits_subscene: Credits,
    lobby: Option<lobby::Lobby>,
}

enum MenuOption {
    Local,
    Online,
    // Settings,
    // Credits,
    #[cfg(not(target_family = "wasm"))]
    Quit,
}

const X_INSET: f32 = 100.;
const TITLE_Y_INSET: f32 = 100.;
const MENU_CONTENT_Y: f32 = 400.;

impl MainMenu {
    pub async fn new(_ctx: &mut Context) -> Self {
        let menu_options = vec![
            MenuOption::Local,
            MenuOption::Online,
            // MenuOption::Settings,
            // MenuOption::Credits,
            #[cfg(not(target_family = "wasm"))]
            MenuOption::Quit,
        ];

        Self {
            menu_options,
            menu_index: 0,
            // settings_subscene: Settings::new(ctx, false),
            // credits_subscene: Credits::new(ctx),
            lobby: None,
        }
    }

    fn text_for_menu_option(&self, menu_option: &MenuOption) -> &str {
        match menu_option {
            MenuOption::Local => "Local",
            MenuOption::Online => "Online",
            // MenuOption::Settings => "Settings",
            // MenuOption::Credits => "Credits",
            #[cfg(not(target_family = "wasm"))]
            MenuOption::Quit => "Quit",
        }
    }
}

impl Scene for MainMenu {
    fn update(&mut self, ctx: &mut Context) {
        // if self.settings_subscene.active {
        //     self.settings_subscene.update(ctx);
        //     return;
        // }

        // if self.credits_subscene.active {
        //     self.credits_subscene.update(ctx);
        //     return;
        // }
        if let Some(lobby) = &mut self.lobby {
            lobby.update(ctx);
            return;
        }

        let menu_option = self
            .menu_options
            .get(self.menu_index)
            .expect("pause menu index out of bounds");

        if action_pressed(Action::Confirm, &ctx.gamepads) {
            play_sfx(ctx, &ctx.audio.sfx.menu_select);

            match menu_option {
                MenuOption::Local => {
                    ctx.switch_scene_to = Some(EScene::Gameplay(super::GameOptions {
                        client: WorldClient::new(Box::new(WorldClientLocal::new())),
                    }));
                }
                MenuOption::Online => {
                    // ctx.switch_scene_to = Some(EScene::Gameplay);
                    self.lobby = Some(lobby::Lobby::new(ctx));
                }
                // MenuOption::Settings => {
                //     self.settings_subscene.active = true;
                // }
                // MenuOption::Credits => {
                //     self.credits_subscene.active = true;
                // }
                #[cfg(not(target_family = "wasm"))]
                MenuOption::Quit => {
                    ctx.request_quit = true;
                }
            }
        }

        if action_pressed(Action::Up, &ctx.gamepads) {
            play_sfx(ctx, &ctx.audio.sfx.menu_move);

            if self.menu_index == 0 {
                self.menu_index = self.menu_options.len() - 1;
            } else {
                self.menu_index -= 1;
            }
        }
        if action_pressed(Action::Down, &ctx.gamepads) {
            play_sfx(ctx, &ctx.audio.sfx.menu_move);

            if self.menu_index == self.menu_options.len() - 1 {
                self.menu_index = 0;
            } else {
                self.menu_index += 1;
            }
        }
    }
    fn draw(&mut self, ctx: &mut Context) {
        // if self.settings_subscene.active {
        //     self.settings_subscene.draw(ctx);
        //     return;
        // }

        // if self.credits_subscene.active {
        //     self.credits_subscene.draw(ctx);
        //     return;
        // }

        draw_text(
            ctx,
            if self.lobby.is_some() {
                "Tron-IO/Menu/Online"
            } else {
                "Tron-IO/Menu"
            },
            X_INSET,
            TITLE_Y_INSET,
            text::Size::Large,
            colors::WHITE,
        );

        if let Some(lobby) = &mut self.lobby {
            lobby.draw(ctx);
            return;
        }

        for (i, menu_option) in self.menu_options.iter().enumerate() {
            let color = if self.menu_index == i {
                colors::GREEN
            } else {
                colors::WHITE
            };

            draw_text(
                ctx,
                self.text_for_menu_option(menu_option),
                X_INSET,
                MENU_CONTENT_Y + (i as f32 * 40.),
                text::Size::Medium,
                color,
            );
        }

        draw_text(
            ctx,
            "Change Select = Arrow Keys | Confirm = Z",
            X_INSET,
            ctx.screen_size.y - 40.,
            text::Size::Small,
            colors::WHITE,
        );

        draw_text(
            ctx,
            format!("v{}", crate::VERSION).as_str(),
            40.,
            ctx.screen_size.y - 40.,
            text::Size::Small,
            colors::WHITE,
        );
    }
}
