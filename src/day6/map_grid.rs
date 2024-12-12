use std::fmt::Display;

#[derive(Debug)]
pub struct MapGrid {
    grid: Vec<Vec<bool>>,
}

impl MapGrid {
    pub fn new(grid: Vec<Vec<bool>>) -> Self {
        MapGrid { grid }
    }

    pub fn is_in_bounds(&self, (x, y): (i32, i32)) -> bool {
        y >= 0 && y < self.grid.len() as i32 && x >= 0 && x < self.grid[0].len() as i32
    }

    pub fn is_obstacle(&self, (x, y): (i32, i32)) -> bool {
        self.grid
            .get(y as usize)
            .and_then(|row| row.get(x as usize))
            .map_or(false, |&cell| cell)
    }
}

impl Display for MapGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.grid {
            for &cell in row {
                write!(f, "{}", if cell {'#'} else {'.'})?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
