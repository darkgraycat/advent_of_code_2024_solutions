use std::collections::HashMap;

#[derive(Debug)]
struct InputData {
    before: HashMap<u32, Vec<u32>>,
    after: HashMap<u32, Vec<u32>>,
    updates: Vec<Vec<u32>>,
}
impl InputData {
    fn parse(input: &String) -> Self {
        let mut before = HashMap::new();
        let mut after = HashMap::new();
        let (order_list, updates_list) = input.split_once("\n\n").expect("Invalid input");
        order_list.lines().for_each(|line| {
            let (bef, aft) = line.split_once("|").expect("Cannot split by |");
            let bef = bef.parse::<u32>().expect("Cannot parse before value");
            let aft = aft.parse::<u32>().expect("Cannot parse after value");

            before.entry(bef).or_insert(Vec::new()).push(aft);
            after.entry(aft).or_insert(Vec::new()).push(bef);
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
            before,
            after,
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

    fn check_order(&self, before: u32, after: u32) -> bool {
        self.is_allowed_after(before, after) && self.is_allowed_before(after, before)
    }

    fn is_allowed_after(&self, value: u32, after: u32) -> bool {
        self.before
            .get(&value)
            .map(|befores| befores.contains(&after))
            .unwrap_or(false)
    }

    fn is_allowed_before(&self, value: u32, before: u32) -> bool {
        self.after
            .get(&value)
            .map(|afters| afters.contains(&before))
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

    let incorrect_updates = input_data.get_incorrect_updates();

    println!("{:?}", incorrect_updates);
}
