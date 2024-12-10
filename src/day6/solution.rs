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

        println!("Next step to {:?}", next_step);

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

pub fn task2(input: String) {
    let (grid, (y, x), direction) = parse(&input);

    let mut guard = Guard::new((x, y), direction.try_into().expect("No direction"));

    let map_grid = MapGrid::new(
        grid.into_iter()
            .map(|line| line.into_iter().map(|ch| ch == '#').collect())
            .collect(),
    );

    loop {
        let next_step = guard.get_next_step();

        println!("Next step to {:?}", next_step);

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
