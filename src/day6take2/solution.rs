/*
* Completelly messed up here, as with previos solutions
* But finally know the reason:
* "You cannot place obstacle on visited cell"
* Thank you pal from the comments on Reddit:
* https://www.reddit.com/r/adventofcode/comments/1hb9odk/2024_day_6_part_2_one_extra_solution_but_it_seems/
*/

use std::{collections::HashSet, ops::Range};

////////////////////////////////////////////////////////////////////////////////
/// Direction
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

impl TryFrom<char> for Direction {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '^' => Ok(Direction::Up),
            'v' => Ok(Direction::Down),
            '<' => Ok(Direction::Left),
            '>' => Ok(Direction::Right),
            _ => Err("Cannot parse direction".to_owned()),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
/// Position
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn moved(&self, direction: Direction) -> Position {
        match direction {
            Direction::Up => Position::new(self.x, self.y - 1),
            Direction::Right => Position::new(self.x + 1, self.y),
            Direction::Down => Position::new(self.x, self.y + 1),
            Direction::Left => Position::new(self.x - 1, self.y),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
/// Guard
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Copy)]
struct Guard {
    position: Position,
    direction: Direction,
}

impl Guard {
    fn get_next(&self) -> Position {
        self.position.moved(self.direction)
    }
    fn turn(&mut self) {
        self.direction = self.direction.turn_right();
    }
    fn forward(&mut self) {
        self.position = self.get_next();
    }
}

////////////////////////////////////////////////////////////////////////////////
/// State
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
struct State {
    guard: Guard,
    blocks: HashSet<Position>,
    visited: HashSet<(Position, Direction)>,
    x_range: Range<usize>,
    y_range: Range<usize>,
}

impl State {
    fn is_looping(&self) -> bool {
        self.visited.contains(&(
            self.guard.position.moved(self.guard.direction),
            self.guard.direction,
        ))
    }

    fn make_step(&mut self) -> Option<Position> {
        let next_position = self.get_next_position()?;

        if self.blocks.contains(&next_position) {
            if !self.visited.insert((next_position, self.guard.direction)) {
                return None;
            }
            self.guard.turn();
        } else {
            self.guard.forward();
        }

        Some(self.guard.position)
    }

    fn get_next_position(&self) -> Option<Position> {
        let next = self.guard.get_next();
        (self.x_range.contains(&next.x) && self.y_range.contains(&next.y)).then_some(next)
    }

    fn with_block(&self, position: Position) -> State {
        let mut cloned = self.clone();
        cloned.blocks.insert(position);
        cloned
    }
}

impl TryFrom<String> for State {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let guard_idx = value
            .find(|c| "<>v^".contains(c))
            .ok_or("No Guard found!")?;
        let scanline = value.lines().next().ok_or("Grid cannot be oneline")?.len() + 1;

        let guard = Guard {
            position: Position::new(guard_idx % scanline, guard_idx / scanline),
            direction: value
                .chars()
                .nth(guard_idx)
                .ok_or("Cannot get direction")?
                .try_into()?,
        };

        let blocks: HashSet<Position> = value
            .match_indices('#')
            .map(|(index, _)| Position {
                x: index % scanline,
                y: index / scanline,
            })
            .collect();

        Ok(State {
            guard,
            blocks,
            visited: HashSet::new(),
            x_range: 0..value.lines().last().unwrap().len(),
            y_range: 0..value.lines().count(),
        })
    }
}

////////////////////////////////////////////////////////////////////////////////
/// Solution
////////////////////////////////////////////////////////////////////////////////

pub fn task2(input: String) {
    let mut state = State::try_from(input).expect("Pew pew");
    let mut loops = 0;

    let mut tries: HashSet<Position> = HashSet::new();

    /* main loop */
    while let Some(next_position) = state.get_next_position() {
        if tries.contains(&next_position) {
            state.make_step();
            continue;
        }

        let mut simulation = state.with_block(next_position);
        tries.insert(next_position);

        /* simulation loop */
        while let Some(_) = simulation.make_step() {}
        if simulation.is_looping() {
            loops += 1;
        }

        state.make_step();
    }

    println!("Result {}", loops);
}
