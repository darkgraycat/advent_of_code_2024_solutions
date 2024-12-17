use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    ops::Range,
};

use crate::Position;

pub fn task1(input: String) {
    let network = AerialsNetwork::try_from(input).expect("Oops");

    println!("DEBUG PRINT:");
    println!("{:?}", network);

    println!("\n\nDISPLAY PRINT:");
    println!("{}", network);
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
    fn collect_antinodes(&mut self) {}
}

impl Display for AerialsNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Network {:?} x {:?}", self.x_range, self.y_range)?;
        for a in &self.aerials {
            writeln!(f, "{:?}", a)?
        }
        for n in &self.antinodes {
            writeln!(f, "{:?}", n)?
        }
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
