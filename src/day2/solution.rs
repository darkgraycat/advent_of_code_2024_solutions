pub fn task1(input: String) {
    let levels = parse(input);

    let max_safe_difference = 3;
    let mut num_of_safe_reports: u32 = 0;

    for level in levels.iter() {
        let invalid_change = check_level(&level, max_safe_difference);
        match invalid_change {
            Some(idx) => println!("{:?} - UNSAFE at {}", level, idx),
            None => {
                println!("{:?} - SAFE", level);
                num_of_safe_reports += 1;
            }
        }
    }

    println!("Result {}", num_of_safe_reports);
}

pub fn task2(input: String) {
    let levels = parse(input);

    let max_safe_difference = 3;
    let mut num_of_safe_reports: u32 = 0;

    for level in levels.iter() {
        match check_level(&level, max_safe_difference) {
            Some(idx) => {
                let mut lvl_a = level.clone();
                let mut lvl_b = level.clone();
                lvl_a.remove(idx);
                lvl_b.remove(idx - 1);
                let check_lvl_a = check_level(&lvl_a, max_safe_difference);
                let check_lvl_b = check_level(&lvl_b, max_safe_difference);

                match (check_lvl_a, check_lvl_b) {
                    (Some(_), Some(_)) => {
                        println!("{:?} - UNSAFE", level);
                    }
                    _ => {
                        println!("{:?} - SAFE", level);
                        num_of_safe_reports += 1;
                    }
                }
            }
            None => {
                println!("{:?} - SAFE", level);
                num_of_safe_reports += 1;
            }
        }
    }

    println!("Result {}", num_of_safe_reports);
}

fn check_level(level: &Vec<u32>, max_difference: i32) -> Option<usize> {
    let is_increasing = level.first() < level.last();
    for (i, pair) in level.windows(2).enumerate() {
        let diff = pair[1] as i32 - pair[0] as i32;

        if diff == 0 || diff.abs() > max_difference {
            return Some(i + 1);
        }

        let is_invalid_increasing = is_increasing && diff < 0;
        let is_invalid_decreasing = !is_increasing && diff > 0;

        if is_invalid_increasing || is_invalid_decreasing {
            return Some(i + 1);
        }
    }

    return None;
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
