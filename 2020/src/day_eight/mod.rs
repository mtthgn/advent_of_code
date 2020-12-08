use std::fs;
use std::collections::HashSet;

pub fn run_part_one() {
    let instructions = parse();
    let instruction_length = instructions.len();

    let mut already_ran = HashSet::<usize>::new();
    let mut position : i32 = 0;
    let mut accumulator : i32 = 0;

    while (0..instruction_length).contains(&(position as usize)) {
        let instruction = &instructions[position as usize];

        if already_ran.contains(&instruction.line_number) {
            break;
        } else {
            already_ran.insert(instruction.line_number);
        }

        match instruction.command.as_str() {
            "jmp" => position = position + instruction.value,
            "nop" => position += 1,
            "acc" => {
                accumulator += instruction.value;
                position += 1
            },
            _ => break,
        }
    }

    println!("Accumulator: {}", accumulator);
}

pub fn run_part_two() {
    let instructions = parse();
    let instruction_length = instructions.len();
    let mut already_switched = HashSet::<usize>::new();
    let mut position : i32 ;
    let mut accumulator : i32;

    loop {
        let mut already_ran = HashSet::<usize>::new();
        let mut switched_this_loop = false;

        position = 0;
        accumulator = 0;

        while (0..instruction_length).contains(&(position as usize)) {
            let instruction = &instructions[position as usize];

            if already_ran.contains(&instruction.line_number) {
                break;
            } else {
                already_ran.insert(instruction.line_number);
            }

            match instruction.command.as_str() {
                "jmp" => {
                    if already_switched.contains(&instruction.line_number) || switched_this_loop {
                        position = position + instruction.value;
                    } else {
                        already_switched.insert(instruction.line_number);
                        switched_this_loop = true;
                        position += 1;
                    }
                },
                "nop" => {
                    if already_switched.contains(&instruction.line_number) || switched_this_loop {
                        position += 1
                    } else {
                        already_switched.insert(instruction.line_number);
                        switched_this_loop = true;
                        position = position + instruction.value;
                    }
                },
                "acc" => {
                    accumulator += instruction.value;
                    position += 1
                },
                _ => break,
            }
        }

        if position as usize >= instruction_length { break }
    }

    println!("Accumulator: {}", accumulator);
}

fn parse() -> Vec<Instruction> {
    let contents = fs::read_to_string("./src/day_eight/input.txt")
        .expect("Unable to read file");

    contents.lines().enumerate()
        .map(|(i, line)| Instruction::new(line.to_string(), i))
        .collect()
}

struct Instruction {
    command: String,
    value: i32,
    line_number: usize
}

impl Instruction {
    fn new(instruction_string: String, line_number: usize) -> Instruction {
        let mut iterator = instruction_string.split(" ");
        let command = iterator.next().unwrap().to_string();
        let value = iterator.next().unwrap().parse::<i32>().unwrap();

        Instruction { command, value, line_number }
    }
}
