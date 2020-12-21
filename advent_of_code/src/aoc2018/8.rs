use advent_of_code::get_str_from_file;
use std::convert::TryInto;

type Nodes = Vec<usize>;

fn sum_metadata_entries(
  nodes: &Nodes,
  start: usize,
  sum: usize,
) -> (usize, usize) {
  let mut index = start;
  let mut sum = sum;
  let [children_count, metadata_count]: [usize; 2] = nodes[index..index + 2].try_into().unwrap();
  index += 2;
  (0..children_count).for_each(|_| {
    let (next_index, diff_sum) = sum_metadata_entries(nodes, index, 0);
    index = next_index;
    sum += diff_sum;
  });
  let metadata_sum: usize = nodes[index..index+metadata_count].iter().sum();
  sum += metadata_sum;
  (index + metadata_count, sum)
}

fn get_root_value(
  nodes: &Nodes,
  start: usize,
  sum: usize,
) -> (usize, usize) {
  let mut index = start;
  let mut sum = sum;
  let [children_count, metadata_count]: [usize; 2] = nodes[index..index + 2].try_into().unwrap();
  index += 2;
  let children_sum: Vec<usize> = (0..children_count).map(|_| {
    let (next_index, child_sum) = get_root_value(nodes, index, 0);
    index = next_index;
    child_sum
  }).collect();
  if children_count == 0 {
    let metadata_sum: usize = nodes[index..index+metadata_count].iter().sum();
    sum += metadata_sum;
  } else {
    let metadata_sum: usize = nodes[index..index+metadata_count].iter().filter_map(|&i| {
      children_sum.get(i - 1)
    }).sum();
    sum += metadata_sum;
  }
  (index + metadata_count, sum)
}

fn main() {
  let data = get_str_from_file(&vec!{"aoc2018", "data", "8.txt"});
  let nodes = data.trim_end().split(" ").map(|x| x.parse::<usize>().unwrap()).collect::<Nodes>();
  let metadata_entries = sum_metadata_entries(&nodes, 0, 0);
  println!("Part 1: the sum of all metadata entries is {}.", metadata_entries.1);
  let root_node_value = get_root_value(&nodes, 0, 0);
  println!("Part 2: the value of the root node is {}.", root_node_value.1);
}
