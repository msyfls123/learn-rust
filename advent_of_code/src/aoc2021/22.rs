#[macro_use] extern crate lazy_static;
extern crate regex;

use std::{collections::{HashSet, hash_map::RandomState}, fmt::Display};

use advent_of_code::get_str_array_from_file;
use regex::{ Regex };

type Point = (isize, isize, isize);
type Cubes = HashSet<Point, RandomState>;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Cuboid {
  x_min: isize,
  x_max: isize,
  y_min: isize,
  y_max: isize,
  z_min: isize,
  z_max: isize,
  op: isize
}

impl Cuboid {
  fn intersect(&self, other: &Self) -> Option<Self> {
    let x_min = self.x_min.max(other.x_min);
    let x_max = self.x_max.min(other.x_max);
    let y_min = self.y_min.max(other.y_min);
    let y_max = self.y_max.min(other.y_max);
    let z_min = self.z_min.max(other.z_min);
    let z_max = self.z_max.min(other.z_max);
    if x_min > x_max || y_min > y_max || z_min > z_max {
      return None
    }
    Some(Cuboid {
      x_min, x_max,
      y_min, y_max,
      z_min, z_max,
      op: -other.op
    })
  }

  fn volume(&self) -> isize {
    (self.x_max - self.x_min + 1) *
    (self.y_max - self.y_min + 1) *
    (self.z_max - self.z_min + 1) *
    self.op
  }
}

impl Display for Cuboid {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{} > x:{}..{},y:{}..{},z:{}..{}", self.op, self.x_min, self.x_max, self.y_min, self.y_max, self.z_min, self.z_max)
  }
}

impl From<&String> for Cuboid {
  fn from(text: &String) -> Self {
    lazy_static! {
      static ref RE_CUBOID: Regex = Regex::new(
        r"(?P<op>on|off)\sx=(?P<x_min>-?\d+)..(?P<x_max>-?\d+),y=(?P<y_min>-?\d+)..(?P<y_max>-?\d+),z=(?P<z_min>-?\d+)..(?P<z_max>-?\d+)"
      ).unwrap();
    }
    let capture = RE_CUBOID.captures(text).unwrap();
    let op = if capture.name("op").unwrap().as_str() == "on" { 1 } else { -1 };
    let x_min = capture.name("x_min").unwrap().as_str().parse().unwrap();
    let x_max = capture.name("x_max").unwrap().as_str().parse().unwrap();
    let y_min = capture.name("y_min").unwrap().as_str().parse().unwrap();
    let y_max = capture.name("y_max").unwrap().as_str().parse().unwrap();
    let z_min = capture.name("z_min").unwrap().as_str().parse().unwrap();
    let z_max = capture.name("z_max").unwrap().as_str().parse().unwrap();
    Self { x_min, x_max, y_min, y_max, z_min, z_max, op }
  }
}

#[test]
fn test_get_cuboid() {
  let result: Cuboid = (&String::from("on x=-41..9,y=-7..43,z=-33..15")).into();
  let expected = Cuboid {
    op: 1,
    x_min: -41,
    x_max: 9,
    y_min: -7,
    y_max: 43,
    z_min: -33,
    z_max: 15,
  };
  assert_eq!(result, expected);
}

fn reboot(region: &Cuboid, steps: &Vec<Cuboid>) -> Cubes {
  let mut cubes = HashSet::new();
  for x in region.x_min..=region.x_max {
    for y in region.y_min..=region.y_max {
      for z in region.z_min..=region.z_max {
        for cuboid in steps {
          let &Cuboid { x_min, x_max, y_min, y_max, z_min, z_max, ..} = cuboid;
          if x_min <= x && x_max >= x &&
            y_min <= y && y_max >= y &&
            z_min <= z && z_max >= z
          {
            if cuboid.op == 1 {
              cubes.insert((x, y, z));
            } else {
              cubes.remove(&(x, y, z));
            }
          }
        }
      }
    }
  }
  cubes
}

fn reboot2(cuboids: &Vec<Cuboid>) -> Vec<Cuboid> {
  let mut cubes:  Vec<Cuboid> = vec!{};
  for cuboid in cuboids {
      let mut merged = cubes.iter().filter_map(|c| cuboid.intersect(c)).collect();
      cubes.append(&mut merged);
      if cuboid.op == 1 {
        cubes.push(cuboid.clone());
      }
  }
  cubes
}

fn main() {
  let data = get_str_array_from_file(&vec!{"aoc2021", "data", "22.txt"});
  let steps = data.iter().map(|text| text.into()).collect();
  let cubes = reboot(&Cuboid {
    x_min: -50,
    x_max: 50,
    y_min: -50,
    y_max: 50,
    z_min: -50,
    z_max: 50,
    op: 1,
  }, &steps);
  println!("Part 1: {}", cubes.len());
  let cubes_count: isize = reboot2(&steps).iter().map(|c| c.volume()).sum();
  println!("Part 2: {}", cubes_count);
}