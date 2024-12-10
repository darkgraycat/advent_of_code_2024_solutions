#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
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
