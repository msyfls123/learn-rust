use advent_of_code::get_str_array_from_file;
use std::collections::HashMap;

fn main() {
  let data = get_str_array_from_file(&vec!{"aoc2020", "data", "1.txt"});
  let nums: Vec<usize> = data.iter().map(|x| x.parse().unwrap()).collect();
  let mut map: HashMap<usize, bool> = HashMap::new();
  for &elem in nums.iter() {
    map.insert(elem, true);
  }
  let res = nums.iter().find(|&x| match map.get(&(2020 - x)) {
    Some(v) => *v,
    None => false,
  }).unwrap();
  println!("Part 1: {}", res * (2020 - res));
}
