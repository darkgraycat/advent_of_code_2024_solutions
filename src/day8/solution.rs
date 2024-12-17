use std::{collections::HashSet, fmt::Display, ops::Range};

use crate::Position;

pub fn task1(input: String) {
    let network = Network::try_from(input).expect("Oops");

    println!("DEBUG PRINT:");
    println!("{:?}", network);

    println!("\n\nDISPLAY PRINT:");
    println!("{}", network);
}

pub fn task2(input: String) {
    todo!()
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct Aerial {
    pos: Position,
    kind: char,
}

impl Aerial {
    fn new(kind: char, pos: Position) -> Self {
        Self { kind, pos }
    }
}

#[derive(Debug)]
struct Network {
    aerials: HashSet<Aerial>,
    x_range: Range<usize>,
    y_range: Range<usize>,
}

impl Display for Network {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Network {:?} x {:?}", self.x_range, self.y_range)?;
        for a in &self.aerials {
            writeln!(f, "{:?}", a)?
        }
        Ok(())
    }
}

impl TryFrom<String> for Network {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let scanline = value.lines().last().ok_or("Cannot parse scanline")?.len() + 1;
        let aerials: HashSet<Aerial> = value
            .char_indices()
            .filter(|(_, c)| !".\n".contains(*c))
            .map(|(idx, c)| Aerial::new(c, Position::new(idx % scanline, idx / scanline)))
            .collect();

        Ok(Network {
            aerials,
            x_range: 0..value.lines().last().unwrap().len(),
            y_range: 0..value.lines().count(),
        })
    }
}
