use std::io::{stdin, Read};

use super::{guard::Guard, map_grid::MapGrid};

pub struct PathTracker<'a> {
    map_grid: &'a MapGrid,
    label: String,
}

impl<'a> PathTracker<'a> {
    pub fn new(map_grid: &'a MapGrid, label: String) -> Self {
        Self { map_grid, label }
    }

    pub fn render(&self, guard: &Guard) -> &Self {
        let (x, y) = guard.position;
        let mut lines: Vec<char> = self
            .map_grid
            .to_string()
            .replace("\n", "")
            .chars()
            .collect();

        lines[self.map_grid.width * y as usize + x as usize] = guard.direction.into();

        let display = lines
            .chunks(self.map_grid.width)
            .map(|chunk| chunk.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("\n");

        println!("{}:\n{}", self.label, display);
        self
    }

    pub fn wait(&self) -> &Self {
        let mut buffer = [0; 1];
        stdin().read_exact(&mut buffer).unwrap();
        self
    }
}
