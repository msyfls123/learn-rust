use advent_of_code::get_str_array_from_file;
use itertools::Itertools;
use std::convert::TryInto;
use std::collections::HashSet;

type Point = (isize, isize);
type VisitedPositions = HashSet<Point>;

#[derive(Debug, Eq, PartialEq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Eq, PartialEq)]
struct Motion {
  direction: Direction,
  step: isize
}

impl Motion {
  fn from_text(text: &str) -> Self {
    let (dir, step) = text.split(" ").collect_tuple().unwrap();
    let mut direction = Direction::Up;
    match dir {
      "L" => {
        direction = Direction::Left;
      },
      "R" => {
        direction = Direction::Right;
      },
      "U" => {
        direction = Direction::Up;
      },
      "D" => {
        direction = Direction::Down
      },
      _ => panic!("no exists")
    }
    Motion { direction, step: step.parse::<isize>().unwrap() }
  }

  fn step(&self, head: &Point) -> Point {
    match self.direction {
      Direction::Left => {
        (head.0 - 1, head.1)
      },
      Direction::Right => {
        (head.0 + 1, head.1)
      },
      Direction::Up => {
        (head.0, head.1 - 1)
      },
      Direction::Down => {
        (head.0, head.1 + 1)
      },
    }
  }

  fn follow(&self, head: &Point, tail: &Point) -> Point {
    let distance = calc_distance(head, tail);
    if distance <= 1 {
      return *tail
    }
    if head.0 != tail.0 && head.1 != tail.1 && distance == 2 {
      return *tail
    }
    match self.direction {
      Direction::Left => {
        (head.0 + 1, head.1)
      },
      Direction::Right => {
        (head.0 - 1, head.1)
      },
      Direction::Up => {
        (head.0, head.1 + 1)
      },
      Direction::Down => {
        (head.0, head.1 - 1)
      },
    }
  }
}

#[test]
fn test_parse_motion() {
  assert_eq!(
    Motion::from_text("R 50"),
    Motion {
      direction: Direction::Right,
      step: 50
    }
  );
}

#[test]
fn test_follow() {
  assert_eq!(
    Motion {
      direction: Direction::Right,
      step: 50
    }.follow(&(0, 1), &(-1, 0)),
    (-1, 0)
  );
  assert_eq!(
    Motion {
      direction: Direction::Right,
      step: 50
    }.follow(&(255, 1), &(254, 1)),
    (254, 1)
  );
  assert_eq!(
    Motion {
      direction: Direction::Up,
      step: 50
    }.follow(&(1, -1), &(254, 1)),
    (1, 0)
  );
}

fn calc_distance(point_a: &Point, point_b: &Point) -> usize {
  ((point_a.0 - point_b.0).abs() + (point_a.1 - point_b.1).abs()).try_into().unwrap()
}

fn simulate(motions: &Vec<Motion>) -> VisitedPositions {
  let mut visited_positions = HashSet::default();
  let mut head = (0,0);
  let mut tail = (0,0);
  visited_positions.insert(tail);
  for motion in motions {
    for _ in 0..motion.step {
      head = motion.step(&head);
      tail = motion.follow(&head, &tail);
      visited_positions.insert(tail);
    }
  }
  visited_positions
}

fn main() {
  let data = get_str_array_from_file(&vec!{"aoc2022", "data", "9.txt"});
  let motions: Vec<Motion> = data.iter().map(|text| Motion::from_text(text)).collect();
  let visited_positions = simulate(&motions);
  println!("Part 1: {:?}", visited_positions.len());
}