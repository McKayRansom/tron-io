
use tron_io_world::GridOptions;

use crate::context::Context;

// pub mod credits;
// pub mod gameplay;
pub mod main_menu;
// pub mod pause;
// pub mod settings;


pub enum EScene {
    Gameplay(GridOptions),
    MainMenu,
}

pub trait Scene {
    fn update(&mut self, ctx: &mut Context);
    fn draw(&mut self, ctx: &mut Context);
}
