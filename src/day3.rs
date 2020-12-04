use std::collections::HashSet;

#[derive(Debug, PartialEq)]
pub struct Map {
  trees: HashSet<(usize, usize)>,
  width: usize,
  height: usize
}

#[aoc_generator(day3)]
pub fn parse_map(input: &str) -> Map {
  let mut trees = HashSet::new();
  let mut width = 0;
  let mut height = 0;

  for (y, row_text) in input.lines().enumerate() {
    for (x, cell) in row_text.chars().enumerate() {
      let has_tree = cell == '#';
      if has_tree {
        trees.insert((x, y));
      }

      if x >= width {
        width = x + 1;
      }

      if y >= height {
        height = y + 1;
      }
    }
  }

  Map { trees, width, height }
}

#[test]
fn test_parse_map() {
  let mut expected_trees = HashSet::new();
  expected_trees.insert((1, 0));
  expected_trees.insert((0, 1));
  expected_trees.insert((2, 1));
  expected_trees.insert((1, 2));
  expected_trees.insert((2, 2));

  assert_eq!(
    parse_map(".#.\n#.#\n.##"),
    Map { trees: expected_trees, width: 3, height: 3 }
  )
}

#[aoc(day3, part1)]
fn part1(map: &Map) -> usize {
  count_hit_trees(map, (0, 0), (3, 1))
}

fn count_hit_trees(
  map: &Map, 
  start: (usize, usize), 
  velocity: (usize, usize)
) -> usize {
  let mut trees_hit = 0;
  let mut position = start;

  while position.1 < map.height {
    if map.trees.contains(&(position.0 % map.width, position.1)) {
      trees_hit += 1;
    }

    position.0 += velocity.0;
    position.1 += velocity.1;
  }

  trees_hit
}

#[aoc(day3, part2)]
fn part2(map: &Map) -> usize {
  count_hit_trees(map, (0, 0), (1, 1))
  * count_hit_trees(map, (0, 0), (3, 1))
  * count_hit_trees(map, (0, 0), (5, 1))
  * count_hit_trees(map, (0, 0), (7, 1))
  * count_hit_trees(map, (0, 0), (1, 2))
}
