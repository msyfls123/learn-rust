use std::collections::HashMap;

type LastIndexMap = HashMap<usize, usize>;

const INPUT: [usize; 6] = [1,0,15,2,10,13];

fn turn(
  last_index_map: &mut LastIndexMap,
  index: usize,
  last_value: usize,
) -> usize {
  let entry = last_index_map.entry(last_value).or_insert(index - 1);
  if *entry == index - 1 {
    // last_index_map.insert(0, index);
    0
  } else {
    let age = index - 1 - *entry;
    *entry = index - 1;
    age
  }
}

fn main() {
  let mut last_index_map: LastIndexMap = HashMap::new();
  INPUT.iter().enumerate().for_each(|(i, &x)| {
    last_index_map.insert(x, i + 1);
  });
  let mut index = INPUT.len();
  let mut last_value = INPUT[index - 1];
  while index < 2020 {
    index += 1;
    last_value = turn(&mut last_index_map, index, last_value);
  }
  println!("Part 1: {}", last_value);
  while index < 30000000 {
    index += 1;
    last_value = turn(&mut last_index_map, index, last_value);
  }
  println!("Part 2: {}", last_value);
}
