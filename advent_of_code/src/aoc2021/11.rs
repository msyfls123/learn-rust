use std::collections::HashMap;

use advent_of_code::get_str_array_from_file;

type OctopusMap = HashMap<(isize, isize), usize>;

#[test]
fn test_get_octopus_map() {
  let lines = vec!{
    "45654",
    "51115",
    "61116",
    "51115",
    "45654"
  };
  let map = get_octopus_map(&lines);
  assert_eq!(map.get(&(1, 4)), Some(&5));
}

fn get_octopus_map(lines: &Vec<&str>) -> OctopusMap {
  let mut map: OctopusMap = HashMap::new();
  lines.iter().enumerate().for_each(|(y, line)| {
    line.chars().enumerate().for_each(|(x, c)| {
      let value = c.to_digit(10).unwrap() as usize;
      map.insert((x as isize, y as isize), value);
    })
  });
  map
}

fn flash_adjacent(map: &mut OctopusMap, pos: (isize, isize)) {
  [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
  ].iter().for_each(|diff| {
      match map.get_mut(&(pos.0 + diff.0, pos.1 + diff.1)) {
        Some(v) => {
          if *v < 10 {
            *v += 1
          }
        },
        None => {}
      }
  })
}

fn flash(map: &mut OctopusMap, is_initial: bool) -> usize {
  if is_initial {
    for (_, val) in map.iter_mut() {
      *val += 1;
    }
  }

  let flashed_octopus_vec: Vec<(isize, isize)> = map.iter().filter_map(|(pos, val)| {
    if val == &10 {
      Some(pos.to_owned())
    } else {
      None
    }
  }).collect();

  if flashed_octopus_vec.len() == 0 {
    return 0;
  }

  flashed_octopus_vec.iter().for_each(|&pos| {
    flash_adjacent(map, pos);
  });

  flashed_octopus_vec.iter().for_each(|pos| {
    match map.get_mut(pos) {
      Some(val) => { *val += 1 },
      None => {}
    }
  });
  flashed_octopus_vec.len() + flash(map, false)
}

#[test]
fn test_flash() {
  let lines = vec!{
    "11111",
    "19991",
    "19191",
    "19991",
    "11111"
  };
  let mut map = get_octopus_map(&lines);
  assert_eq!(flash(&mut map, true), 9);
}

fn step(map: &mut OctopusMap) -> usize {
  let flash_count = flash(map, true);
  for (_, val) in map.iter_mut() {
    if *val >= 10 {
      *val = 0;
    }
  }
  flash_count
}

fn main() {
  let lines = get_str_array_from_file(&vec!{"aoc2021", "data", "11.txt"});
  let mut map = get_octopus_map(&(lines.iter().map(|v| v.as_str()).collect()));
  let mut map2 = map.clone();
  let map_size = map.len();
  let mut flash_total = 0;
  for _ in 0..100 {
    flash_total += step(&mut map)
  }
  println!("Part 1: {}", flash_total);

  let synchronizing_step = (1..).find(|i| {
    step(&mut map2) == map_size
  });

  println!("Part 2: {:?}", synchronizing_step);
}
