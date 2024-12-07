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
                update.windows(2).all(|pair| {
                    let (bef, aft) = (pair[0], pair[1]);
                    self.before
                        .get(&bef)
                        .map(|befores| befores.contains(&aft))
                        .unwrap_or(false)
                        && self
                            .after
                            .get(&aft)
                            .map(|afters| afters.contains(&bef))
                            .unwrap_or(false)
                })
            })
            .cloned()
            .collect()
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

pub fn task2(input: String) {}
