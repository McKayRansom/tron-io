#[derive(Debug, Default)]
pub enum GridSize {
    #[default]
    Small,
    Medium,
    Large,
}

pub struct GameOptions {
    grid_size: GridSize,
    players: u8,
    // teams
    // boost? other powerups?
}

impl Default for GameOptions {
    fn default() -> Self {
        Self {
            grid_size: Default::default(),
            players: 2,
        }
    }
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
