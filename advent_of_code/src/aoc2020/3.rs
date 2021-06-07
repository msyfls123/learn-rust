use advent_of_code::get_str_array_from_file;

static SLOPES: [(usize, usize); 5] = [
  (1, 1),
  (3, 1),
  (5, 1),
  (7, 1),
  (1, 2),
];

fn encounter_trees(
  map: &Vec<Vec<bool>>,
  right: usize,
  down: usize,
) -> usize {
  let y_len = map.len();
  let x_len = map[0].len();
  let steps = (y_len - 1) / down;
  (1..=steps).filter(|&step| {
    let y = down * step;
    map[y][(right * step) % x_len]
  }).count()
}

fn main() {
  let data = get_str_array_from_file(&vec!{"aoc2020", "data", "3.txt"});
  let map: Vec<Vec<bool>> = data.iter().map(|x| x.split("").filter_map(|c| {
    match c {
      "#" => Some(true),
      "." => Some(false),
      _ => None,
    }
  }).collect()).collect();
  // let y_len = map.len();
  // let x_len = map[0].len();
  // let encounter_trees = (1..y_len).filter(|&y| {
  //   map[y][(3 * y) % x_len]
  // }).count();
  println!("Part 1: {}", encounter_trees(&map, 3, 1));
  let trees_product = SLOPES.iter()
    .map(|&(right, down)| encounter_trees(&map, right, down));
  println!("{:?}", trees_product.clone().collect::<Vec<usize>>());
  println!("Part 2: {}", trees_product.fold(1, |acc, x| acc * x));
}
