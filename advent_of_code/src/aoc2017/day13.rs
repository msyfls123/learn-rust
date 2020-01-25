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

fn get_severity(pairs: &Vec<Vec<usize>>, delay: usize) -> usize {
  pairs.iter().fold(
    0,
    |mut acc, pair| {
      if let [depth, range] = &pair[..] {
        if pos(*range, *depth + delay) == 0 {
          acc += range * depth;
        }
      }
      acc
    }
  )
}

fn main() {
  let array = get_str_array_from_file(&vec!{"aoc2017", "day13_data.txt"});
  let depth_range_pairs: Vec<Vec<usize>> = array.iter().map(
    |x| x.split(": ").map(|y| y.parse::<usize>().unwrap()).collect()
  ).collect();
  println!("{:?}", depth_range_pairs);
  let severity = get_severity(&depth_range_pairs, 0);
  println!("Part 1: {}", severity);
  let fewest_delay = (0..).find(|&delay| {
    let first_got_caught = match &depth_range_pairs.first().unwrap()[..] {
        [depth, range] => pos(*range, *depth + (delay as usize)) == 0,
        _ => false
    };
    get_severity(&depth_range_pairs, delay as usize) == 0 && !first_got_caught
  }).unwrap();
  println!("Part 2: {}", fewest_delay);
}