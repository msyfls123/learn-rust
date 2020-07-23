use advent_of_code::get_str_array_from_file;
use std::collections::HashMap;

type POSITION = [i32; 2];
type DIRECTION = [i32; 2];
enum TURNING {
  LEFT,
  RIGHT,
}

fn turn_direction(current: DIRECTION, turning: TURNING) -> DIRECTION {
  match turning {
    TURNING::LEFT => [-current[1], current[0]],
    TURNING::RIGHT => [current[1], -current[0]],
  }
}

fn main() {
  let array: Vec<Vec<char>> = get_str_array_from_file(&vec!{"aoc2017", "day22_data.txt"}).iter().map(|row| {
    row.as_bytes().iter().map(|&x| x as char).collect()
  }).collect();
  let mut map: HashMap<POSITION, char> = HashMap::new();
  array.iter().enumerate().for_each(|(i, row)| {
    row.iter().enumerate().for_each(|(j, &cell)| {
      let cursor = map.entry([i as i32, j as i32]).or_insert('.');
      *cursor = cell
    })
  });
  let len = array.len() as i32;
  let mut position = [len / 2, len / 2];
  let mut direction = [-1, 0];
  let mut total = 0;
  let mut infections = 0;

  while total < 10000 {
    let current = map.entry(position).or_insert('.');
    if *current == '#' {
      direction = turn_direction(direction, TURNING::RIGHT);
      *current = '.';
    } else {
      direction = turn_direction(direction, TURNING::LEFT);
      *current = '#';
      infections += 1;
    }
    position = [position[0] + direction[0], position[1] + direction[1]];
    total += 1;
  }
  println!("Part 1: {}", infections);
}
