use std::collections::HashMap;

pub fn task1(input: String) {
    let (before_map, after_map, updates) = parse(&input);

    println!("BEFORE {:?}", before_map);
    println!("AFTER {:?}", after_map);
    println!("UPDATES {:?}", updates);
}

pub fn task2(input: String) {}

fn parse(
    input: &String,
) -> (
    HashMap<u32, Vec<u32>>,
    HashMap<u32, Vec<u32>>,
    Vec<Vec<u32>>,
) {
    let mut before_map = HashMap::new();
    let mut after_map = HashMap::new();
    input
        .lines()
        .take_while(|line| !line.is_empty())
        .for_each(|line| {
            let (bef, aft) = line.split_once("|").expect("Cannot split by |");
            let bef = bef.parse::<u32>().expect("Cannot parse before value");
            let aft = aft.parse::<u32>().expect("Cannot parse after value");

            before_map.entry(bef).or_insert(Vec::new()).push(aft);
            after_map.entry(aft).or_insert(Vec::new()).push(bef);
        });

    let updates = input
        .lines()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .map(|line| {
            line.split(",")
                .map(|n| n.parse::<u32>().expect("Updates parsing error"))
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    return (before_map, after_map, updates);
}
