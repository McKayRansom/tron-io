use super::{Grid, Point, UpdateResult};

use nanoserde::{DeBin, SerBin};


#[derive(DeBin, SerBin, Debug, Clone, Copy)]
pub struct BikeUpdate {
    pub id: u8,
    pub dir: Point,
}

impl BikeUpdate {
    pub fn new(id: u8, dir: Point) -> Self {
        Self { id, dir }
    }
}

#[derive(DeBin, SerBin, Debug, Clone, Default)]
pub struct GridUpdateMsg {
    pub tick: u32,
    pub hash: u64,
    pub updates: Vec<BikeUpdate>,
}

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

impl Grid {
    pub fn apply_update(&mut self, update: &BikeUpdate) {
        // Apply the update to the grid
        let bike = self.bikes.get_mut(update.id as usize).unwrap();
        bike.dir = update.dir;
        // bike.update(&mut self.occupied, false);
    }

    pub fn apply_updates(&mut self, updates: &GridUpdateMsg) -> UpdateResult {
        // tick and seed?
        for update in updates.updates.iter() {
            self.apply_update(update);
        }
        if self.tick != updates.tick {
            self.tick = updates.tick;
            self.update()
        } else {
            UpdateResult::InProgress
        }

    }
}


#[cfg(test)]
mod tests {
    use crate::grid::{bike::{DOWN, RIGHT}, Grid};

    use super::*;

    #[test]
    fn test_grid_update() {
        let mut grid = Grid::new();
        assert_eq!(grid.bikes[0].dir, RIGHT);

        let update = BikeUpdate {
            id: 1,
            dir: DOWN,
        };
        let msg = GridUpdateMsg {
            tick: 1,
            hash: 1234,
            updates: vec![update],
        };
        grid.apply_updates(&msg);
        // assert_eq!(msg.tick, 1);
        // assert_eq!(msg.seed, 1234);
        assert_eq!(grid.bikes[0].dir, DOWN);
    }
}
