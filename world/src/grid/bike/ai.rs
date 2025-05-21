use crate::grid::{Grid, Point};

use super::*;

pub const DIRS: &[Point] = &[UP, DOWN, LEFT, RIGHT];
pub const DIRS_REV: &[Point] = &[RIGHT, LEFT, DOWN, UP];

/*
 * AI
 * Mode cutoff: Pathfind to enemy bikes!
 * Mode survive: Turn when we reach wall
 * 
 * This is much better, but needs tweaking to not suicide so much...
 * 
 * Ideas:
 * - Instead of pathing to position, path to predicted pos (this is harder to calc than I thought)
 * - Randomize X vs Y in successsors to shake things up
 * - random seed to shake things up
 */
impl Bike {
    pub fn ai_update(&self, grid: &Grid) -> Option<BikeUpdate> {
        // let path_grid: Vec<Vec<f32>> = Vec::ne(&self, other);
        if !self.alive {
            return None;
        }

        let new_head = (self.head.0 + self.dir.0, self.head.1 + self.dir.1);
        // OLD: miss wall logic, works find still when we can't reach enemies
        if grid.occupied.is_occupied(new_head) {
            let dirs = if grid.rng.gen_range(0, 2) == 0 {
                DIRS
            } else {
                DIRS_REV
            };
            for dir in dirs {
                let new_head = (self.head.0 + dir.0, self.head.1 + dir.1);
                if !grid.occupied.is_occupied(new_head) {
                    if self.dir != *dir {
                        return Some(BikeUpdate {
                            boost: false,
                            id: self.id,
                            dir: *dir,
                        });
                    } else {
                        return None;
                    }
                }
            }
        }
        let cutoff_pos: Vec<(i16, i16)> = grid
            .bikes
            .iter()
            .filter_map(|bike| {
                if bike.id == self.id || bike.team == self.team {
                    None
                } else {
                    Some((bike.head.0 + bike.dir.0, bike.head.1 + bike.dir.1))
                }
            })
            .collect();

        let path = pathfinding::prelude::bfs(
            &self.head,
            |pos| {
                [
                    (pos.0 + 1, pos.1),
                    (pos.0 - 1, pos.1),
                    (pos.0, pos.1 + 1),
                    (pos.0, pos.1 - 1),
                ]
                .into_iter()
                .filter(|pos| {
                    grid.occupied
                        .get_cell(*pos)
                        .is_some_and(|cell| !cell.is_occupied() || cell.is_bike())
                })
            },
            // |pos| pos != &self.head && grid.occupied.get_cell(*pos).unwrap().is_bike(),
            |pos| cutoff_pos.contains(pos),
        );

        if let Some(path) = path {
            if let Some(pos) = path.get(1) {
                let dir = (-self.head.0 + pos.0, -self.head.1 + pos.1);
                if dir == self.dir {
                    return None;
                }
                // log::info!("Path from {:?} to {:?}", self.head, pos);
                return Some(BikeUpdate {
                    id: self.id,
                    dir,
                    boost: false,
                    // TODO: This needs tweaking to not die constantly
                    // boost: self.boost_time == 0 && path.len() < 10, // && grid.rng.rand() < u32::MAX / 10,
                });
            }
        } 

        None
    }
}
