use std::{collections::HashSet, fmt::Display, ops::Range};

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

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn move_to(&self, direction: Direction) -> Position {
        match direction {
            Direction::Up => Position::new(self.x, self.y - 1),
            Direction::Right => Position::new(self.x + 1, self.y),
            Direction::Down => Position::new(self.x, self.y + 1),
            Direction::Left => Position::new(self.x - 1, self.y),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Guard {
    position: Position,
    direction: Direction,
}

impl Guard {
    fn get_next(&self) -> Position {
        self.position.move_to(self.direction)
    }
    fn turn(&mut self) {
        self.direction = self.direction.turn_right();
    }
    fn forward(&mut self) {
        self.position = self.get_next();
    }
}

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
        self.visited
            .contains(&(self.guard.position, self.guard.direction))
    }

    fn make_step(&mut self) -> Option<Position> {
        let next_position = self.get_next_position()?;

        self.visited
            .insert((self.guard.position, self.guard.direction));

        if self.blocks.contains(&next_position) {
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
        cloned.visited.clear();
        cloned
    }
}

impl TryFrom<String> for State {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let blocks: HashSet<Position> = value
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter(|&(_, ch)| ch == '#')
                    .map(move |(x, _)| Position::new(x, y))
            })
            .collect();

        let guard: Guard = value
            .lines()
            .enumerate()
            .find_map(|(y, line)| {
                line.chars().position(|c| c == '^').map(|x| Guard {
                    position: Position::new(x, y),
                    direction: Direction::Up,
                })
            })
            .ok_or("Guard not found")?;

        Ok(State {
            guard,
            blocks,
            visited: HashSet::new(),
            x_range: 0..value.lines().last().unwrap().len(),
            y_range: 0..value.lines().count(),
        })
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in self.y_range.clone() {
            for x in self.x_range.clone() {
                let pos = Position { x, y };

                // Check if the guard is at this position
                if self.guard.position == pos {
                    let guard_char = match self.guard.direction {
                        Direction::Up => '^',
                        Direction::Down => 'v',
                        Direction::Left => '<',
                        Direction::Right => '>',
                    };
                    write!(f, "{}", guard_char)?;
                } else if self.blocks.contains(&pos) {
                    write!(f, "#")?;
                } else if let Some((_, _)) = self.visited.get(&(pos, Direction::Up)) {
                    write!(f, "|")?;
                } else if let Some((_, _)) = self.visited.get(&(pos, Direction::Down)) {
                    write!(f, "|")?;
                } else if let Some((_, _)) = self.visited.get(&(pos, Direction::Left)) {
                    write!(f, "-")?;
                } else if let Some((_, _)) = self.visited.get(&(pos, Direction::Right)) {
                    write!(f, "-")?;
                } else {
                    // Empty space
                    write!(f, ".")?;
                }
            }
            writeln!(f)?; // Newline after each row
        }
        Ok(())
    }
}

pub fn wait_input() {
    let mut buffer = [0; 1];
    std::io::Read::read_exact(&mut std::io::stdin(), &mut buffer).unwrap();
}

pub fn task2(input: String) {
    let mut state = State::try_from(input).expect("Pew pew");
    let mut loops = 0;

    /* main loop */
    loop {
        println!("!MAIN\n{}", state);
        // wait_input();

        if let Some(next_position) = state.get_next_position() {
            let mut simulation = state.with_block(next_position);

            state.make_step();

            while let Some(_) = simulation.make_step() {
                println!("!SIMULATION\n{}", simulation);
                // wait_input();
                if simulation.is_looping() {
                    loops += 1;
                    println!("!LOOP {}\n", loops);
                    break;
                }
            }
        } else {
            break;
        }
    }

    println!("Result {}", loops);

    // println!("Initial state {:?}", state);
}
