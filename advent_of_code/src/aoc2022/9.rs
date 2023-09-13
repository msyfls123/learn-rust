use advent_of_code::get_str_array_from_file;
use itertools::Itertools;
use std::convert::TryInto;
use std::collections::HashSet;

type Point = (isize, isize);
type VisitedPositions = HashSet<Point>;


fn calc_distance(point_a: &Point, point_b: &Point) -> usize {
  ((point_a.0 - point_b.0).abs() + (point_a.1 - point_b.1).abs()).try_into().unwrap()
}

fn get_unitary_step(point_a: &Point, point_b: &Point) -> Point {
  let x = point_b.0 - point_a.0;
  let y = point_b.1 - point_a.1;
  let x_step = x.checked_div(x.abs()).unwrap_or_default();
  let y_step = y.checked_div(y.abs()).unwrap_or_default();
  (x_step, y_step)
}

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

  fn step(&self) -> Point {
    match self.direction {
      Direction::Left => {
        (-1, 0)
      },
      Direction::Right => {
        (1, 0)
      },
      Direction::Up => {
        (0, -1)
      },
      Direction::Down => {
        (0, 1)
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


#[derive(Debug)]
struct Rope {
  knots: Vec<Point>,
  length: usize
}

impl Rope {
  fn create(length: usize) -> Self {
    Self {
      knots: vec!((0,0); length),
      length,
    }
  }

  fn run_motion(&mut self, motion: &Motion) -> VisitedPositions {
    let mut visited = HashSet::new();
    visited.insert(*self.knots.last().unwrap());

    for _ in 0..motion.step {
      self.step(motion);
      visited.insert(*self.knots.last().unwrap());
    }
    visited
  }

  fn step(&mut self, motion: &Motion) {
    let mut prev_point = (0, 0);
    for (i, knot) in self.knots.iter_mut().enumerate() {
      if i == 0 {
        let step = motion.step();
        *knot = (&knot.0 + step.0, &knot.1 + step.1);
      } else {
        let distance = calc_distance(knot, &prev_point);

        // directly or diagonally adjacent, don't move
        if (distance <= 1) || (knot.0 != prev_point.0 && knot.1 != prev_point.1 && distance == 2) {
          // do nothing
        } else {
          // not adjacent
          let step = get_unitary_step(knot, &prev_point);
          *knot = (&knot.0 + step.0, &knot.1 + step.1);
        }

      }
      prev_point = knot.to_owned();
    }
  }
}

#[test]
fn test_rope_step() {
  let mut rope = Rope::create(8);
  let motion = Motion::from_text("R 50");
  rope.step(&motion);
  assert_eq!(rope.knots[0], (1,0));
}

fn main() {
  let data = get_str_array_from_file(&vec!{"aoc2022", "data", "9.txt"});
  let motions: Vec<Motion> = data.iter().map(|text| Motion::from_text(text)).collect();

  let mut rope1 = Rope::create(2);
  let visited_positions = motions.iter().fold(HashSet::new(), |mut visited: VisitedPositions, m| {
    visited.extend(&rope1.run_motion(m));
    visited
  });
  println!("Part 1: {}", visited_positions.len());

  let mut rope2 = Rope::create(10);
  let visited_positions = motions.iter().fold(HashSet::new(), |mut visited: VisitedPositions, m| {
    visited.extend(&rope2.run_motion(m));
    visited
  });
  println!("Part 2: {}", visited_positions.len());
}
