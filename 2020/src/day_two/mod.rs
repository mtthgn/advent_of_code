use std::fs;

struct PasswordPolicy {
  min_amount: usize,
  max_amount: usize,
  character: char
}

pub fn run_part_one() {
  parse_file_and_validate(validate_part_one)
}

pub fn run_part_two() {
  parse_file_and_validate(validate_part_two)
}

fn validate_part_one(policy: &PasswordPolicy, password: &String) -> bool {
  let occurences = password.chars()
    .filter(|c| c == &policy.character)
    .count();

  (policy.min_amount..=policy.max_amount).contains(&occurences)
}

fn validate_part_two(policy: &PasswordPolicy, password: &String) -> bool {
    let first_location = password.chars().nth(policy.min_amount - 1).unwrap();
    let last_location = password.chars().nth(policy.max_amount - 1).unwrap();

    return first_location != last_location && ((first_location == policy.character) || (last_location == policy.character));
}


fn parse_file_and_validate(validator: fn(&PasswordPolicy, &String) -> bool) {
  let contents = fs::read_to_string("./src/day_two/input.txt")
    .expect("Unable to read file.");

  let results = contents.lines()
    .filter(|line| check_password(line.to_string(), validator))
    .count();

  println!("Valid passwords: {}", results);
}

fn check_password(input: String, validator: fn(&PasswordPolicy, &String) -> bool) -> bool {
  let split_input: Vec<String> = input.split(": ")
    .map(|s| s.to_string())
    .collect();
  let policy = create_password_policy(&split_input[0]);
  let password = &split_input[1];

  validator(&policy, password)
}

fn create_password_policy(input: &String) -> PasswordPolicy {
  let split_input: Vec<String> = input.split(" ")
    .map(|s| s.to_string())
    .collect();
  let split_amounts: Vec<String> = split_input[0].split("-")
    .map(|s| s.to_string())
    .collect();

  let character = split_input[1].chars().next().unwrap();
  let min_amount = split_amounts[0].parse::<usize>().unwrap();
  let max_amount = split_amounts[1].parse::<usize>().unwrap();

  PasswordPolicy {
    min_amount,
    max_amount,
    character
  }
}
