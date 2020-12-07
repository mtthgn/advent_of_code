use std::fs;

struct Passport {
 birth_year: Option<usize>,
 issue_year: Option<usize>,
 expiration_year: Option<usize>,
 height: Option<String>,
 hair_color: Option<String>,
 eye_color: Option<String>,
 passport_id: Option<String>,
 country_id: Option<String>
}

impl Passport {
    fn all_fields_present(&self) -> bool {
        return self.birth_year.is_some() &&
            self.issue_year.is_some() &&
            self.expiration_year.is_some() &&
            self.height.is_some() &&
            self.hair_color.is_some() &&
            self.eye_color.is_some() &&
            self.passport_id.is_some();
    }

    fn all_fields_valid(&self) -> bool {
        return self.valid_birth_year() &&
            self.valid_issue_year() &&
            self.valid_expiration_year() &&
            self.valid_height() &&
            self.valid_hair_color() &&
            self.valid_eye_color() &&
            self.valid_passport_id();
    }

    fn valid_birth_year(&self) -> bool {
        if self.birth_year.is_none() {
            return false;
        }

        return (1920..=2002).contains(&self.birth_year.unwrap());
    }

    fn valid_issue_year(&self) -> bool {
        if self.issue_year.is_none() {
            return false;
        }

        return (2010..=2020).contains(&self.issue_year.unwrap());
    }

    fn valid_expiration_year(&self) -> bool {
        if self.expiration_year.is_none() { return false; }

        return (2020..=2030).contains(&self.expiration_year.unwrap());
    }

    fn valid_height(&self) -> bool {
        if self.height.is_none() { return false; }

        let height : &String = self.height.as_ref().unwrap();
        let length = height.len();

        if length < 2 { return false; }

        let unit = &height[(length- 2)..length];
        let digit_string = &height[0..(length - 2)];
        let digit = match digit_string.parse::<usize>() {
            Ok(n) => n,
            Err(_e) => 0
        };

        match unit {
            "in" => (59..=76).contains(&digit),
            "cm" => (150..=193).contains(&digit),
            _ => false
        }
    }

    fn valid_hair_color(&self) -> bool {
        if self.hair_color.is_none() { return false; }

        let hair_color : &String = self.hair_color.as_ref().unwrap();

        let length = hair_color.len();
        if length != 7 { return false; }

        let mut characters = hair_color.chars();
        let first_character = characters.next().unwrap();
        if first_character != '#' { return false; }

        return characters.all(|character| {
            ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f'].iter().any(|c| c == &character)
        });
    }

    fn valid_eye_color(&self) -> bool {
        if self.eye_color.is_none() { return false; }

        let eye_color : &String = self.eye_color.as_ref().unwrap();

        return ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].iter().any(|color| color == eye_color);
    }

    fn valid_passport_id(&self) -> bool {
        if self.passport_id.is_none() { return false; }

        let passport_id : &String = self.passport_id.as_ref().unwrap();

        let length = passport_id.len();
        if length != 9 { return false; }

        return passport_id.chars().all(|character| {
            ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'].iter().any(|c| c == &character)
        });
    }
}

pub fn run_part_one() {
    let mut valid_count = 0;

    parse_and_transform(|passport: Passport|{
        if passport.all_fields_present() {
            valid_count += 1;
        }
    });

    println!("valid count: {}", valid_count);
}

pub fn run_part_two() {
    let mut valid_count = 0;

    parse_and_transform(|passport: Passport|{
        if passport.all_fields_valid() {
            valid_count += 1;
        }
    });

    println!("valid count: {}", valid_count);
}

fn parse_and_transform<F>(mut callback: F) where F: FnMut(Passport) {
    let contents = fs::read_to_string("./src/day_four/input.txt")
        .expect("Unable to read file");

    contents.split("\n\n").for_each(|passport_blob| {
        let mut passport = Passport {
            birth_year: None,
            issue_year: None,
            expiration_year: None,
            height: None,
            hair_color: None,
            eye_color: None,
            passport_id: None,
            country_id: None
        };

        passport_blob.lines()
            .flat_map(|line| line.split_whitespace())
            .map(|field_string| {
                let mut pair = field_string.split(":");
                let field = pair.next().unwrap();
                let value = pair.next().unwrap().to_string();

                return (field, value);
            })
            .for_each(|(field, value)|{
                match field {
                    "byr" => passport.birth_year = Some(value.parse::<usize>().unwrap()),
                    "iyr" => passport.issue_year = Some(value.parse::<usize>().unwrap()),
                    "eyr" => passport.expiration_year = Some(value.parse::<usize>().unwrap()),
                    "hgt" => passport.height = Some(value),
                    "hcl" => passport.hair_color = Some(value),
                    "ecl" => passport.eye_color = Some(value),
                    "pid" => passport.passport_id = Some(value),
                    "cid" => passport.country_id = Some(value),
                    _ => println!("Unrecognized field: {}", field)
                }
            });

        callback(passport);
    })
}
