use super::Point;


pub struct GridUpdate {
    pub id: u8,
    pub head: Point,
    pub dir: Point,
}

pub struct GridUpdateMsg {
    pub tick: u32,
    pub seed: u32,
    pub updates: Vec<GridUpdate>,
}