pub fn task1(input: String) {
    let operators = vec!['+', '*'];

    let calibrations: Vec<Calibration> = input
        .lines()
        .filter_map(|line| Calibration::try_from(line.to_owned()).ok())
        .collect();

    let calibrators: Vec<Calibrator> = calibrations
        .into_iter()
        .map(|calibration| Calibrator::new(calibration, operators.clone()))
        .collect();

    let calibrated: Vec<i64> = calibrators
        .iter()
        .filter_map(|calibrator| calibrator.calibrate())
        .collect();

    println!("{:?}", calibrated.iter().sum::<i64>());
}

pub fn task2(input: String) {
    let operators = vec!['+', '*', '|'];

    let calibrations: Vec<Calibration> = input
        .lines()
        .filter_map(|line| Calibration::try_from(line.to_owned()).ok())
        .collect();

    let calibrators: Vec<Calibrator> = calibrations
        .into_iter()
        .map(|calibration| Calibrator::new(calibration, operators.clone()))
        .collect();

    let calibrated: Vec<i64> = calibrators
        .iter()
        .filter_map(|calibrator| calibrator.calibrate())
        .collect();

    println!("{:?}", calibrated.iter().sum::<i64>());
}

#[derive(Debug, Clone)]
struct Calibration {
    test_value: i64,
    values: Vec<i64>,
}

impl TryFrom<String> for Calibration {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let (test_value, values) = value.split_once(':').ok_or("Invalid input")?;

        Ok(Calibration {
            test_value: test_value.parse().expect("Parsing error"),
            values: values
                .split_whitespace()
                .filter_map(|v| v.parse().ok())
                .collect(),
        })
    }
}

#[derive(Debug)]
enum Operation {
    Add,
    Multiply,
    Concat,
}

impl TryFrom<char> for Operation {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '+' => Ok(Operation::Add),
            '*' => Ok(Operation::Multiply),
            '|' => Ok(Operation::Concat),
            _ => Err("Invalid operator".to_owned()),
        }
    }
}

#[derive(Debug)]
struct Calibrator {
    calibration: Calibration,
    combinations: Vec<Vec<Operation>>,
}

impl Calibrator {
    fn new(calibration: Calibration, operators: Vec<char>) -> Self {
        let operations: Vec<Vec<Operation>> =
            Calibrator::generate_combinations(operators, calibration.values.len() as u32 - 1)
                .into_iter()
                .map(|combination| {
                    combination
                        .chars()
                        .map(|ch| ch.try_into().expect("Cannot parse operation"))
                        .collect()
                })
                .collect();
        Self {
            calibration,
            combinations: operations,
        }
    }

    fn calibrate(&self) -> Option<i64> {
        let Calibration { test_value, values } = &self.calibration;
        for ops in self.combinations.iter() {
            let result = values
                .windows(2)
                .enumerate()
                .scan(values[0], |a, (idx, window)| {
                    let b = window[1];
                    *a = match &ops[idx] {
                        Operation::Add => *a + b,
                        Operation::Multiply => *a * b,
                        Operation::Concat => format!("{}{}", a, b).parse().unwrap(),
                    };
                    Some(*a)
                })
                .last()
                .unwrap_or(values[0]);

            if result == *test_value {
                return Some(result);
            }
        }

        None
    }

    fn generate_combinations(operators: Vec<char>, length: u32) -> Vec<String> {
        let total = operators.len().pow(length);

        let mut combinations = Vec::new();

        for i in 0..total {
            let combo: String = (0..length)
                .map(|j| operators[(i / operators.len().pow(j)) % operators.len()])
                .collect();
            combinations.push(combo.chars().rev().collect());
        }

        combinations
    }
}
