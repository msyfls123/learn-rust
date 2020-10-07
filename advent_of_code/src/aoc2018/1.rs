use std::collections::HashMap;
use advent_of_code::get_str_array_from_file;

fn main() {
  let array = get_str_array_from_file(&vec!{"aoc2018", "data", "1.txt"});
  let list: Vec<i32> = array.iter().map(|x| x.parse::<i32>().unwrap()).collect();
  let result: i32 = list.iter().sum();
  println!("Part 1: {:?}", result);

  let len = list.len();
  let mut total = 0;
  let mut map: HashMap<i32, bool> = HashMap::new();
  map.insert(0, true);
  let twice = (0..).find_map(|x| {
    let index = x % len;
    total += list[index];
    let entry = map.get(&total);
    match entry {
      Some(_) => Some(total),
      None => {
        map.insert(total, true);
        None
      }
    }
  }).unwrap();
  println!("Part 2: {}", twice);
}
