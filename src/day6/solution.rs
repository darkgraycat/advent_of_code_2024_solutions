use std::time::Instant;

use crate::day6::{guard::Guard, map_grid::MapGrid};

pub fn task1(input: String) {
    let (grid, (y, x), direction) = parse(&input);

    let mut guard = Guard::new((x, y), direction.try_into().expect("No direction"));

    let map_grid = MapGrid::new(
        grid.into_iter()
            .map(|line| line.into_iter().map(|ch| ch == '#').collect())
            .collect(),
    );

    loop {
        let next_step = guard.get_next_step();

        if !map_grid.is_in_bounds(next_step) {
            break;
        }

        if map_grid.is_obstacle(next_step) {
            guard.rotate_right();
        }

        guard.make_step();
    }

    println!("MapGrid\n{}", map_grid);
    println!("Uniq steps {}", guard.visited.len());
}

enum SimulationResults {
    InLoop,
    OutOfBounds,
}

fn simulate(guard: &mut Guard, map_grid: &MapGrid) -> SimulationResults {
    // println!("Simulate for {:?} - {:?}", guard.position, guard.direction);
    loop {
        while map_grid.is_obstacle(guard.get_next_step()) {
            guard.rotate_right()
        }

        if !map_grid.is_in_bounds(guard.get_next_step()) {
            return SimulationResults::OutOfBounds;
        }

        if guard.is_in_loop() {
            return SimulationResults::InLoop;
        }

        guard.make_step();
        // println!("Made step to {:?} - {:?}", guard.position, guard.direction);
    }
}

pub fn task2(input: String) {
    let (grid, (y, x), direction) = parse(&input);

    let mut guard = Guard::new((x, y), direction.try_into().expect("No direction"));

    let map_grid = MapGrid::new(
        grid.into_iter()
            .map(|line| line.into_iter().map(|ch| ch == '#').collect())
            .collect(),
    );

    let mut counter = 0;
    let mut max_iterations = 100_000;

    let start_time = Instant::now();

    loop {
        while map_grid.is_obstacle(guard.get_next_step()) {
            guard.rotate_right()
        }

        if !map_grid.is_in_bounds(guard.get_next_step()) {
            break;
        }

        guard.make_step();

        let mut guard_sim = guard.clone();
        guard_sim.rotate_right();

        if let SimulationResults::InLoop = simulate(&mut guard_sim, &map_grid) {
            // println!("LOOP");
            counter += 1;
        }

        max_iterations -= 1;
        if max_iterations <= 0 {
            break;
        }
    }

    // old/incorrect solutions - 1358, 1413, 1412
    println!("Time elapsed {:?}", start_time.elapsed());
    println!("Iterations {}", 100_000 - max_iterations);
    println!("Counter {}", counter);
    println!("Uniq steps {}", guard.visited.len());
}

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
