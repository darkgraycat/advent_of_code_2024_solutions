use std::{collections::HashSet, fmt::Display};

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

#[derive(Debug)]
struct Guard {
    position: (i32, i32),
    direction: Direction,
    visited: HashSet<(i32, i32)>,
}

impl Guard {
    fn new(position: (i32, i32), direction: Direction) -> Self {
        Guard {
            position,
            direction,
            visited: HashSet::new(),
        }
    }

    fn make_step(&mut self, map_grid: &MapGrid) -> bool {
        let (nx, ny) = self.get_next_step();

        if !map_grid.is_in_bounds((nx, ny)) {
            return false;
        }

        if map_grid.is_obstacle((nx, ny)) {
            self.rotate_right();
            let (nx, ny) = self.get_next_step();
            self.position = (nx, ny);
            self.visited.insert((nx, ny));
            return true;
        }
        
        self.visited.insert((nx, ny));
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

#[derive(Debug)]
struct MapGrid {
    grid: Vec<Vec<bool>>,
}

impl MapGrid {
    fn new(grid: Vec<Vec<bool>>) -> Self {
        MapGrid { grid }
    }

    fn is_in_bounds(&self, (x, y): (i32, i32)) -> bool {
        y >= 0 && y < self.grid.len() as i32 && x >= 0 && x < self.grid[0].len() as i32
    }

    fn is_obstacle(&self, (x, y): (i32, i32)) -> bool {
        self.grid
            .get(y as usize)
            .and_then(|row| row.get(x as usize))
            .map_or(false, |&cell| cell)
    }
}

pub fn task1(input: String) {
    let (grid, (y, x), direction) = parse(&input);

    let mut guard = Guard::new((x, y), direction.try_into().expect("No direction"));

    let map_grid = MapGrid::new(
        grid.into_iter()
            .map(|line| line.into_iter().map(|ch| ch == '#').collect())
            .collect(),
    );

    let mut steps = 0;
    loop {
        let can_move_forward = guard.make_step(&map_grid);
        steps += 1;
        if !can_move_forward {
            break;
        }
    }

    // println!("{map_grid:?}");
    // println!("{guard:?}");

    println!("Result {}", steps);
    println!("Uniq steps {}", guard.visited.len());
}

pub fn task2(input: String) {}

fn parse(input: &String) -> (Vec<Vec<char>>, (i32, i32), char) {
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
    (grid, (guard.0 as i32, guard.1 as i32), guard.2)
}
