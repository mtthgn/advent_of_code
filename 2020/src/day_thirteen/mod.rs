use std::fs;

pub fn run_part_one() {
    let (estimate, bus_ids) = parse();
    let mut shortest_distance : Option<(usize, usize)> = None;

    for (_, id) in bus_ids {
        let id_distance = ((estimate / id + 1) * id) - estimate;

        match shortest_distance {
            Some((_, distance)) if distance < id_distance  => (),
            Some((_, _distance)) => shortest_distance = Some((id, id_distance)),
            None => shortest_distance = Some((id, id_distance))
        }
    }

    let (id, distance) = shortest_distance.unwrap();
    println!("ID: {}, Distance: {}, Multipled: {}", id, distance, id * distance);
}

pub fn run_part_two() {
    let (_, bus_ids) = parse();
    let (_, first_value) = bus_ids[0];
    let (last_index, _) = bus_ids[bus_ids.len() - 1];

    let mut modulo : u64 = first_value as u64;
    let mut products : u64 = first_value as u64;
    let mut last_product_index = 0;


    loop {
        let mut all_match = true;

        for (index, id) in bus_ids.iter() {
            if (modulo + *index as u64) % (*id as u64) == 0 {
                if index > &last_product_index {
                    if index == &last_index { break; }
                    last_product_index = *index;
                    products *= *id as u64;
                }
                continue;
            }

            modulo += products;
            all_match = false;
            break;
        }

        if all_match == true { break; }
    }

    println!("{}", modulo);
}

fn parse() -> (usize, Vec<(usize, usize)>) {
    let contents = fs::read_to_string("./src/day_thirteen/input.txt")
        .expect("Unable to read file");
    let mut iter = contents.lines();

    let estimate_string = iter.next().unwrap();
    let estimate = estimate_string.parse::<usize>().unwrap();
    let bus_id_string = iter.next().unwrap();
    let mut bus_ids = Vec::<(usize, usize)>::new();

    for (index, id) in bus_id_string.split(',').enumerate() {
        match id {
            "x" => (),
            _ => bus_ids.push((index, id.parse::<usize>().unwrap()))
        }
    }

    (estimate, bus_ids)
}

