// use core::cmp::*;
use advent_of_code::get_str_array_from_file;

#[derive(Debug)]
enum Group {
  Goblin,
  Elf
}

type Position = (usize, usize);

#[derive(Debug)]
struct Unit {
  pos: Position,
  group: Group,
  hit_points: usize
}

#[derive(Debug)]
struct Wall {
  pos: Position
}

#[derive(Debug)]
struct OpenCavern {
  pos: Position
}

#[derive(Debug)]
enum Area {
  Unit(Unit),
  Wall(Wall),
  OpenCavern(OpenCavern),
}

// impl Ord for Unit {
//   fn cmp(&self, other: &Self) -> Ordering {
//     self.pos.cmp(&other.pos)
//   }
// }

// impl PartialOrd for Unit {
//   fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//       Some(self.pos.cmp(&other.pos))
//   }
// }

// impl PartialEq for Unit {
//   fn eq(&self, other: &Self) -> bool {
//       self.pos == other.pos
//   }
// }

// impl Eq for Unit {}

fn get_pos(area: &Area) -> Position {
  match area {
    Area::Unit(unit) => unit.pos,
    Area::Wall(wall) => wall.pos,
    Area::OpenCavern(open_cavern) => open_cavern.pos,
  }
}

fn get_area(y: usize, x: usize, char: char) -> Area {
  match char {
    '#' => Area::Wall(Wall {
      pos: (y, x),
    }),
    '.' => Area::OpenCavern(OpenCavern {
      pos: (y, x),
    }),
    'G' => Area::Unit(Unit {
      pos: (y, x),
      group: Group::Goblin,
      hit_points: 200,
    }),
    'E' => Area::Unit(Unit {
      pos: (y, x),
      group: Group::Elf,
      hit_points: 200,
    }),
    _ => panic!("This char is not implemented as Area!")
  }
}

fn main() {
  let data = get_str_array_from_file(&vec!{"aoc2018", "data", "15.txt"});
  let areas: Vec<Vec<Area>> = data.iter().enumerate().map(|(y, line)| {
    line.chars().enumerate().map(|(x, c)| {
      get_area(y, x, c)
    }).collect()
  }).collect();
  println!("{:?}", areas[29][14]);
}
