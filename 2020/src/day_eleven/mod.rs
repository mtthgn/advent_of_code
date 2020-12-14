use std::fs;
use std::collections::HashMap;

pub fn run_part_one() {
    let contents = fs::read_to_string("./src/day_eleven/input.txt")
        .expect("Unable to read file");

    let mut current_grid = Grid::new(contents);
    let mut count = 0;

    loop {
        if count % 100 == 0 { println!("{}", count); }
        let new_grid = current_grid.simulate_arrival();
        if new_grid == current_grid { break; }
        current_grid = new_grid;
        count += 1;
    }

    println!("{}", count);
    println!("Occupied Seats: {}", current_grid.occupied_seats());
}

pub fn run_part_two() {
    let contents = fs::read_to_string("./src/day_eleven/input.txt")
        .expect("Unable to read file");

    let mut current_grid = Grid::new(contents);
    let mut count = 0;

    loop {
        if count % 100 == 0 { println!("{}", count); }
        let new_grid = current_grid.simulate_part_two();
        if new_grid == current_grid { break; }
        current_grid = new_grid;
        count += 1;
    }

    println!("{}", count);
    println!("Occupied Seats: {}", current_grid.occupied_seats());
}

#[derive(PartialEq, Copy, Clone)]
enum State { Occupied, Empty, Floor }

struct Grid {
    map: HashMap<(usize, usize), State>,
    row_count: usize,
    column_count: usize
}

impl Grid {
    fn new(map_string: String) -> Grid {
        let mut map = HashMap::<(usize, usize), State>::new();
        let mut row_count = 0;
        let mut column_count = 0;

        map_string.lines().enumerate().for_each(|(row, line)| {
            line.chars().enumerate().for_each(|(column, character)| {
                let state = match character {
                    'L' => State::Empty,
                    '#' => State::Occupied,
                    '.' => State::Floor,
                    _ => State::Floor
                };

                map.insert((row, column), state);
                column_count = column;
            });
            row_count = row;
        });

        Grid { map, row_count, column_count }
    }

    fn occupied_seats(&self) -> usize {
        self.map.iter().fold(0, |sum, ((_row,_column), state)| {
            if *state == State::Occupied { sum + 1 } else { sum }
        })
    }

    fn simulate_arrival(&self) -> Grid {
        let mut map = HashMap::<(usize, usize), State>::new();
        let row_count = self.row_count;
        let column_count = self.column_count;

        self.map.iter().for_each(|((row, column), state)|
            match state {
                State::Floor => {
                    map.insert((*row, *column), State::Floor);
                },
                State::Occupied => {
                    if self.neighbors_occupied(row, column) >= 4 {
                        map.insert((*row, *column), State::Empty);
                    } else {
                        map.insert((*row, *column), State::Occupied);
                    }
                },
                State::Empty => {
                    if self.neighbors_occupied(row, column) == 0 {
                        map.insert((*row, *column), State::Occupied);
                    } else {
                        map.insert((*row, *column), State::Empty);
                    }
                }
            }
        );

        Grid { map, row_count, column_count }
    }

    fn simulate_part_two(&self) -> Grid {
        let mut map = HashMap::<(usize, usize), State>::new();
        let row_count = self.row_count;
        let column_count = self.column_count;

        self.map.iter().for_each(|((row, column), state)|
            match state {
                State::Floor => {
                    map.insert((*row, *column), State::Floor);
                },
                State::Occupied => {
                    if self.visible_occupied_count(row, column) >= 5 {
                        map.insert((*row, *column), State::Empty);
                    } else {
                        map.insert((*row, *column), State::Occupied);
                    }
                },
                State::Empty => {
                    if self.visible_occupied_count(row, column) == 0 {
                        map.insert((*row, *column), State::Occupied);
                    } else {
                        map.insert((*row, *column), State::Empty);
                    }
                }
            }
        );

        Grid { map, row_count, column_count }
    }

    fn state_at(&self, row: usize, column: usize) -> Option<&State> {
       self.map.get(&(row, column))
    }

    fn neighbors_occupied(&self, row: &usize, column: &usize) -> usize {
        let neighbors = self.adjacent_states(row, column);
        neighbors.iter().fold(0, |sum, state| {
            if *state == State::Occupied { sum + 1 } else { sum }
        })
    }

    fn adjacent_states(&self, row: &usize, column: &usize) -> Vec<State> {
        let mut adjacents = Vec::<State>::new();

        let min_row : usize = std::cmp::max(0, (*row as i32) - 1) as usize;
        let max_row : usize = std::cmp::min(self.row_count, row + 1);
        let min_column: usize = std::cmp::max(0, (*column as i32)-1) as usize;
        let max_column: usize = std::cmp::min(self.column_count, column + 1);

        for current_row in min_row..=max_row {
            for current_column in min_column..=max_column {
                if current_row == *row && current_column == *column { continue; }

                match self.state_at(current_row, current_column) {
                    Some(state) => adjacents.push(*state),
                    None => ()
                }
            }
        }

        adjacents
    }

    fn visible_occupied_count(&self, row: &usize, column: &usize) -> usize {
        let mut states = Vec::<State>::new();

        for x in -1..=1 {
            for y in -1..=1 {
                if x == 0 && y == 0 { continue; }
                states.push(self.find_state(row, column, (x, y)));
            }
        }

        states.iter()
            .fold(0, |sum, state| if *state == State::Occupied { sum + 1 } else { sum })
    }

    fn find_state(&self, origin_row: &usize, origin_column: &usize, slope: (i32, i32)) -> State {
        let mut loops = 1;

        loop {
            let row = (*origin_row as i32) + (slope.0 * loops);
            let column = (*origin_column as i32) + (slope.1 * loops);

            if row > (self.row_count as i32) || row < 0 || column > (self.column_count as i32) || column < 0 {
                break;
            }

            match self.state_at(row as usize, column as usize) {
                Some(state) if *state == State::Floor => (),
                Some(state) => return *state,
                None => ()
            }

            loops += 1;
        }

        State::Floor
    }

}

impl PartialEq for Grid {
    fn eq(&self, other: &Self) -> bool {
        self.map.iter()
            .all(|((row, column), state)| state == other.state_at(*row, *column).unwrap())
    }
}

impl Eq for Grid {}
