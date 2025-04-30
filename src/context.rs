use gamepads::Gamepads;
use macroquad::{
    math::Vec2, text::{load_ttf_font, Font}, time::get_time, window::{screen_height, screen_width}
};

use crate::{assets_path::determine_asset_path, audio, input::virtual_gamepad::VirtualGamepad, scene::EScene};

pub struct Context {
    pub font: Font,
    #[allow(dead_code)]
    pub screen_size: Vec2,
    pub switch_scene_to: Option<EScene>,
    pub request_quit: bool,
    pub gamepads: Gamepads,
    pub virtual_gamepad: VirtualGamepad,
    pub audio: audio::AudioAtlas,
    pub time: f64,
}

impl Context {
    pub async fn default() -> Self {

        let base_assets_path = determine_asset_path();
        Self {
            font: load_ttf_font("assets/editundo.ttf").await.unwrap(),
            screen_size: Vec2::new(0.0, 0.0),
            switch_scene_to: None,
            request_quit: false,
            gamepads: Gamepads::new(),
            virtual_gamepad: VirtualGamepad::new(),
            audio: audio::AudioAtlas::new(&base_assets_path).await,
            time: get_time(),
        }
    }

    pub fn update(&mut self) {
        self.gamepads.poll();
        self.screen_size = Vec2::new(screen_width(), screen_height());
        self.time = get_time();
        self.virtual_gamepad = self.virtual_gamepad.update(self);
    }
}
