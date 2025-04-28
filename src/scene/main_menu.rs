// use super::credits::Credits;
// use super::settings::Settings;
use super::{EScene, Scene};
// use crate::consts::*;
use crate::context::Context;
use crate::text::{self, draw_text};
use crate::ui::menu::{Menu, MenuItem};
use macroquad::color::colors;
use macroquad::math::vec2;
use tron_io::world::client::WorldClient;
use tron_io::world::local::WorldClientLocal;

mod lobby;

pub struct MainMenu {
    menu: Menu<MenuOption>,
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
        Self {
            menu: Menu::new(vec![
                MenuItem::new("Local".into(), MenuOption::Local),
                MenuItem::new("Online".into(), MenuOption::Online),
                // MenuItem::new("Settings".into(), MenuOption::Settings),
                #[cfg(not(target_family = "wasm"))]
                MenuItem::new("Quit".into(), MenuOption::Quit),
            ]),
            lobby: None,
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

        let menu_pos = vec2(X_INSET, MENU_CONTENT_Y);

        if let Some(selected) = self.menu.draw(ctx, menu_pos) {
            match selected {
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
