use advent_of_code::get_str_array_from_file;

fn pos(x: usize, y: usize) -> usize {
  let step = x - 1;
  let is_forward = y % (step * 2) == y % step;
  if is_forward {
    y % step
  } else {
    step - (y % step)
  }
}

fn main() {
  let array = get_str_array_from_file(&vec!{"aoc2017", "day13_data.txt"});
  let depth_range_pairs: Vec<Vec<usize>> = array.iter().map(
    |x| x.split(": ").map(|y| y.parse::<usize>().unwrap()).collect()
  ).collect();
  println!("{:?}", depth_range_pairs);
  let severity = depth_range_pairs.iter().fold(
    0,
    |mut acc, pair| {
      if let [depth, range] = &pair[..] {
        if pos(*range, *depth) == 0 {
          acc += range * depth;
        }
      }
      acc
    }
  );
  println!("Part 1: {}", severity);
}