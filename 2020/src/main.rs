use std::env;

mod day_one;
mod day_two;
mod day_three;
mod day_four;
mod day_five;
mod day_six;
mod day_seven;
mod day_eight;
mod day_nine;
mod day_ten;
mod day_eleven;
mod day_twelve;
mod day_thirteen;

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
        "day_four_part_one" => day_four::run_part_one(),
        "day_four_part_two" => day_four::run_part_two(),
        "day_five_part_one" => day_five::run_part_one(),
        "day_five_part_two" => day_five::run_part_two(),
        "day_six_part_one" => day_six::run_part_one(),
        "day_six_part_two" => day_six::run_part_two(),
        "day_seven_part_one" => day_seven::run_part_one(),
        "day_seven_part_two" => day_seven::run_part_two(),
        "day_eight_part_one" => day_eight::run_part_one(),
        "day_eight_part_two" => day_eight::run_part_two(),
        "day_nine_part_one" => day_nine::run_part_one(),
        "day_nine_part_two" => day_nine::run_part_two(),
        "day_ten_part_one" => day_ten::run_part_one(),
        "day_ten_part_two" => day_ten::run_part_two(),
        "day_eleven_part_one" => day_eleven::run_part_one(),
        "day_eleven_part_two" => day_eleven::run_part_two(),
        "day_twelve_part_one" => day_twelve::run_part_one(),
        "day_twelve_part_two" => day_twelve::run_part_two(),
        "day_thirteen_part_one" => day_thirteen::run_part_one(),
        "day_thirteen_part_two" => day_thirteen::run_part_two(),
        _ => println!("Unrecognized Day input")
    }
}
