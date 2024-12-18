use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    ops::Range,
};

use crate::Position;

pub fn task1(input: String) {
    let mut network = AerialsNetwork::try_from(input).expect("Oops");

    network.collect_antinodes();

    println!("\n\nDISPLAY PRINT:");
    println!("{}", network);

    println!("Total antinodes {}", network.antinodes.len());
}

pub fn task2(input: String) {
    todo!()
}

#[derive(Debug)]
struct AerialsNetwork {
    aerials: HashMap<char, Vec<Position>>,
    antinodes: HashSet<(char, Position)>,
    x_range: Range<usize>,
    y_range: Range<usize>,
}

impl AerialsNetwork {
    fn collect_antinodes(&mut self) {
        self.antinodes.clear();
        for (kind, aerials) in self.aerials.iter() {
            println!("{} ----------------", kind);
            for (i, aerial_a) in aerials.iter().enumerate() {
                for aerial_b in aerials.iter().skip(i + 1) {
                    println!("Comparing {:?} with {:?}", aerial_a, aerial_b);

                    for pos in self.calc_antinode_pair(*kind, aerial_a, aerial_b) {
                        self.antinodes.insert((*kind, pos));
                    }
                }
            }
        }
    }

    fn calc_antinode_pair(&self, kind: char, aerial_a: &Position, aerial_b: &Position) -> Vec<Position> {
        let mut antinode_positions = Vec::new();
        let (mut dx, mut dy) = aerial_a.delta(aerial_b);

        let mut is_straight = false;

        if dx == 0 || dy == 0 || dx.abs() == dy.abs() {
            dx = dx.signum().abs();
            dy = dy.signum().abs();
            is_straight = true;
        }

        let mut pos_a = (aerial_a.x as i32, aerial_a.y as i32);
        let mut pos_b = (aerial_b.x as i32, aerial_b.y as i32);

        loop {
            pos_a = (pos_a.0 + dx, pos_a.1 + dy);
            pos_b = (pos_b.0 - dx, pos_b.1 - dy);

            if self.in_range(pos_a) {
                let pos = Position { x: pos_a.0 as usize, y: pos_a.1 as usize };
                if is_straight || self.get_aerial_kind(&pos).map_or(true, |k| kind != k) {
                    antinode_positions.push(pos);
                }
            }

            if self.in_range(pos_b) {
                let pos = Position { x: pos_b.0 as usize, y: pos_b.1 as usize };
                if is_straight || self.get_aerial_kind(&pos).map_or(true, |k| kind != k) {
                    antinode_positions.push(pos);
                }
            }

            if !self.in_range(pos_a) && !self.in_range(pos_b) {
                break;
            }
        }

        antinode_positions
    }

    fn in_range(&self, (x, y): (i32, i32)) -> bool {
        if x < 0 || y < 0 {
            return false;
        }
        self.x_range.contains(&(x as usize)) && self.y_range.contains(&(y as usize))
    }

    fn get_aerial_kind(&self, position: &Position) -> Option<char> {
        self.aerials
            .iter()
            .find_map(|(kind, aerials)| aerials.contains(position).then(|| *kind))
    }
}

impl Display for AerialsNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (max_x, max_y) = (self.x_range.end, self.y_range.end);
        let mut grid: Vec<char> = ".".repeat(max_x * max_y).chars().collect();

        writeln!(f, "Network:\t{} x {}", max_x, max_y)?;

        for antinode in self.antinodes.iter() {
            let (_, pos) = antinode;
            grid[max_x * pos.y + pos.x] = '#';
            writeln!(f, "Antinode:\t{:?}", antinode)?;
        }

        for (kind, aerials) in self.aerials.iter() {
            writeln!(f, "Aerials: \t{} {:?}", kind, aerials)?;
            for Position { x, y } in aerials {
                grid[max_x * y + x] = *kind;
            }
        }

        let display = grid
            .chunks(max_x)
            .map(|c| c.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("\n");

        writeln!(f, "Grid:\n{}", display)?;

        Ok(())
    }
}

impl TryFrom<String> for AerialsNetwork {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let scanline = value.lines().last().ok_or("Cannot parse scanline")?.len() + 1;
        let mut aerials: HashMap<char, Vec<Position>> = HashMap::new();
        value.char_indices().filter(|(_, c)| !".\n".contains(*c)).for_each(|(idx, c)| {
            aerials
                .entry(c)
                .or_insert(Vec::new())
                .push(Position::new(idx % scanline, idx / scanline))
        });
        Ok(AerialsNetwork {
            aerials,
            antinodes: HashSet::new(),
            x_range: 0..value.lines().last().unwrap().len(),
            y_range: 0..value.lines().count(),
        })
    }
}
