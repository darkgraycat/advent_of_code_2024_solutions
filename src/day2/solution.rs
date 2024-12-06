pub fn task1(input: String) {
    let levels = parse(input);

    let max_safe_difference: u32 = 3;
    let mut num_of_safe_reports: u32 = 0;

    for level in levels.iter() {
        let is_increasing = level.first() < level.last();
        println!("level {:?} is increasing {}", level, is_increasing);

        let is_unsafe_change = level.windows(2).all(|pair| {
            let diff: i32 = pair[1] as i32 - pair[0] as i32;
            if diff == 0 {
                return false;
            }
            let safe_difference = diff.abs() as u32 <= max_safe_difference;
            let safe_direction = if is_increasing { diff > 0 } else { diff < 0 };

            // println!("{:?} {} {} {}", pair, diff, safe_difference, safe_direction);
            return safe_difference && safe_direction;
        });

        if is_unsafe_change {
            println!("{:?} is SAFE!", level);
            num_of_safe_reports += 1;
        } else {
            println!("{:?} is UNSAFE!", level);
        }
    }

    println!("Result {}", num_of_safe_reports);
}

pub fn task2(input: String) {
    let levels = parse(input);

    let max_safe_difference: u32 = 3;
    let mut num_of_safe_reports: u32 = 0;

    for level in levels.iter() {
        let is_increasing = level.first() < level.last();
        println!("level {:?} is increasing {}", level, is_increasing);

        let mut problem_dampener_factor_flag_used = false;

        let is_unsafe_change = level.windows(2).all(|pair| {
            let diff: i32 = pair[1] as i32 - pair[0] as i32;
            if diff == 0 {
                return false;
            }
            let safe_difference = diff.abs() as u32 <= max_safe_difference;
            let safe_direction = if is_increasing { diff > 0 } else { diff < 0 };

            if !safe_difference || !safe_direction {
                if problem_dampener_factor_flag_used {
                    return false;
                }
                problem_dampener_factor_flag_used = true;
                return true;
            }

            return true;
        });

        if is_unsafe_change {
            println!("{:?} is SAFE!", level);
            num_of_safe_reports += 1;
        } else {
            println!("{:?} is UNSAFE!", level);
        }
    }

    println!("Result {}", num_of_safe_reports);
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
