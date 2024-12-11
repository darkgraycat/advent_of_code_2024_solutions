#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn get_coords(&self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }

    pub fn get_rotated_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Left => Direction::Up,
            Direction::Down => Direction::Left,
        }
    }
}

impl TryFrom<char> for Direction {
    type Error = String;

    fn try_from(ch: char) -> Result<Self, Self::Error> {
        match ch {
            '^' => Ok(Direction::Up),
            'v' => Ok(Direction::Down),
            '<' => Ok(Direction::Left),
            '>' => Ok(Direction::Right),
            _ => Err(format!("Cannot parse {}", ch)),
        }
    }
}
