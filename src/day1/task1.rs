pub fn run(input: String) {
    let (mut left, mut right) = parse(input);

    left.sort();
    right.sort();

    let result: u32 = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| if l > r { l - r } else { r - l })
        .sum();
    println!("Result: {}", result);
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
