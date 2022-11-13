use advent_of_code::get_str_from_file;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, PartialEq)]
struct Area {
  x_min: isize,
  x_max: isize,
  y_min: isize,
  y_max: isize,
}

fn get_area(text: &str) -> Area {
  lazy_static! {
    static ref RE_AREA: Regex = Regex::new(r"x=(?P<x_min>[-\d]+)\.\.(?P<x_max>[-\d]+),\sy=(?P<y_min>[-\d]+)\.\.(?P<y_max>[-\d]+)").unwrap();
  }
  let cap = RE_AREA.captures(text).unwrap();
  let x_min = cap.name("x_min").unwrap().as_str().parse().unwrap();
  let x_max = cap.name("x_max").unwrap().as_str().parse().unwrap();
  let y_min = cap.name("y_min").unwrap().as_str().parse().unwrap();
  let y_max = cap.name("y_max").unwrap().as_str().parse().unwrap();
  Area {
    x_max,
    x_min,
    y_max,
    y_min
  }
}

#[test]
fn test_get_area() {
  assert_eq!(get_area("target area: x=20..30, y=-10..-5"), Area {
    x_min: 20,
    x_max: 30,
    y_min: -10,
    y_max: -5
  })
}

fn main() {
  let data = get_str_from_file(&vec!{"aoc2021", "data", "17.txt"});
  let area = get_area(&data);

  let vy = - 1 - area.y_min;
  let highest_y_position = (vy + 1) * vy / 2;
  println!("Part 1: {}", highest_y_position);
}