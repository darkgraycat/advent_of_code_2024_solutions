use std::collections::HashMap;

pub fn task1(input: String) {
    let (mut left, mut right) = parse(input);

    left.sort();
    right.sort();

    let result: u32 = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| l.abs_diff(*r))
        .sum();
    println!("Result: {}", result);
}

pub fn task2(input: String) {
    let (left, right) = parse(input);

    let mut right_map: HashMap<u32, u32> = HashMap::new();
    right
        .iter()
        .for_each(|x| *right_map.entry(*x).or_insert(0) += 1);

    let result: u32 = left
        .iter()
        .map(|x| x * right_map.get(x).copied().unwrap_or(0))
        .sum();

    println!("{:?}", result);
}

fn parse(input: String) -> (Vec<u32>, Vec<u32>) {
    input
        .lines()
        .filter_map(|line| {
            let mut nums = line.trim().split_whitespace().map(|c| c.parse::<u32>());
            match (nums.next(), nums.next()) {
                (Some(Ok(n1)), Some(Ok(n2))) => Some((n1, n2)),
                _ => None,
            }
        })
        .unzip()
}
