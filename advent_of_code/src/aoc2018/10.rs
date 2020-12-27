#[macro_use] extern crate lazy_static;
extern crate regex;

use std::collections::HashMap;
use regex::{ Regex };
use advent_of_code::get_str_array_from_file;

type Position = (i32, i32);
type Velocity = (i32, i32);

struct Point {
  pos: Position,
  vel: Velocity
}

struct Bounds {
  top: i32,
  bottom: i32,
  left: i32,
  right: i32,
}

fn get_bounds(positions: &Vec<Position>) -> Bounds {
  let top = positions.iter().min_by_key(|&p| p.1).unwrap().1;
  let bottom = positions.iter().max_by_key(|&p| p.1).unwrap().1;
  let left = positions.iter().min_by_key(|&p| p.0).unwrap().0;
  let right = positions.iter().max_by_key(|&p| p.0).unwrap().0;
  Bounds { top, bottom, left, right }
}

struct Sky {
  points: Vec<Point>
}

impl Sky {
  fn get_point_positions_by_time(&self, seconds: i32) -> Vec<Position> {
    self.points.iter().map(|p| {
      let (x, y) = p.pos;
      let (vx, vy) = p.vel;
      (x + vx * seconds, y + vy * seconds)
    }).collect()
  }

  fn check_possible(&self, seconds: i32) -> bool {
    let positions = self.get_point_positions_by_time(seconds);
    let right = get_bounds(&positions).right;
    let mut map: HashMap<Position, bool> = HashMap::new();
    positions.iter().for_each(|&p| {
      map.entry(p).or_insert(true);
    });
    positions.iter().any(|&p| {
      (1..=2).all(|dx| {
        let x = p.0 + dx;
        let y = p.1;
        x <= right && map.get(&(x, y)) != None
      })
    })
  }

  fn print_message(&self, seconds: i32) {
    let positions = self.get_point_positions_by_time(seconds);
    let Bounds { top, bottom, left, right } = get_bounds(&positions);
    let mut map: HashMap<Position, bool> = HashMap::new();
    positions.iter().for_each(|&p| {
      map.entry(p).or_insert(true);
    });
    (top..bottom + 1).for_each(|y| {
      (left..right + 1).for_each(|x| {
        match map.get(&(x, y)) {
          Some(true) => {
            print!("*")
          },
          _ => {
            print!(" ")
          }
        };
      });
      println!("");
    })
  }
}

fn get_point(text: &str) -> Point {
  lazy_static!{
    static ref REG: Regex = Regex::new(r"position=<\s*(?P<x>[-\d]+),\s*(?P<y>[-\d]+)>\s*velocity=<\s*(?P<vx>[-\d]+),\s*(?P<vy>[-\d]+)>").unwrap();
  }
  let captured = REG.captures(text).unwrap();
  let x = captured.name("x").unwrap().as_str().parse::<i32>().unwrap();
  let y = captured.name("y").unwrap().as_str().parse::<i32>().unwrap();
  let vx = captured.name("vx").unwrap().as_str().parse::<i32>().unwrap();
  let vy = captured.name("vy").unwrap().as_str().parse::<i32>().unwrap();
  Point {
    pos: (x, y),
    vel: (vx, vy),
  }
}

// magic number, plz try yourself :)
static START_SECONDS: i32 = 10327;
static LIMIT: usize = 7;

fn main() {
  let array = get_str_array_from_file(&vec!["aoc2018", "data", "10.txt"]);
  let points: Vec<Point> = array.iter().map(|t| get_point(t)).collect();
  let sky = Sky { points };
  let mut seconds = START_SECONDS;
  let mut count = 0;
  loop {
    seconds += 1;
    if sky.check_possible(seconds) {
      println!("=== {} start ===", seconds);
      sky.print_message(seconds);
      println!("=== {} end ===", seconds);
      count += 1;
    }
    if count == LIMIT {
      break
    }
  }
}
