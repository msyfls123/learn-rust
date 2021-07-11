use advent_of_code::get_str_array_from_file;

type Direction = (isize, isize);

#[derive(Debug)]
struct Ship {
  direction: Direction,
  position: (isize, isize),
}

fn turn(direction: &Direction, clockwise: bool, count: isize) -> Direction {
  (0..count % 4).fold(direction.to_owned(), |acc, _| {
    if clockwise {
      (acc.1, -acc.0)
    } else {
      (-acc.1, acc.0)
    }
  })
}

fn navigate(ship: &Ship, instruction: &str) -> Ship {
  let (first, last) = instruction.split_at(1);
  let num = last.to_string().parse::<isize>().unwrap();
  let Ship { position, direction } = ship;
  let (x, y) = position;
  let (vx, vy) = direction;
  match first {
    "N" => Ship { position: (*x, y + num), ..*ship },
    "S" => Ship { position: (*x, y - num), ..*ship },
    "E" => Ship { position: (x + num, *y), ..*ship },
    "W" => Ship { position: (x - num, *y), ..*ship },
    "L" => Ship { direction: turn(&direction, false, num / 90), ..*ship },
    "R" => Ship { direction: turn(&direction, true, num / 90), ..*ship },
    "F" => Ship { position: (x + num * vx, y + num * vy), ..*ship },
    _ => panic!("not covered"),
  }
}

fn navigate_waypoint(ship: &Ship, instruction: &str) -> Ship {
  let (first, last) = instruction.split_at(1);
  let num = last.to_string().parse::<isize>().unwrap();
  let Ship { position, direction } = ship;
  let (x, y) = position;
  let (vx, vy) = direction;
  match first {
    "N" => Ship { direction: (*vx, vy + num), ..*ship },
    "S" => Ship { direction: (*vx, vy - num), ..*ship },
    "E" => Ship { direction: (vx + num, *vy), ..*ship },
    "W" => Ship { direction: (vx - num, *vy), ..*ship },
    "L" => Ship { direction: turn(&direction, false, num / 90), ..*ship },
    "R" => Ship { direction: turn(&direction, true, num / 90), ..*ship },
    "F" => Ship { position: (x + num * vx, y + num * vy), ..*ship },
    _ => panic!("not covered"),
  }
}

fn main() {
  let instructions = get_str_array_from_file(&vec!{"aoc2020", "data", "12.txt"});
  let location = instructions.iter().fold(Ship { direction: (1, 0), position: (0, 0)}, |ship, instruction| {
    navigate(&ship, instruction)
  }).position;
  println!("Part 1: {}", location.0.abs() + location.1.abs());
  let location = instructions.iter().fold(Ship { direction: (10, 1), position: (0, 0)}, |ship, instruction| {
    navigate_waypoint(&ship, instruction)
  }).position;
  println!("Part 2: {}", location.0.abs() + location.1.abs());
}
