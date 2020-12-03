use std::env;

mod day_one;
mod day_two;
mod day_three;

fn main() {
    let args: Vec<String> = env::args().collect();
    let selection = &args[1];

    match selection.as_str() {
        "day_one_part_one" => day_one::run_part_one(),
        "day_one_part_two" => day_one::run_part_two(),
        "day_two_part_one" => day_two::run_part_one(),
        "day_two_part_two" => day_two::run_part_two(),
        "day_three_part_one" => day_three::run_part_one(),
        "day_three_part_two" => day_three::run_part_two(),
        _ => println!("Unrecognized Day input")
    }
}
