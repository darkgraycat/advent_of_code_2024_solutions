use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Direction {
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

pub fn task1(input: String) {
    let (grid, (y, x), direction) = parse(&input);

    let direction: Direction = direction.try_into().expect("No direction");

    println!("{grid:?}\n{x}-{y}\n{direction:?}");
}

pub fn task2(input: String) {}

fn parse(input: &String) -> (Vec<Vec<char>>, (usize, usize), char) {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let guard = grid
        .iter()
        .enumerate()
        .find_map(|(row, line)| {
            line.iter()
                .position(|&c| c == '^' || c == 'v' || c == '>' || c == '<')
                .map(|col| (row, col, line[col]))
        })
        .expect("No guard found");
    (grid, (guard.0, guard.1), guard.2)
}
