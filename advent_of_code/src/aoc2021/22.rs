#[macro_use] extern crate lazy_static;
extern crate regex;

use std::collections::{HashSet, hash_map::RandomState};

use advent_of_code::get_str_array_from_file;
use regex::{ Regex };

type Point = (isize, isize, isize);
type Cubes = HashSet<Point, RandomState>;

#[derive(Debug, PartialEq, Eq)]
struct Cuboid {
  x_min: isize,
  x_max: isize,
  y_min: isize,
  y_max: isize,
  z_min: isize,
  z_max: isize,
  flag: bool
}

fn get_cuboid(text: &str) -> Cuboid {
  lazy_static! {
    static ref RE_CUBOID: Regex = Regex::new(
      r"(?P<flag>on|off)\sx=(?P<x_min>-?\d+)..(?P<x_max>-?\d+),y=(?P<y_min>-?\d+)..(?P<y_max>-?\d+),z=(?P<z_min>-?\d+)..(?P<z_max>-?\d+)"
    ).unwrap();
  }
  let capture = RE_CUBOID.captures(text).unwrap();
  let flag = capture.name("flag").unwrap().as_str() == "on";
  let x_min = capture.name("x_min").unwrap().as_str().parse().unwrap();
  let x_max = capture.name("x_max").unwrap().as_str().parse().unwrap();
  let y_min = capture.name("y_min").unwrap().as_str().parse().unwrap();
  let y_max = capture.name("y_max").unwrap().as_str().parse().unwrap();
  let z_min = capture.name("z_min").unwrap().as_str().parse().unwrap();
  let z_max = capture.name("z_max").unwrap().as_str().parse().unwrap();
  Cuboid { x_min, x_max, y_min, y_max, z_min, z_max, flag }
}

#[test]
fn test_get_cuboid() {
  let result = get_cuboid("on x=-41..9,y=-7..43,z=-33..15");
  let expected = Cuboid {
    flag: true,
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
            if cuboid.flag {
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

fn main() {
  let data = get_str_array_from_file(&vec!{"aoc2021", "data", "22.txt"});
  let steps = data.iter().map(|text| get_cuboid(text)).collect();
  let cubes = reboot(&Cuboid {
    x_min: -50,
    x_max: 50,
    y_min: -50,
    y_max: 50,
    z_min: -50,
    z_max: 50,
    flag: true,
  }, &steps);
  println!("Part 1: {}", cubes.len());
}