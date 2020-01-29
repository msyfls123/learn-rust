use advent_of_code::aoc2017::day10::get_knot_hash;
use std::collections::HashMap;

type Point = (i32, i32);

static SIBLINGS: [Point; 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn get_related_squares(
  map: &HashMap<Point, bool>,
  point: &Point
) -> Vec<Point> {
  let mut list = vec!{};
  for sibling in &SIBLINGS {
    let cur_point = (point.0 + sibling.0, point.1 + sibling.1);
    match map.get(&cur_point) {
      Some(true) => list.push(cur_point),
      _ => ()
    }
  }
  list
}

fn get_region(
  map: &HashMap<Point, bool>,
  point: &Point
) -> Vec<Point> {
  let mut region: Vec<Point> = vec!{*point};
  let mut early_total = 0;
  while early_total != region.iter().count() {
    early_total = region.iter().count();
    let copied_region = region.clone();
    for point in copied_region.iter() {
      let mut related_squares = get_related_squares(&map, &point);
      region.append(&mut related_squares);
    };
    region.sort();
    region.dedup();
  };
  region
}

fn main() {
  let input = "ugkiagan";
  let hashes: Vec<u128> = (0..128).into_iter().map(|x| get_knot_hash(&format!("{}-{}", input, x)[..])).collect();
  let squares: Vec<Vec<bool>> = hashes.iter().map(|x| format!("{:0128b}", x)).map(|x| {
    x.split("").map(|y| y == "1").collect::<Vec<bool>>()
  }).collect();
  let total_used_squares = squares.iter().fold(
    0,
    |acc, x| {
      let used_squares = x.iter().filter(|&x| *x).count();
      acc + used_squares
    }
  );
  println!("Part 1: {}", total_used_squares);
  let map = squares.iter().enumerate().fold(
    HashMap::new(),
    |mut acc, (row, columns)| {
      for (column, &used) in columns.into_iter().enumerate() {
        if used {
          acc.insert((row as i32, column as i32), true);
        }
      }
      acc
    }
  );
  let mut rest_squares: Vec<Point> = map.keys().map(|&x| x).collect();
  let mut regions: Vec<Vec<Point>> = vec!{};
  while !rest_squares.is_empty() {
    let first_square = rest_squares.first().unwrap();
    let region = get_region(&map, &first_square);
    regions.push(region.clone());
    rest_squares.retain(|x| region.iter().all(|y| y != x));
  }
  println!("Part 2: {}", regions.len());
}