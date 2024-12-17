use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    ops::Range,
};

use crate::Position;

pub fn task1(input: String) {
    let vv = vec![Position::new(10, 5), Position::new(8, 5), Position::new(1, 4)];

    let pos = Position::new(1, 4);
    let contains = vv.contains(&pos);

    println!("Contains: {}", contains);

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
    antinodes: HashSet<Position>,
    x_range: Range<usize>,
    y_range: Range<usize>,
}

impl AerialsNetwork {
    fn collect_antinodes(&mut self) {
        self.antinodes.clear();
        for (kind, aerials) in self.aerials.iter() {
            println!("{} ----------------", kind);
            for (i, pos_a) in aerials.iter().enumerate() {
                for pos_b in aerials.iter().skip(i + 1) {
                    println!("Comparing {:?} with {:?}", pos_a, pos_b);
                    let (dx, dy) = pos_a.delta(pos_b);

                    let node_a = (pos_a.x as i32 + dx, pos_a.y as i32 + dy);
                    let node_b = (pos_b.x as i32 - dx, pos_b.y as i32 - dy);

                    if self.in_range(node_a) {
                        let node_a = Position { x: node_a.0 as usize, y: node_a.1 as usize };
                        // println!("IN range A {:?}", node_a);
                        if self.get_aerial_kind(&node_a).map_or(true, |k| *kind != k) {
                            self.antinodes.insert(node_a);
                        }
                    }
                    if self.in_range(node_b) {
                        // println!("IN range B {:?}", node_b);
                        let node_b = Position { x: node_b.0 as usize, y: node_b.1 as usize };
                        if self.get_aerial_kind(&node_b).map_or(true, |k| *kind != k) {
                            self.antinodes.insert(node_b);
                        }
                    }
                }
            }
        }
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

        for (kind, aerials) in self.aerials.iter() {
            writeln!(f, "Aerials: \t{} {:?}", kind, aerials)?;
            for Position { x, y } in aerials {
                grid[max_x * y + x] = *kind;
            }
        }
        for antinode in self.antinodes.iter() {
            grid[max_x * antinode.y + antinode.x] = '#';
            writeln!(f, "Antinode:\t{:?}", antinode)?;
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
