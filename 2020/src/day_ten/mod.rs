use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn run_part_one() {
    let contents = fs::read_to_string("./src/day_ten/input.txt")
        .expect("Unable to read file");

    let mut current_jolts = 0;
    let mut jolt_differences = HashMap::<usize, usize>::new();
    let mut adapters : Vec<usize> = contents.lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect();

    adapters.sort();

    // built in adapter has a rating 3 higher than the highest value in the
    // input, so just inserting the difference now.
    jolt_differences.insert(3, 1);

    for value in adapters.iter() {
        let difference =  value - current_jolts;

        if difference <= 3 {
            let diff_count = jolt_differences.entry(difference).or_insert(0);
            *diff_count += 1;
            current_jolts = value.clone();
        }
    }

    for (key, value) in jolt_differences.iter() {
        println!("{} --> {}", key, value);
    }

    let one_count = jolt_differences.get(&1).unwrap();
    let three_count = jolt_differences.get(&3).unwrap();
    println!("Result: {}", one_count * three_count);
}

pub fn run_part_two() {
    let contents = fs::read_to_string("./src/day_ten/input.txt")
        .expect("Unable to read file");

    let mut tree = HashMap::<i64, HashSet<i64>>::new();
    let mut cache = HashMap::<i64, i64>::new();
    let mut adapters : HashSet<i64> = contents.lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect();

    adapters.insert(0);

    for adapter in adapters.iter() {
        let mut nodes = HashSet::<i64>::new();

        for i in [1, 2, 3].iter() {
            let value = adapter - i;
            if adapters.contains(&value) { nodes.insert(value); }
        }

        tree.insert(*adapter, nodes);
    }

    let max_jolt = adapters.iter().max().unwrap();
    let possiblities = paths_to_jolt(*max_jolt, &tree, &mut cache);

    println!("{}", possiblities);
}

fn paths_to_jolt(current_jolt: i64, tree: &HashMap<i64, HashSet<i64>>, cache: &mut HashMap<i64, i64>) -> i64 {
    if current_jolt == 0 {
        cache.insert(current_jolt, 1);
        return 1
    }

    match cache.get(&current_jolt) {
        Some(value) => return *value,
        None => {
            let children = tree.get(&current_jolt).unwrap();
            let sum = children.iter().fold(0, |sum, child| sum + paths_to_jolt(*child, tree, cache));
            cache.insert(current_jolt, sum);
            return sum
        }
    }
}
