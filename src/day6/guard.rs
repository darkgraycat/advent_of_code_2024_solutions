use std::collections::HashSet;

use super::direction::Direction;

#[derive(Debug)]
pub struct Guard {
    pub position: (i32, i32),
    pub direction: Direction,
    pub visited: HashSet<((i32, i32), Direction)>,
}

impl Guard {
    pub fn new(position: (i32, i32), direction: Direction) -> Self {
        Guard {
            position,
            direction,
            visited: HashSet::new(),
        }
    }

    pub fn make_step(&mut self) {
        let next_step = self.get_next_step();

        self.visited.insert((self.position, self.direction));
        self.position = next_step;
    }

    pub fn get_next_step(&self) -> (i32, i32) {
        let (x, y) = self.position;
        let (dx, dy) = self.direction.get_coords();
        (x + dx, y + dy)
    }

    pub fn is_in_loop(&self) -> bool {
        self.visited.contains(&(self.position, self.direction))
    }

    pub fn rotate_right(&mut self) {
        self.direction = self.direction.get_rotated_right();
    }
}
