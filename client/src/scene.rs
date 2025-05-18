
pub struct GameOptions {
    // pub client: WorldClient,
    // bool 
}

pub enum EScene {
    Gameplay(GameOptions),
    MainMenu,
}

// use tron_io_world::client::WorldClient;

use crate::context::Context;

// pub mod credits;
// pub mod gameplay;
pub mod main_menu;
// pub mod pause;
// pub mod settings;

pub trait Scene {
    fn update(&mut self, ctx: &mut Context);
    fn draw(&mut self, ctx: &mut Context);
}
