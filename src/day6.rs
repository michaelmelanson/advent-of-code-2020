use std::collections::HashSet;

#[derive(Debug, PartialEq)]
pub struct Passenger(HashSet<char>);

#[derive(Debug, PartialEq)]
pub struct PassengerGroup(Vec<Passenger>);

#[aoc_generator(day6)]
pub fn parse_customs_forms(input: &str) -> Vec<PassengerGroup> {
  let mut groups = Vec::new();
  let mut current_group = PassengerGroup(Vec::new());

  for line in input.lines() {
    if line.is_empty() {
      groups.push(current_group);
      current_group = PassengerGroup(Vec::new());
      continue;
    }

    let mut passenger = Passenger(HashSet::new());
    for c in line.chars() {
      passenger.0.insert(c);
    }
    current_group.0.push(passenger);
  }

  groups.push(current_group);
  groups
}

#[aoc(day6, part1)]
pub fn count_group_answers(groups: &Vec<PassengerGroup>) -> usize {
  let mut total = 0;

  for group in groups {
    let mut answers = HashSet::new();

    for passenger in &group.0 {
      for answer in &passenger.0 {
        answers.insert(answer);
      }
    }

    total += answers.len();
  }

  total
}

#[aoc(day6, part2)]
pub fn count_group_answers_intersection(groups: &Vec<PassengerGroup>) -> usize {
  let mut total = 0;

  for group in groups {
    let mut answers = group.0.first().unwrap().0.clone();

    for passenger in &group.0 {
      answers = answers // start with the existing answers
        .intersection(&passenger.0) // remove anything not in this one
        .map(|c| *c) // deference `&char` into `char`
        .collect(); // collect the list back up into a HashSet
    }

    total += answers.len();
  }

  total
}
