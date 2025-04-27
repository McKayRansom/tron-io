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

#[derive(DeBin, SerBin, Debug, Clone)]
pub struct GridUpdateMsg {
    pub tick: u32,
    pub seed: u32,
    pub updates: Vec<BikeUpdate>,
}

#[derive(DeBin, SerBin, Debug, Clone)]
pub struct JoinResponse {
    pub id: u8,
    pub seed: u32,
}


#[derive(DeBin, SerBin, Debug, Clone)]
pub enum ServerMsg {
    Update(GridUpdateMsg),
    JoinResponse(JoinResponse),
}

#[derive(DeBin, SerBin, Debug, Clone)]
pub enum ClientMsg {
    Update(GridUpdateMsg),
    Join,
}

impl Grid {
    pub fn apply_update(&mut self, update: &BikeUpdate) {
        // Apply the update to the grid
        let bike = self.bikes.get_mut(update.id as usize - 1).unwrap();
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
            seed: 1234,
            updates: vec![update],
        };
        grid.apply_updates(&msg);
        // assert_eq!(msg.tick, 1);
        // assert_eq!(msg.seed, 1234);
        assert_eq!(grid.bikes[0].dir, DOWN);
    }
}
