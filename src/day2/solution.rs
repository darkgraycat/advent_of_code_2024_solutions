pub fn task1(input: String) {
    let levels = parse(input);

    let max_safe_difference = 3;
    let mut num_of_safe_reports: u32 = 0;

    for level in levels.iter() {
        let is_unsafe_change = check_level(&level, max_safe_difference);
        if is_unsafe_change {
            println!("{:?} is SAFE!", level);
            num_of_safe_reports += 1;
        } else {
            println!("{:?} is UNSAFE!", level);
        }
    }

    println!("Result {}", num_of_safe_reports);
}

fn check_level(level: &Vec<u32>, max_difference: i32) -> bool {
    let is_increasing = level.first() < level.last();
    level.windows(2).all(|pair| {
        let diff = pair[1] as i32 - pair[0] as i32;
        if diff == 0 {
            return false;
        }
        let safe_diff = diff.abs() <= max_difference;
        let safe_dir = if is_increasing { diff > 0 } else { diff < 0 };
        return safe_diff && safe_dir;
    })
}

fn parse(input: String) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|c| c.parse::<u32>().unwrap_or(0))
                .collect::<Vec<u32>>()
        })
        .collect()
}
