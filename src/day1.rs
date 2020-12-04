// Check out the post that accompanies this.
// https://michaelmelanson.substack.com/p/advent-of-code-2020-day-1

use std::str::FromStr;

#[aoc_generator(day1)]
pub fn parse_expense_report(input: &str) -> Vec<u64> {
  input // take the input
    .lines() // split it into lines
    .map(|line| u64::from_str(line).unwrap()) // turn it into numbers
    .collect::<Vec<_>>() // collect the list into a vector
}

#[test]
fn test_generator() {
  assert_eq!(
    parse_expense_report("55\n42\n1234\n10"),
    vec![55, 42, 1234, 10]
  );
}


#[aoc(day1, part1)]
pub fn find_2020(input: &Vec<u64>) -> u64 {
  for (first_index, first) in input.iter().enumerate() {
    for second in input[first_index+1..].iter() {
      if first + second == 2020 {
        return first * second;
      }
    }
  }

  panic!("could not find solution");
}


#[aoc(day1, part2)]
pub fn find_three_numbers(input: &Vec<u64>) -> u64 {
  for (first_index, first) in input.iter().enumerate() {
    for (second_index, second) in input[first_index+1..].iter().enumerate() {
      for third in input[second_index+1..].iter() {
        if first + second + third == 2020 {
          return first * second * third;
        }
      }
    }
  }

  panic!("could not find solution");
}
