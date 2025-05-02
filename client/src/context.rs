use macroquad::{
    math::Vec2,
    text::{Font, load_ttf_font},
    time::get_time,
    window::{screen_height, screen_width},
};

use crate::{assets_path::determine_asset_path, audio, input::InputContext, scene::EScene};

pub struct Context {
    pub font: Font,
    pub screen_size: Vec2,
    pub switch_scene_to: Option<EScene>,
    pub request_quit: bool,
    pub audio: audio::AudioAtlas,
    pub input: InputContext,
    pub time: f64,
}

impl Context {
    pub async fn default() -> Self {
        let base_assets_path = determine_asset_path();
        Self {
            font: load_ttf_font(base_assets_path.join("editundo.ttf").to_str().unwrap())
                .await
                .unwrap(),
            screen_size: Vec2::new(0.0, 0.0),
            switch_scene_to: None,
            request_quit: false,
            input: InputContext::new(),
            audio: audio::AudioAtlas::new(&base_assets_path).await,
            time: get_time(),
        }
    }

    pub fn update(&mut self) {
        self.input.update();
        self.screen_size = Vec2::new(screen_width(), screen_height());
        self.time = get_time();
    }
}
