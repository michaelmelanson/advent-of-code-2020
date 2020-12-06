

#[derive(Debug, PartialEq)]
pub enum Instruction {
  Left, Right,
  Front, Back
}

#[derive(Debug, PartialEq)]
pub struct Ticket(Vec<Instruction>);

impl Ticket {
  fn seat_id(&self) -> usize {
    let mut row = 0;
    let mut column = 0;

    for instruction in &self.0 {
      match instruction {
        Instruction::Front => { row = (row << 1) + 0; },
        Instruction::Back => { row = (row << 1) + 1; },
        Instruction::Left => { column = (column << 1) + 0; },
        Instruction::Right => { column = (column << 1) + 1; },
      }
    }

    (row * 8) + column
  }
}

#[test]
fn test_ticket_seat_id() {
  assert_eq!(
    Ticket(vec![
      Instruction::Front,
      Instruction::Back,
      Instruction::Front,
      Instruction::Back,
      Instruction::Back,
      Instruction::Front,
      Instruction::Front,
      Instruction::Right,
      Instruction::Left,
      Instruction::Right,
    ]).seat_id(),
    357
  );
}

#[aoc_generator(day5)]
pub fn parse_tickets(input: &str) -> Vec<Ticket> {
  let mut tickets = Vec::new();

  for line in input.lines() {
    let mut ticket = Ticket(Vec::new());

    for c in line.chars() {
      let instruction = match c {
        'L' => Instruction::Left,
        'R' => Instruction::Right,
        'F' => Instruction::Front,
        'B' => Instruction::Back,
        _ => panic!("unknown instruction {}", c)
      };

      ticket.0.push(instruction);
    }

    tickets.push(ticket);
  }

  tickets
}

#[aoc(day5, part1)]
fn find_biggest_seat_id(input: &Vec<Ticket>) -> usize {
  input.iter()
    .map(|ticket| ticket.seat_id())
    .max()
    .unwrap()
}


#[aoc(day5, part2)]
fn find_our_seat(input: &Vec<Ticket>) -> usize {
  let mut seat_ids = input.iter()
    .map(|ticket| ticket.seat_id())
    .collect::<Vec<_>>();
  seat_ids.sort();
  
  let offset = seat_ids[0];

  seat_ids.iter()
    .enumerate()
    .find(|(index, seat_id)| *index != **seat_id - offset )
    .unwrap()
    .1 /* the seat ID */ - 1
}
