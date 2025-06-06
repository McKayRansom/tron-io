use crate::{
    AiDifficulty,
    grid::{Grid, Point},
};

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
 *
 * WHAT WE NEED:
 * - Pathing grid with weights, right next to bike is negative weight, 2 tiles away is positive, etc...
 * So that we don't suicide constantly!
 */
// pub struct AiPathGrid {
//     costs: Vec<u8>,
// }

// impl AiPathGrid {
//     pub fn create(grid: &Grid) -> Self {

//     }
// }

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
                            id: self.id,
                            dir: *dir,
                            flags: 0,
                        });
                    } else {
                        return None;
                    }
                }
            }
        }

        if grid.ai_diff == AiDifficulty::Easy {
            return None;
        }

        // these are the desired poses, could be improved LOL
        let cutoff_pos: Vec<(i16, i16)> = grid
            .bikes
            .iter()
            .filter_map(|bike| {
                if bike.id == self.id || bike.team == self.team || bike.alive == false {
                    None
                } else {
                    // NOTE: Looking at bike's dir is kinda cheating and suboptimal anyways...
                    Some(add(bike.head, multiply(bike.dir, 2)))
                }
            })
            .collect();

        // Don't ever path in front of another bike (feels hacky, probably need a full grid pathing costs algo)
        let avoid_pos: Vec<(i16, i16)> = grid
            .bikes
            .iter()
            .filter_map(|bike| {
                if bike.id == self.id || bike.alive == false {
                    None
                } else {
                    Some(vec![
                        add(bike.head, LEFT),
                        add(bike.head, RIGHT),
                        add(bike.head, UP),
                        add(bike.head, DOWN),
                    ])
                }
            })
            .flatten()
            .collect();
        // .reduce(Vec::new(), |big_vec, elem| big_vec + elem)

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
                        .is_some_and(|cell| !cell.is_occupied() /*|| cell.is_bike() */)
                        && !avoid_pos.contains(pos)
                })
            },
            // |pos| pos != &self.head && grid.occupied.get_cell(*pos).unwrap().is_bike(),
            |pos| cutoff_pos.contains(pos),
        );

        if let Some(path) = path {
            if let Some(pos) = path.get(1) {
                let dir = (-self.head.0 + pos.0, -self.head.1 + pos.1);
                // TODO: This needs tweaking to not die constantly
                let boost = self.boost_time == 0
                    && path.len() < 15
                    && grid.rng.rand() < u32::MAX / 10
                    && self.boost_count > 0
                    && grid.ai_diff == AiDifficulty::Hard;
                if dir == self.dir && !boost {
                    return None;
                }
                // log::info!("Path from {:?} to {:?}", self.head, pos);
                return Some(BikeUpdate {
                    id: self.id,
                    dir,
                    // boost: false,
                    flags: if boost { FLAG_BOOST } else { 0 },
                });
            }
        }

        None
    }
}
