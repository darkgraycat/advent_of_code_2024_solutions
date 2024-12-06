use regex::Regex;

pub fn task1(input: String) {
    let re = Regex::new(r"mul\(\d+,\d+\)").expect("Regex error");
    let parsed = parse(input, &re);

    let re = Regex::new(r"mul\((\d+),(\d+)\)").expect("Regex error");
    let result: i32 = parsed
        .iter()
        .filter_map(|instruction| extract(instruction.as_str(), &re))
        .map(|(a, b)| a * b)
        .sum();

    println!("{:?}", result);
}

fn parse(input: String, re: &Regex) -> Vec<String> {
    re.captures_iter(input.as_str())
        .map(|cap| cap[0].to_string())
        .collect()
}

fn extract(instruction: &str, re: &Regex) -> Option<(i32, i32)> {
    re.captures(instruction).and_then(|captures| {
        let num1 = captures[1].parse().ok()?;
        let num2 = captures[2].parse().ok()?;
        Some((num1, num2))
    })
}
