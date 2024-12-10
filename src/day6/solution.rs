use crate::day6::{guard::Guard, map_grid::MapGrid};

pub fn task1(input: String) {
    let (grid, (y, x), direction) = parse(&input);

    let mut guard = Guard::new((x, y), direction.try_into().expect("No direction"));

    let mut map_grid = MapGrid::new(
        grid.into_iter()
            .map(|line| line.into_iter().map(|ch| ch == '#').collect())
            .collect(),
    );

    let mut steps = 0;
    loop {
        let can_move_forward = guard.make_step(&map_grid);
        steps += 1;
        if !can_move_forward {
            break;
        }
    }

    // println!("{map_grid:?}");
    // println!("{guard:?}");

    println!("MapGrid\n{}", map_grid);
    map_grid.place_obstacle((0, 0));
    map_grid.place_obstacle((1, 0));
    map_grid.place_obstacle((0, 1));
    map_grid.place_obstacle((1, 1));
    println!("MapGrid\n{}", map_grid);
    println!("Uniq steps {}", guard.visited.len());
}

pub fn task2(input: String) {}

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
