use nanoserde::{DeBin, SerBin};

pub mod grid;
use grid::GridUpdateMsg;
pub mod client;
pub mod server;
// pub mod online;
pub mod local;

// const PLAYER_MAX: usize = 4;
const SCORE_WIN: u8 = 3;

#[derive(Default, DeBin, SerBin, Debug, Copy, Clone, PartialEq, Eq)]
pub enum WorldState {
    #[default]
    Waiting,
    Playing,
    RoundOver(Option<u8>),
    GameOver(u8),
}


#[derive(DeBin, SerBin, Debug, Clone)]
pub struct ClientPlayer {
    pub name: String,
    // optimization: bitpack
    pub ready: bool,
    pub team_request: u8,
}

#[derive(DeBin, SerBin, Debug, Clone)]
pub struct ServerPlayer {
    // pub score: u8,
    pub name: String,
    pub ready: bool,
    pub is_ai: bool,
    // pub team: u8, (This is determined by the player ID for now)
}

#[derive(DeBin, SerBin, Debug, Clone)]
pub struct ServerMsg {
    // future optimization: Send hash instead
    // pub connection_id: u8,
    pub local_player_ids: Vec<u8>,
    pub players: Vec<ServerPlayer>,
    pub state: WorldState,
    pub grid_update: Option<GridUpdateMsg>,
    pub options: Option<GridOptions>,
    pub score: Vec<u8>,
}

#[derive(DeBin, SerBin, Debug, Clone)]
pub struct ClientMsg {
    // pub connection_id: Option<u8>,
    pub players: Vec<ClientPlayer>,
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
    fn update(&mut self, time: f64);
}

#[derive(Debug, Default, Clone, Copy, DeBin, SerBin, PartialEq, Eq)]
pub enum GridSize {
    #[default]
    Small,
    Medium,
    Large,
}

impl GridSize {
    pub fn incr(&mut self)  {
        *self = match self {
            GridSize::Small => GridSize::Medium,
            GridSize::Medium => GridSize::Large,
            GridSize::Large => GridSize::Large,
        }
    }
    pub fn decr(&mut self) {
        *self = match self {
            GridSize::Small => GridSize::Small,
            GridSize::Medium => GridSize::Medium,
            GridSize::Large => GridSize::Large,
        }
    }
    pub fn dim(&self) -> (i16, i16) {
        match self {
            GridSize::Small => (80, 80),
            GridSize::Medium => (100, 100),
            GridSize::Large => (120, 120),
        }

    }
}

pub const MIN_TEAMS: u8 = 2;
pub const MAX_TEAMS: u8 = 4;

pub const MIN_PLAYERS: u8 = 1;
pub const MAX_PLAYERS: u8 = 4;

#[derive(Debug, Clone, Copy, DeBin, SerBin, PartialEq, Eq)]
pub struct GridOptions {
    pub grid_size: GridSize,
    pub teams: u8,
    pub players: u8,
    // teams
    // boost? other powerups?
}

impl Default for GridOptions {
    fn default() -> Self {
        Self {
            grid_size: Default::default(),
            teams: MIN_TEAMS,
            players: MIN_PLAYERS,
        }
    }
}

