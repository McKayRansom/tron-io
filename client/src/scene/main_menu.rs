// use super::settings::Settings;
use super::Scene;
// use crate::consts::*;
use crate::context::{Context, VIRTUAL_HEIGHT};
use crate::text::{self, draw_text, draw_text_screen_centered};
use crate::ui::menu::{Menu};
use credits::Credits;
use game_options::GameOptionsScene;
use macroquad::color::colors;
use macroquad::math::vec2;
use settings_scene::SettingsScene;

mod credits;
mod lobby;
mod settings_scene;
mod game_options;

pub struct MainMenu {
    selected: usize,
    settings_subscene: SettingsScene,
    credits_subscene: Credits,
    options_subscene: GameOptionsScene,
    lobby: Option<lobby::Lobby>,
}

const X_INSET: f32 = 100.;
const TITLE_Y_INSET: f32 = 100.;
const MENU_CONTENT_Y: f32 = 200.;

impl MainMenu {
    pub async fn new(ctx: &mut Context) -> Self {
        Self {
            selected: 0,
            credits_subscene: Credits::new(ctx),
            settings_subscene: SettingsScene::new(ctx),
            options_subscene: GameOptionsScene::new(ctx),
            lobby: None,
        }
    }
}

impl Scene for MainMenu {
    fn update(&mut self, ctx: &mut Context) {
        if let Some(lobby) = &mut self.lobby {
            lobby.update(ctx);
            return;
        }
    }

    fn draw(&mut self, ctx: &mut Context) {
        if self.settings_subscene.active {
            self.settings_subscene.draw(ctx);
            return;
        }

        if self.credits_subscene.active {
            self.credits_subscene.draw(ctx);
            return;
        }

        if self.options_subscene.active {
            self.options_subscene.draw(ctx);
            return;
        }

        draw_text_screen_centered(
            ctx,
            if self.lobby.is_some() {
                "Tron-IO/Menu/Online"
            } else {
                "Tron-IO/Menu"
            },
            TITLE_Y_INSET,
            text::Size::Large,
            colors::WHITE,
        );

        if let Some(lobby) = &mut self.lobby {
            lobby.draw(ctx);
            return;
        }

        let mut menu = Menu::new(vec2(X_INSET, MENU_CONTENT_Y), self.selected);

        if menu.option("New Grid", ctx) {
            self.options_subscene.active = true;
        }
        if menu.option("Online", ctx) {
            self.lobby = Some(lobby::Lobby::new(ctx));
        }
        if menu.option("Settings", ctx) {
            self.settings_subscene.active = true;
        }
        if menu.option("Credits", ctx) {
            self.credits_subscene.active = true;
        }

        #[cfg(not(target_family = "wasm"))]
        if menu.option("Quit", ctx) {
            ctx.request_quit = true;
        }

        self.selected = menu.finish();
        // vec![
        //     MenuItem::new("Settings".into(), MenuOption::Settings),
        //     MenuItem::new("Credits".into(), MenuOption::Credits),
        //     #[cfg(not(target_family = "wasm"))]
        //     MenuItem::new("Quit".into(), MenuOption::Quit),
        // ]),

        draw_text(
            ctx,
            "Change Select = [arrow keys] or [WASD] | Confirm = [Enter] or [LSHIFT]",
            X_INSET,
            VIRTUAL_HEIGHT - 40.,
            text::Size::Small,
            colors::WHITE,
        );

        draw_text(
            ctx,
            format!("v{}", crate::VERSION).as_str(),
            40.,
            VIRTUAL_HEIGHT - 40.,
            text::Size::Small,
            colors::WHITE,
        );
    }
}
