use advent_of_code::get_str_array_from_file;

fn main() {
  let data = get_str_array_from_file(&vec!{"aoc2020", "data", "3.txt"});
  let map: Vec<Vec<bool>> = data.iter().map(|x| x.split("").filter_map(|c| {
    match c {
      "#" => Some(true),
      "." => Some(false),
      _ => None,
    }
  }).collect()).collect();
  let y_len = map.len();
  let x_len = map[0].len();
  let encounter_trees = (1..y_len).filter(|&y| {
    map[y][(3 * y) % x_len]
  }).count();
  println!("Part 1: {}", encounter_trees);
}
