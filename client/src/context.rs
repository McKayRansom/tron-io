use macroquad::{
    prelude::{Material, ShaderSource, load_material},
    text::{Font, load_ttf_font},
    texture::{RenderTarget, render_target},
    time::get_time,
    window::set_fullscreen,
};

use crate::{
    assets_path::determine_asset_path, audio, input::InputContext, scene::EScene,
    settings::GameSettings,
};

pub struct Context {
    pub font: Font,
    // pub font_dot: Font,
    pub font_line: Font,
    // pub screen_size: Vec2,
    pub switch_scene_to: Option<EScene>,
    pub request_quit: bool,
    pub audio: audio::AudioAtlas,
    pub input: InputContext,
    pub time: f64,
    pub settings: GameSettings,
    pub crt_material: Material,
    pub glow_material: Material,
    pub render_target: RenderTarget,
    pub grid_render_target: RenderTarget,
}

pub const GRID_VIEWPORT_SIZE: f32 = 1080.0;

pub const VIRTUAL_WIDTH: f32 = 1920.0;
pub const VIRTUAL_HEIGHT: f32 = 1080.0;

impl Context {
    pub async fn default() -> Self {
        let base_assets_path = determine_asset_path();
        let font = load_ttf_font(
            base_assets_path
                .join("fonts/editundo.ttf")
                .to_str()
                .unwrap(),
        )
        .await
        .unwrap();
        let settings = GameSettings::load();
        set_fullscreen(settings.fullscreen);

        // let render_target =

        Self {
            font: font,
            // font_dot: load_ttf_font(base_assets_path.join("fonts/edundot.ttf").to_str().unwrap())
            //     .await
            //     .unwrap(),
            font_line: load_ttf_font(
                base_assets_path
                    .join("fonts/edunline.ttf")
                    .to_str()
                    .unwrap(),
            )
            .await
            .unwrap(),
            // screen_size: Vec2::new(0.0, 0.0),
            switch_scene_to: None,
            request_quit: false,
            input: InputContext::new(),
            audio: audio::AudioAtlas::new(&base_assets_path, &settings).await,
            time: get_time(),
            settings,

            crt_material: load_material(
                ShaderSource::Glsl {
                    vertex: include_str!("../assets/shaders/crt.vert"),
                    fragment: include_str!("../assets/shaders/crt.frag"),
                },
                Default::default(),
            )
            .unwrap(),

            glow_material: load_material(
                ShaderSource::Glsl {
                    vertex: include_str!("../assets/shaders/crt.vert"),
                    fragment: include_str!("../assets/shaders/crt.frag"),
                },
                Default::default(),
            )
            .unwrap(),
            // TODO: Set filter mode if needed!
            render_target: render_target(VIRTUAL_WIDTH as u32, VIRTUAL_HEIGHT as u32),
            grid_render_target: render_target(GRID_VIEWPORT_SIZE as u32, GRID_VIEWPORT_SIZE as u32),
        }
    }

    pub fn save_settings(&mut self) {
        self.settings.save();
        set_fullscreen(self.settings.fullscreen);
        self.audio.settings(&self.settings);
    }

    pub fn update(&mut self) {
        self.input.update();
        // self.screen_size = Vec2::new(screen_width(), screen_height());
        self.time = get_time();
        self.audio.update();
    }
}
