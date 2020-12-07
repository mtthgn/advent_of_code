use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn run_part_one() {
    let luggages = parse();
    let initial_color = "shiny gold";

    let bags = fits_in_bags(initial_color.to_string(), &luggages);

    println!("{}", bags.len());
}

pub fn run_part_two() {
    let luggages = parse();
    let initial_color = "shiny gold";

    let total_bags = total_bags_needed(initial_color.to_string(), &luggages);
    println!("{}", total_bags);
}

fn parse() -> HashMap<String, Luggage> {
    let contents = fs::read_to_string("./src/day_seven/input.txt")
        .expect("Unable to read file");
    let luggages = contents.lines().map(|line| Luggage::new(line.to_string()));
    let mut luggage_map = HashMap::<String, Luggage>::new();

    for luggage in luggages {
        luggage_map.insert(luggage.color.clone(), luggage);
    }

    return luggage_map;
}

struct Luggage {
    color: String,
    requirements: HashMap<String, usize>
}

impl Luggage {
    fn new(luggage_string: String) -> Luggage {
        // light red bags contain 1 bright white bag, 2 muted yellow bags.
        let mut split = luggage_string.split(" bags contain ");
        let color = split.next().unwrap().to_string();
        let requirement_string = split.next().unwrap().trim_end_matches(".");
        let mut requirements = HashMap::<String, usize>::new();

        if requirement_string != "no other bags" {
            for requirement in requirement_string.split(", ") {
                let mut without_suffix = requirement.trim_end_matches(" bags");
                without_suffix = without_suffix.trim_end_matches(" bag");

                let mut requirement_split = without_suffix.splitn(2, ' ');
                let amount_value = requirement_split.next().unwrap();
                let color_key = requirement_split.next().unwrap().to_string();

                requirements.insert(color_key, amount_value.parse::<usize>().unwrap());
            }
        }

        return Luggage { color, requirements };
    }
}

fn fits_in_bags(color: String, luggages: &HashMap<String, Luggage>) -> HashSet<String> {
    let mut bags = HashSet::new();

    luggages.iter().for_each(|(color_key, luggage)| {
        if luggage.requirements.contains_key(&color) {
            bags.insert(color_key.to_string());

            for additional_color in fits_in_bags(color_key.to_string(), luggages).iter() {
                bags.insert(additional_color.to_string());
            }
        }
    });

    return bags;
}

fn total_bags_needed(color: String, luggages: &HashMap<String, Luggage>) -> usize {
    let luggage = luggages.get(&color).unwrap();
    let mut final_total = 0;
    luggage.requirements.iter().for_each(|(color_key, amount)| {
        final_total += amount;
        final_total += amount * total_bags_needed(color_key.to_string(), luggages);
    });

    return final_total;
}
