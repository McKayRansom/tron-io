use super::{
    Occupied, Point,
    bike::{Bike, DOWN, LEFT, RIGHT, UP},
    msg::BikeUpdate,
};

pub const DIRS: &[Point] = &[UP, DOWN, LEFT, RIGHT];
pub const DIRS_REV: &[Point] = &[RIGHT, LEFT, DOWN, UP];

impl Bike {
    pub fn ai_update(
        &self,
        grid: &Occupied,
        rng: &macroquad::rand::RandGenerator,
    ) -> Option<BikeUpdate> {
        let new_head = (self.head.0 + self.dir.0, self.head.1 + self.dir.1);

        if grid.is_occupied(new_head) {
            let dirs = if rng.gen_range(0, 2) == 0 {
                DIRS
            } else {
                DIRS_REV
            };
            for dir in dirs {
                let new_head = (self.head.0 + dir.0, self.head.1 + dir.1);
                if !grid.is_occupied(new_head) {
                    if self.dir != *dir {
                        return Some(BikeUpdate {
                            id: self.id,
                            dir: *dir,
                        });
                    } else {
                        return None;
                    }
                }
            }
        }

        None
    }
}
