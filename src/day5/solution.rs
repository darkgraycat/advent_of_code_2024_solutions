use std::collections::HashMap;

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

        InputData {
            order,
            updates,
        }
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

    println!("{:?}", input_data);

    let incorrect_updates = input_data.get_incorrect_updates();

    // 75,97,47,61,53 becomes 97,75,47,61,53.
    // 61,13,29 becomes 61,29,13.
    // 97,13,75,29,47 becomes 97,75,47,29,13.
    // 97,75,29,13      13

    let fixed_updated = incorrect_updates
        .iter()
        .map(|update| {
            let mut current: usize = 0;
            while current < update.len() {
                println!("{}", update[current]);
                current += 1;
            }
            return update;
        })
        .cloned()
        .collect::<Vec<Vec<u32>>>();

    println!("{:?}", fixed_updated);
}

fn find_intersection(vec1: &Vec<u32>, vec2: &Vec<u32>) -> Vec<u32> {
    vec1.iter().filter(|x| vec2.contains(x)).cloned().collect()
}
