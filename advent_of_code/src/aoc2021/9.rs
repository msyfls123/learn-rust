use std::{cell::RefCell, collections::HashSet, iter::FromIterator};

use advent_of_code::get_str_array_from_file;
use std::iter::IntoIterator;

type HeightMap = Vec<Vec<usize>>;

type Point = (usize, usize);

type HeightStore = HashSet<Point>;

fn get_adjacent_points(
  width: usize,
  height: usize,
  point: (usize, usize), 
) -> Vec<(usize, usize)> {
  [
    (-1, 0),
    (1, 0),
    (0, -1),
    (0, 1)
  ].iter().filter_map(|adjacent| {
    let pos = (point.0 as isize + adjacent.0, point.1 as isize + adjacent.1);
    if pos.0 < 0 || pos.0 >= width as isize || pos.1 < 0 || pos.1 >= height as isize {
      None
    } else {
      Some((pos.0 as usize, pos.1 as usize))
    }
  }).collect()
}

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
  get_adjacent_points(width, height, point).iter().all(|pos| {
    map[pos.1][pos.0] > point_height
  })
}

#[test]
fn test_get_basin() {
  let map = vec!{
    vec!{2,1,9,9,9,4,3,2,1,0},
    vec!{3,9,8,7,8,9,4,9,2,1},
    vec!{9,8,5,6,7,8,9,8,9,2},
  };
  let points = map.iter().enumerate().flat_map(|(y, row)| {
    row.iter().enumerate().filter_map(move |(x, &height)| {
      if height == 9 {
        None
      } else {
        Some((x, y))
      }
    })
  });
  let store: HeightStore = HashSet::from_iter(points);
  let width = map[0].len();
  let height = map.len();
  let start_point = HashSet::from([(width - 1, 0)]);
  assert_eq!(get_basin(&store, width, height, start_point).len(), 9);
}

fn get_basin(
  store: &HeightStore,
  width: usize,
  height: usize,
  points: HeightStore,
) -> HeightStore {
  if points.len() == 0 {
    return HashSet::new();
  }
  let adjacent_points: Vec<Point> = points.iter().flat_map(|&point| {
    get_adjacent_points(width, height, point)
  }).filter(|point| store.contains(point)).collect();
  let adjacent_points = HashSet::from_iter(adjacent_points);
  let rest_store: HeightStore = store.difference(&adjacent_points).map(|x| x.to_owned()).collect();
  points.union(&get_basin(&rest_store, width, height, adjacent_points)).map(|x| x.to_owned()).collect()
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

  let map = height_map.borrow();
  let lower_9_points = map.iter().enumerate().flat_map(|(y, row)| {
    row.iter().enumerate().filter_map(move |(x, &height)| {
      if height == 9 {
        None
      } else {
        Some((x, y))
      }
    })
  });
  let mut store: HeightStore = HashSet::from_iter(lower_9_points);
  let mut basins = vec!{};
  while store.len() > 0 {
    let start_point = store.iter().next().unwrap().to_owned();
    store.remove(&start_point);
    let start_points = HashSet::from([start_point]);
    let basin = get_basin(&store, map_width, map_height, start_points);
    basins.push(basin.len());
    store = store.difference(&basin).map(|v| v.to_owned()).collect();
  }

  basins.sort_by(|a, b| b.cmp(a));
  let top_3_basins_multiply_value = basins.iter().take(3).fold(1, |acc, v| acc * v);
  println!("Part 2: {}", top_3_basins_multiply_value);
}
