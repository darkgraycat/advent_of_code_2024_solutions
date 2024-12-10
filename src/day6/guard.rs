use std::collections::HashSet;

use super::direction::Direction;

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

    pub fn make_step(&mut self) {
        let (nx, ny) = self.get_next_step();

        println!("Going to {}, {}", nx, ny);

        self.visited.insert((nx, ny, self.direction));
        self.position = (nx, ny);
    }

    pub fn get_next_step(&self) -> (i32, i32) {
        let (x, y) = self.position;
        let (dx, dy) = self.direction.get_coords();
        (x + dx, y + dy)
    }

    pub fn rotate_right(&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}
