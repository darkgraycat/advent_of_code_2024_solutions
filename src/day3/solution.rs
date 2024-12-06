use std::str::FromStr;

use regex::Regex;

pub fn task1(input: String) {
    let re = Regex::new(r"mul\(\d+,\d+\)").expect("Regex error");
    let instructions = parse(input, &re);

    let re = Regex::new(r"mul\((\d+),(\d+)\)").expect("Regex error");
    let result: i32 = instructions
        .iter()
        .filter_map(|instruction| extract(instruction.as_str(), &re))
        .map(|(a, b)| a * b)
        .sum();

    println!("{:?}", result);
}

#[derive(Debug)]
enum Operation {
    Do,
    Dont,
    Mul(i32, i32),
}
impl FromStr for Operation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"mul\((\d+),(\d+)\)").expect("Regex error");
        match s {
            "do()" => Ok(Operation::Do),
            "don't()" => Ok(Operation::Dont),
            _ => {
                let (a, b) = extract(s, &re).expect("Error parsing instruction");
                Ok(Operation::Mul(a, b))
            }
        }
    }
}

pub fn task2(input: String) {
    let re = Regex::new(r"(mul\(\d+,\d+\))|(do\(\))|(don't\(\))").expect("Regex error");
    let instructions = parse(input, &re);
    let operations = instructions
        .iter()
        .map(|inst| inst.parse().unwrap())
        .collect::<Vec<Operation>>();

    let mut enabled = true;
    let mut result = 0;
    for operation in operations.iter() {
        match operation {
            Operation::Do => enabled = true,
            Operation::Dont => enabled = false,
            Operation::Mul(a, b) => {
                if enabled {
                    result += a * b;
                }
            }
        }
    }

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
