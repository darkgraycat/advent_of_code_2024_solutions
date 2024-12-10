use std::collections::HashSet;

use super::{direction::Direction, map_grid::MapGrid};

#[derive(Debug)]
pub struct Guard {
    position: (i32, i32),
    direction: Direction,
    pub visited: HashSet<(i32, i32, Direction)>,
}

impl Guard {
    pub fn new(position: (i32, i32), direction: Direction) -> Self {
        Guard {
            position,
            direction,
            visited: HashSet::new(),
        }
    }

    pub fn make_step(&mut self, map_grid: &MapGrid) -> bool {
        let (nx, ny) = self.get_next_step();

        if !map_grid.is_in_bounds((nx, ny)) {
            return false;
        }

        if map_grid.is_obstacle((nx, ny)) {
            self.rotate_right();
            let (nx, ny) = self.get_next_step();
            self.position = (nx, ny);
            self.visited.insert((nx, ny, self.direction));
            return true;
        }

        self.visited.insert((nx, ny, self.direction));
        self.position = (nx, ny);
        true
    }

    fn get_next_step(&self) -> (i32, i32) {
        let (x, y) = self.position;
        let (dx, dy) = match self.direction {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };
        (x + dx, y + dy)
    }

    fn rotate_right(&mut self) {
        match self.direction {
            Direction::Up => self.direction = Direction::Right,
            Direction::Right => self.direction = Direction::Down,
            Direction::Down => self.direction = Direction::Left,
            Direction::Left => self.direction = Direction::Up,
        };
    }
}
