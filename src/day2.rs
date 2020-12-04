// Check out the post that accompanies this.
// https://michaelmelanson.substack.com/p/advent-of-code-2020-day-2

use std::ops::RangeInclusive;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct PasswordEntry {
  letter: char,
  count: RangeInclusive<u8>,
  password: String
}

#[aoc_generator(day2)]
pub fn parse_password_entries(input: &str) -> Vec<PasswordEntry> {
  let lines = input.lines();
  let entries = lines.map(|line| {
    let mut parts = line.split(" ");
    let mut count_parts = parts.next().unwrap().split("-");
    let count_min = u8::from_str(count_parts.next().unwrap()).unwrap();
    let count_max = u8::from_str(count_parts.next().unwrap()).unwrap();
    let count = count_min..=count_max;

    let letter_part = parts.next().unwrap();
    let letter = letter_part.chars().nth(0).unwrap();

    let password = parts.next().unwrap().to_string();

    PasswordEntry { letter, count, password }
  });

  entries.collect::<Vec<_>>()
}

#[test]
fn test_parse_password_entries() {
  assert_eq!(
    parse_password_entries("3-11 z: zzzzzdzzzzlzz\n3-7 x: xjxbgpxxgtx\n3-4 v: vvmv\n"), 
    vec![
      PasswordEntry {
        count: 3..=11,
        letter: 'z',
        password: "zzzzzdzzzzlzz".to_string()
      },
      PasswordEntry {
        count: 3..=7,
        letter: 'x',
        password: "xjxbgpxxgtx".to_string()
      },
      PasswordEntry {
        count: 3..=4,
        letter: 'v',
        password: "vvmv".to_string()
      },
    ]);
}

#[aoc(day2, part1)]
fn count_valid_passwords(entries: &Vec<PasswordEntry>) -> u32 {
  let mut valid_passwords = 0;

  for entry in entries {
    let mut letter_count = 0; 
    for letter in entry.password.chars() {
      if letter == entry.letter {
        letter_count += 1;
      }
    }

    if entry.count.contains(&letter_count) {
      valid_passwords += 1;
    }
  }

  valid_passwords
}

#[aoc(day2, part2)]
fn count_actual_valid_passwords(entries: &Vec<PasswordEntry>) -> u32 {
  let mut valid_passwords = 0;

  for entry in entries {
    let first_position = *entry.count.start();
    let first_letter = entry.password.chars()
      .nth(first_position as usize - 1)
      .unwrap();

    let second_position = *entry.count.end();
    let second_letter = entry.password.chars()
      .nth(second_position as usize - 1)
      .unwrap();

    let is_valid = (first_letter == entry.letter)
                 ^ (second_letter == entry.letter);

    // println!(
    //   "{}: {} - looking for {}, first letter {} at position {}, second letter {} at position {}",
    //   if is_valid { "VALID" } else { "INVALID" },
    //   entry.password,
    //   entry.letter,
    //   first_letter, first_position,
    //   second_letter, second_position
    // );

    if is_valid {
      valid_passwords += 1;
    }
  }

  valid_passwords
}