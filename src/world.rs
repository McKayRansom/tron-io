use crate::grid::msg::{ClientMsg, ServerMsg};


pub mod client;
pub mod server;
pub mod online;
pub mod local;

const PLAYER_MAX: usize = 4;
const SCORE_WIN: u8 = 3;


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
