#[macro_use] extern crate lazy_static;
extern crate regex;

use regex::{ Regex };
use advent_of_code::get_str_array_from_file;
use std::collections::HashMap;

#[derive(Debug)]
struct Rectangle {
  x: i32,
  y: i32,
  width: i32,
  height: i32,
}

fn get_claim(text: &str) -> Rectangle {
  lazy_static! {
    static ref RE_DISTANCE: Regex = Regex::new(r"#(?P<id>\d+)\s@\s(?P<x>\d+),(?P<y>\d+):\s(?P<width>\d+)x(?P<height>\d+)").unwrap();
  }
  let captured = RE_DISTANCE.captures(text).unwrap();
  let x = captured.name("x").unwrap().as_str().parse::<i32>().unwrap();
  let y = captured.name("y").unwrap().as_str().parse::<i32>().unwrap();
  let width = captured.name("width").unwrap().as_str().parse::<i32>().unwrap();
  let height = captured.name("height").unwrap().as_str().parse::<i32>().unwrap();
  Rectangle {
    x,
    y,
    width,
    height,
  }
}

fn main() {
  let array = get_str_array_from_file(&vec!{"aoc2018", "data", "3.txt"});
  let claims: Vec<Rectangle> = array.iter().map(|x| get_claim(x)).collect();
  let mut map: HashMap<(i32, i32), i32> = HashMap::new();

  claims.iter().for_each(|rec| {
    (rec.x..(rec.x + rec.width)).for_each(|x| {
      (rec.y..(rec.y + rec.height)).for_each(|y| {
        *map.entry((x, y)).or_insert(0) += 1;
      });
    });
  });
  let count = map.values().filter(|&x| *x > 1).count();
  println!("Part 1: {}", count);
}
