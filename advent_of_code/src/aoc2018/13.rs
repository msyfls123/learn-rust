use advent_of_code::get_str_array_from_file;
use std::collections::HashMap;

type Position = (isize, isize);

#[derive(Debug)]
struct Cart {
  speed: (isize, isize),
  position: Position,
  met_intersection: usize,
}

static TURNS_LIST: [char; 9] = ['/', '\\', '+', '-', '|', '^', 'v', '>', '<'];

impl Cart {
  fn meet_intersection(&self) -> (isize, isize) {
    let turns = (self.met_intersection + 2) % 4;
    let mut speed = self.speed.clone();
    (0..turns).for_each(|_| {
      speed = (-speed.1, speed.0)
    });
    speed
  }
  fn moves(&mut self, turns: &HashMap<Position, char>) {
    let (x, y) = self.position;
    let (vx, vy) = self.speed;
    self.position = (x + vx, y + vy);
    match turns.get(&self.position) {
      Some('+') => {
        self.met_intersection += 1;
        self.speed = self.meet_intersection();
      },
      Some('/') => {
        if vx != 0 {
          self.speed = (0, -vx)
        } else {
          self.speed = (-vy, 0)
        }
      },
      Some('\\') => {
        if vx != 0 {
          self.speed = (0, vx)
        } else {
          self.speed = (vy, 0)
        }
      },
      Some('|') => {},
      Some('-') => {},
      Some('^') => {},
      Some('v') => {},
      Some('>') => {},
      Some('<') => {},
      _ => {
        println!("{:?}", self.position);
        panic!("~~~");
      },
    }
  }
}

fn main() {
  let array = get_str_array_from_file(&vec!["aoc2018", "data", "13.txt"]);
  let map: Vec<Vec<char>> = array.iter().map(|text| text.chars().collect()).collect();
  let len = map.len();
  let mut carts: Vec<Cart> = vec![];
  let mut turns_map: HashMap<Position, char> = HashMap::new();
  map.iter().enumerate().for_each(|(y, row)| {
    row.iter().enumerate().for_each(|(x, column)| {
      if TURNS_LIST.contains(column) {
        turns_map.insert((x as isize, y as isize), *column);
      }
    });
  });
  map.iter().enumerate().for_each(|(y, row)| {
    row.iter().enumerate().for_each(|(x, column)| {
      let direction = match *column {
        '>' => Some((1, 0)),
        '<' => Some((-1, 0)),
        '^' => Some((0, -1)),
        'v' => Some((0, 1)),
        _ => None
      };
      match direction {
        Some(speed) => {
          let cart = Cart {
            speed,
            position: (x as isize, y as isize),
            met_intersection: 0,
          };
          carts.push(cart);
        },
        _ => {}
      }
    });
  });
  // println!("{:?}", carts);
  // println!("{:?}", turns_map);
  let mut crashed = false;
  let mut round = 0;
  while !crashed {
    carts.sort_by_key(|c| {
      c.position.1 * (len as isize) + c.position.0
    });
    // carts.reverse();
    round += 1;
    let mut positions: HashMap<Position, bool> = HashMap::new();
    carts.iter().for_each(|c| {
      positions.insert(c.position, true);
    });
    if round % 100 == 0 {
      println!("=== Round {}", round);
      println!("{:?}", carts.iter().map(|c| c.position));
    }
    let cart_len = carts.len();
    (0..cart_len).for_each(|i| {
      let cart = &mut carts[i];
      positions.remove(&cart.position);
      cart.moves(&turns_map);
      match positions.get(&cart.position) {
        Some(_a) => {
          crashed = true;
          println!("Crashed: {:?}", cart.position);
        },
        _ => {
          positions.insert(cart.position, true);
        }
      }
    });
  }
  
}
