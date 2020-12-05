use std::str::FromStr;
use std::collections::HashMap;

#[allow(unused_macros)]
macro_rules! collection {
  // map-like
  ($($k:expr => $v:expr),* $(,)?) => {
      std::iter::Iterator::collect(std::array::IntoIter::new([$(($k, $v),)*]))
  };
  // set-like
  ($($v:expr),* $(,)?) => {
      std::iter::Iterator::collect(std::array::IntoIter::new([$($v,)*]))
  };
}

#[derive(Clone, Default, Debug, PartialEq)]
pub struct Passport(HashMap<Field, String>);

impl Passport {
  pub fn contains_field(&self, field: Field) -> bool {
    self.0.contains_key(&field)
  }

  pub fn is_valid(&self) -> bool {
    if !self.contains_field(Field::BirthYear) { return false; }
    if !self.contains_field(Field::IssueYear) { return false; }
    if !self.contains_field(Field::ExpirationYear) { return false; }
    if !self.contains_field(Field::Height) { return false; }
    if !self.contains_field(Field::HairColour) { return false; }
    if !self.contains_field(Field::EyeColour) { return false; }
    if !self.contains_field(Field::PassportID) { return false; }
    // if !self.contains_field(Field::CountryID) { return false; }

    true
  }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Field {
  BirthYear,
  IssueYear,
  ExpirationYear,
  Height,
  HairColour,
  EyeColour,
  PassportID,
  CountryID
}

#[aoc_generator(day4)]
pub fn parse_passports(input: &str) -> Vec<Passport> {
  let mut passports = Vec::new();

  let mut current_passport = Passport::default();
  for line in input.lines() {
    if line.is_empty() {
      passports.push(current_passport.clone());
      current_passport = Passport::default();
      continue;
    }

    for field in line.split(" ") {
      let mut parts = field.split(":");
      let key = parts.next().unwrap();
      let value = parts.next().unwrap();

      let key = match key {
        "byr" => Field::BirthYear,
        "iyr" => Field::IssueYear,
        "eyr" => Field::ExpirationYear,
        "hgt" => Field::Height,
        "hcl" => Field::HairColour,
        "ecl" => Field::EyeColour,
        "pid" => Field::PassportID,
        "cid" => Field::CountryID,
        _ => panic!("unknown field key {}", key)
      };

      current_passport.0.insert(key, value.to_string());
    }
  }

  passports.push(current_passport.clone());

  passports
}

#[test]
fn test_parse_passports() {
  assert_eq!(
    parse_passports("ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 cid:147 hgt:183cm\n\niyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\nhcl:#cfa07d byr:1929"),
    vec![
      Passport(collection! {
        Field::EyeColour => "gry".to_string(),
        Field::PassportID => "860033327".to_string(),
        Field::ExpirationYear => "2020".to_string(),
        Field::HairColour => "#fffffd".to_string(),
        Field::BirthYear => "1937".to_string(),
        Field::IssueYear => "2017".to_string(),
        Field::CountryID => "147".to_string(),
        Field::Height => "183cm".to_string()
      }),

      Passport(collection! {
        Field::IssueYear => "2013".to_string(),
        Field::EyeColour => "amb".to_string(),
        Field::CountryID => "350".to_string(),
        Field::ExpirationYear => "2023".to_string(),
        Field::PassportID => "028048884".to_string(),
        Field::HairColour => "#cfa07d".to_string(),
        Field::BirthYear => "1929".to_string()
      }),
    ]
  )
}

#[test]
fn test_passport_is_valid() {
  assert!(Passport(collection! {
    Field::EyeColour => "gry".to_string(),
    Field::PassportID => "860033327".to_string(),
    Field::ExpirationYear => "2020".to_string(),
    Field::HairColour => "#fffffd".to_string(),
    Field::BirthYear => "1937".to_string(),
    Field::IssueYear => "2017".to_string(),
    Field::CountryID => "147".to_string(),
    Field::Height => "183cm".to_string()
  }).is_valid());

  assert!(!Passport(collection! {
    Field::IssueYear => "2013".to_string(),
    Field::EyeColour => "amb".to_string(),
    Field::CountryID => "350".to_string(),
    Field::ExpirationYear => "2023".to_string(),
    Field::PassportID => "028048884".to_string(),
    Field::HairColour => "#cfa07d".to_string(),
    Field::BirthYear => "1929".to_string()
  }).is_valid());

  assert!(Passport(collection! {
    Field::HairColour => "#ae17e1".to_string(),
    Field::IssueYear => "2013".to_string(),
    Field::ExpirationYear => "2024".to_string(),
    Field::EyeColour => "brn".to_string(),
    Field::PassportID => "760753108".to_string(),
    Field::BirthYear => "1931".to_string(),
    Field::Height => "179cm".to_string()
  }).is_valid());
}

#[aoc(day4, part1)]
pub fn count_valid_passports(passports: &Vec<Passport>) -> usize {
  let mut valid_passports = 0;

  for passport in passports {
    if passport.is_valid() {
      valid_passports += 1;
    }
  }

  valid_passports
}

#[aoc(day4, part2)]
pub fn count_actually_valid_passports(passports: &Vec<Passport>) -> usize {
  let mut valid_passports = 0;

  'next_passport: for passport in passports {
    if !passport.is_valid() {
      continue;
    }

    for (field, value) in passport.0.iter() {
      match field {
        Field::BirthYear => {
          if !is_number_in_range(value, 1920, 2002) {
            continue 'next_passport;
          }
        },

        Field::IssueYear => {
          if !is_number_in_range(value, 2010, 2020) {
            continue 'next_passport;
          }
        },

        Field::ExpirationYear => {
          if !is_number_in_range(value, 2020, 2030) {
            continue 'next_passport;
          }
        },

        Field::Height => {
          if let Some(cm) = value.strip_suffix("cm") {
            if !is_number_in_range(cm, 150, 193) {
              continue 'next_passport;
            }  
          } else if let Some(inches) = value.strip_suffix("in") {
            if !is_number_in_range(inches, 59, 76) {
              continue 'next_passport;
            }  
          } else {
            continue 'next_passport;
          }
        },

        Field::HairColour => {
          if !value.starts_with("#") {
            continue 'next_passport;
          }

          if !value[1..].chars().all(|c| "0123456789abcdef".contains(c)) {
            continue 'next_passport;
          }
        },

        Field::EyeColour => {
          if !["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&value.as_str()) {
            continue 'next_passport;
          }
        },

        Field::PassportID => {
          if value.len() != 9 {
            continue 'next_passport;
          }

          if !value.chars().all(|c| "0123456789".contains(c)) {
            continue 'next_passport;
          }
        },

        Field::CountryID => {}
      }
    }

    valid_passports += 1;
  }

  valid_passports
}

fn is_number_in_range(value: &str, min: usize, max: usize) -> bool {
  if let Ok(value) = usize::from_str(value) {
    if value < min || value > max { return false; }
  } else {
    return false;
  }
  return true;
}