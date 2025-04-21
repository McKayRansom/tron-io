use macroquad::text::{load_ttf_font, Font};


pub struct Context {
    pub font: Font,
}

impl Context {
    pub async fn default() -> Self {
        Self {
            font: load_ttf_font("resources/editundo.ttf").await.unwrap()
        }
    }
}
