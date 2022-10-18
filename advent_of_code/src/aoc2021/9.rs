use std::{collections::HashMap, cell::RefCell};

use advent_of_code::get_str_array_from_file;

type HeightMap = Vec<Vec<usize>>;

#[test]
fn test_is_low_point() {
  let map = vec!{
    vec!{2,1,9},
    vec!{3,9,8},
  };
  let width = map[0].len();
  let height = map.len();
  let point_1 = (1, 0);
  let point_2 = (1, 1);
  assert_eq!(is_low_point(&map, width, height, point_1), true);
  assert_eq!(is_low_point(&map, width, height, point_2), false);
}

fn is_low_point(
  map: &HeightMap,
  width: usize,
  height: usize,
  point: (usize, usize),
) -> bool{
  let point_height = map[point.1][point.0];
  [
    (-1, 0),
    (1, 0),
    (0, -1),
    (0, 1)
  ].iter().all(|adjacent| {
    let pos = (point.0 as isize + adjacent.0, point.1 as isize + adjacent.1);
    if pos.0 < 0 || pos.0 >= width as isize || pos.1 < 0 || pos.1 >= height as isize {
      return true;
    }
    map[pos.1 as usize][pos.0 as usize] > point_height
  })
}

fn main() {
  let data = get_str_array_from_file(&vec!{"aoc2021", "data", "9.txt"});
  let height_map: HeightMap = data.iter().map(|line| {
    line.chars().map(|c| c.to_digit(10).unwrap() as usize).collect()
  }).collect();
  let height_map = RefCell::new(height_map);
  let map_width = height_map.borrow()[0].len();
  let map_height = height_map.borrow().len();
  let low_points: Vec<usize> = (0..map_width).flat_map(|x| {
    let height_map = height_map.clone();
    (0..map_height).filter_map(move |y| {
      let point = (x, y);
      if is_low_point(&height_map.borrow(), map_width, map_height, point) {
        let point_value = height_map.borrow()[y][x].to_owned();
        Some(point_value)
      } else {
        None
      }
    }).to_owned()
  }).collect();
  let all_low_points_sum: usize = low_points.iter().map(|v| v + 1).sum();
  println!("Part 1: {}", all_low_points_sum);
}
