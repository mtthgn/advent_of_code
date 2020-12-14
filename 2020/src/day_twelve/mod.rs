use std::fs;

pub fn run_part_one() {
    let mut orientation : i32 = 90;
    let mut x_position : i32 = 0;
    let mut y_position : i32 = 0;

    let callback = |instruction: Instruction| {
        match instruction {
            Instruction::Unknown(_value) => (),
            Instruction::North(value) => y_position += value,
            Instruction::South(value) => y_position -= value,
            Instruction::East(value) => x_position += value,
            Instruction::West(value) => x_position -= value,
            Instruction::Left(value) => orientation = reorient(orientation, -1 * value),
            Instruction::Right(value) => orientation = reorient(orientation, value),
            Instruction::Forward(value) => {
                match orientation {
                    0   => y_position += value,
                    90  => x_position += value,
                    180 => y_position -= value,
                    270 => x_position -= value,
                    _   => println!("Something went wrong")
                }
            }
        }
    };

    parse(callback);

    println!("Manhattan Distance: |{}| + |{}| = {}", x_position, y_position, x_position.abs() + y_position.abs());
}

pub fn run_part_two() {
    let mut boat : (i32, i32) = (0, 0);
    let mut waypoint : (i32, i32) = (10, 1);

    let callback = |instruction: Instruction| {
        match instruction {
            Instruction::Unknown(_value) => (),
            Instruction::North(value) => waypoint.1 += value,
            Instruction::South(value) => waypoint.1 -= value,
            Instruction::East(value)  => waypoint.0 += value,
            Instruction::West(value)  => waypoint.0 -= value,
            Instruction::Left(value)  => for _ in 0..value/90 { waypoint = (-waypoint.1, waypoint.0) },
            Instruction::Right(value) => for _ in 0..value/90 { waypoint = (waypoint.1, -waypoint.0) },
            Instruction::Forward(value) => boat = (boat.0 + waypoint.0 * value, boat.1 + waypoint.1 * value)
        }
    };

    parse(callback);
    println!("Manhattan Distance: |{}| + |{}| = {}", boat.0, boat.1, boat.0.abs() + boat.1.abs());
}

fn reorient(starting_degree: i32, rotate_degree: i32) -> i32 {
    let new_degree = starting_degree + rotate_degree;

    if new_degree >= 0 {
        new_degree % 360
    } else {
        360 + (new_degree % 360)
    }
}

fn parse<F>(mut callback: F) where F: FnMut(Instruction) {
    let contents = fs::read_to_string("./src/day_twelve/input.txt")
        .expect("Unable to read file");

    contents.lines().map(|line| {
        let (first, last) = line.split_at(1);
        let value = last.parse::<i32>().unwrap();
        match first {
            "N" => Instruction::North(value),
            "S" => Instruction::South(value),
            "E" => Instruction::East(value),
            "W" => Instruction::West(value),
            "F" => Instruction::Forward(value),
            "L" => Instruction::Left(value),
            "R" => Instruction::Right(value),
            _ => Instruction::Unknown(value)
        }
    }).for_each(|instruction|
        callback(instruction)
    );
}

enum Instruction {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Forward(i32),
    Left(i32),
    Right(i32),
    Unknown(i32)
}
