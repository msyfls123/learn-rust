use std::collections::HashMap;

use advent_of_code::get_str_array_from_file;

type FishMap = HashMap<usize, usize>;

#[test]
fn test_num() {
  let old_vec = [(0, 6), (3, 2)];
  let old_map: FishMap = HashMap::from(old_vec);
  let new_vec = [(6, 6), (2, 2), (8, 6)];
  let new_map: FishMap = HashMap::from(new_vec);
  assert_eq!(spawn_fish_map(&old_map), new_map);
}

fn spawn_fish_map(map: &FishMap) -> FishMap {
  let mut new_map: FishMap = HashMap::new();
  map.into_iter().for_each(|(num, count)| {
    match *num {
      0 => {
        *(new_map.entry(6).or_insert(0)) += count;
        *(new_map.entry(8).or_insert(0)) += count;
      },
      _ => {
        *(new_map.entry(num - 1).or_insert(0)) += count;
      }
    }
  });
  new_map
}

fn main() {
  let data = &get_str_array_from_file(&vec!{"aoc2021", "data", "6.txt"})[0];
  let lanternfishes: Vec<usize> = data.split(",").map(|t| t.parse::<usize>().unwrap()).collect();
  let mut fish_map: FishMap = HashMap::new();
  lanternfishes.iter().for_each(|&v| {
    *(fish_map.entry(v).or_insert(0)) += 1;
  });
  (0..80).for_each(|_| {
    fish_map = spawn_fish_map(&fish_map);
  });
  let lanternfish_count: usize = fish_map.values().sum();
  println!("Part 1: {}", lanternfish_count);
}
