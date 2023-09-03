use itertools::{Itertools, assert_equal};

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

fn parse_motion(text: &str) -> Motion {
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

#[test]
fn test_parse_motion() {
  assert_eq!(
    parse_motion("R 50"),
    Motion {
      direction: Direction::Right,
      step: 50
    }
  );
}

fn main() {
}