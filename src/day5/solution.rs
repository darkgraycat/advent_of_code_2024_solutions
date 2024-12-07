use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug)]
struct InputData {
    order: HashMap<u32, Vec<u32>>,
    updates: Vec<Vec<u32>>,
}
impl InputData {
    fn parse(input: &String) -> Self {
        let mut order = HashMap::new();
        let (order_list, updates_list) = input.split_once("\n\n").expect("Invalid input");
        order_list.lines().for_each(|line| {
            let (bef, aft) = line.split_once("|").expect("Cannot split by |");
            let bef = bef.parse::<u32>().expect("Cannot parse before value");
            let aft = aft.parse::<u32>().expect("Cannot parse after value");

            order.entry(bef).or_insert(Vec::new()).push(aft);
        });

        let updates = updates_list
            .lines()
            .map(|line| {
                line.split(",")
                    .map(|n| n.parse::<u32>().expect("Updates parsing error"))
                    .collect::<Vec<u32>>()
            })
            .collect::<Vec<Vec<u32>>>();

        InputData { order, updates }
    }

    fn get_correct_updates(&self) -> Vec<Vec<u32>> {
        self.updates
            .iter()
            .filter(|update| {
                update
                    .windows(2)
                    .all(|pair| self.check_order(pair[0], pair[1]))
            })
            .cloned()
            .collect()
    }

    fn get_incorrect_updates(&self) -> Vec<Vec<u32>> {
        self.updates
            .iter()
            .filter(|update| {
                update
                    .windows(2)
                    .any(|pair| !self.check_order(pair[0], pair[1]))
            })
            .cloned()
            .collect()
    }

    fn check_order(&self, value: u32, after: u32) -> bool {
        self.order
            .get(&value)
            .map(|befores| befores.contains(&after))
            .unwrap_or(false)
    }
}

pub fn task1(input: String) {
    let input_data = InputData::parse(&input);

    let correct_updates = input_data.get_correct_updates();

    let results: u32 = correct_updates
        .iter()
        .map(|update| update[update.len() / 2])
        .sum();

    println!("{:?}", results);
}

pub fn task2(input: String) {
    let input_data = InputData::parse(&input);

    let mut incorrect_updates = input_data.get_incorrect_updates();


    incorrect_updates.iter_mut().for_each(|update| {
        update.sort_by(|&a, &b| {
            if input_data.check_order(a, b) {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });
    });

    let results: u32 = incorrect_updates
        .iter()
        .map(|update| update[update.len() / 2])
        .sum();

    println!("{:?}", results);
}

fn find_intersection(vec1: &Vec<u32>, vec2: &Vec<u32>) -> Vec<u32> {
    vec1.iter().filter(|x| vec2.contains(x)).cloned().collect()
}
