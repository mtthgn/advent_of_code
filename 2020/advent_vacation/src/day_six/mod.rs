use std::fs;
use std::collections::HashSet;

struct Group {
    answers: Vec<String>,
}

impl Group {
    fn new(group_string: String) -> Group {
        let answers = group_string.lines()
            .map(|line| line.to_string())
            .collect();

        return Group { answers };
    }

    fn unique_answers(&self) -> usize {
        let mut set = HashSet::new();

        for answer in &self.answers {
            for c in answer.chars() {
                set.insert(c);
            }
        }

        return set.len();
    }

    fn all_answered_yes(&self) -> usize {
        let mut set = HashSet::new();
        let alphabet = [
            "a", "b", "c", "d", "e",
            "f", "g", "h", "i", "j",
            "k", "l", "m", "n", "o",
            "p", "q", "r", "s", "t",
            "u", "v", "w", "x", "y", "z"
        ];

        for character in alphabet.iter() {
            if self.answers.iter().all(|answer| answer.contains(character)) {
                set.insert(character);
            }
        }

        return set.len();
    }
}

pub fn run_part_one() {
    let groups = parse();
    let total : usize = groups.iter()
        .map(|group| group.unique_answers())
        .sum();

    println!("Total Yes: {}", total);
}

pub fn run_part_two() {
    let groups = parse();
    let total : usize = groups.iter()
        .map(|group| group.all_answered_yes())
        .sum();

    println!("Total Yes: {}", total);
}

fn parse() -> Vec<Group> {
    let contents = fs::read_to_string("./src/day_six/input.txt")
        .expect("Unable to read file");

    return contents.split("\n\n")
        .map(|group_string| Group::new(group_string.to_string()))
        .collect();
}
