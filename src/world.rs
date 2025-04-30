use nanoserde::{DeBin, SerBin};

use crate::grid::GridUpdateMsg;


pub mod client;
pub mod server;
pub mod online;
pub mod local;

const PLAYER_MAX: usize = 4;
const SCORE_WIN: u8 = 3;

#[derive(Default, DeBin, SerBin, Debug, Copy, Clone, PartialEq, Eq)]
pub enum WorldState {
    #[default]
    Waiting,
    Playing,
    RoundOver(u8),
    GameOver(u8),
}

#[derive(DeBin, SerBin, Debug, Clone)]
pub struct ServerMsg {
    pub id: u8,
    pub state: WorldState,
    pub grid_update: Option<GridUpdateMsg>,
}

#[derive(DeBin, SerBin, Debug, Clone)]
pub struct ClientMsg {
    pub ready: bool,
    pub state: WorldState,
    pub update: Option<GridUpdateMsg>,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    /// move up (player, menu, etc.)
    Up,
    /// move down (player, menu, etc.)
    Down,
    /// move left (player, menu, etc.)
    Left,
    /// move  right (player, menu, etc.)
    Right,
    /// select the menu option or prompt to continue
    Confirm,
    /// go back in the menu
    Cancel,
    /// reset the level to the starting positions
    Reset,
    /// go back a move
    Rewind,
    /// the gameplay and bring up a menu
    Pause,
}

pub trait ClientConnection {
    fn send(&mut self, msg: &ClientMsg);
    fn try_recv(&mut self) -> Option<ServerMsg>;
    fn update(&mut self);
}

