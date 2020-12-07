use std::fs;

struct Slope {
    x: usize,
    y: usize
}

pub fn run_part_one() {
    let slope = Slope{ x: 3, y: 1 };
    let tree_count = traverse_slope(&slope);

    println!("Trees Encountered {}", tree_count);
}

pub fn run_part_two() {
    let slopes = [
        Slope{ x: 1, y: 1 },
        Slope{ x: 3, y: 1 },
        Slope{ x: 5, y: 1 },
        Slope{ x: 7, y: 1 },
        Slope{ x: 1, y: 2 }
    ];

    let mut multiplied_results : u64 = 1;

    slopes.iter()
        .map(|slope| traverse_slope(slope))
        .for_each(|v| multiplied_results *= v);

    println!("Final number: {}", multiplied_results);
}

fn traverse_slope(slope: &Slope) -> u64 {
    let mut tree_count = 0;
    let mut position = 0;
    let tree_finder = |index: usize, row: String| {
        if index % slope.y == 0 {
            let length = row.len();

            if row.chars().nth(position % length).unwrap()  == '#' {
                tree_count += 1;
            }
            position += slope.x;
        }
    };

    parse_and_traverse(tree_finder);
    return tree_count;
}

fn parse_and_traverse<F>(mut traverse: F) where F: FnMut(usize, String) {
    let contents = fs::read_to_string("./src/day_three/input.txt")
        .expect("Unable to read file");

    contents.lines()
        .enumerate()
        .for_each(|(index, row)| traverse(index, row.to_string()));
}
