use macroquad::{
    math::Vec2,
    text::{load_ttf_font, Font}, window::{screen_height, screen_width},
};

pub struct Context {
    pub font: Font,
    #[allow(dead_code)]
    pub screen_size: Vec2,
}

impl Context {
    pub async fn default() -> Self {
        Self {
            font: load_ttf_font("resources/editundo.ttf").await.unwrap(),
            screen_size: Vec2::new(0.0, 0.0),
        }
    }

    #[allow(dead_code)]
    pub fn update(&mut self) {
        self.screen_size = Vec2::new(screen_width(), screen_height());
    }
}
