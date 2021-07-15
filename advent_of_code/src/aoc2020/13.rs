use advent_of_code::get_str_array_from_file;
use modinverse::modinverse;

fn main() {
  let data = get_str_array_from_file(&vec!{"aoc2020", "data", "13.txt"});
  let earliest_timestamp: usize = data[0].parse().unwrap();
  let buses: Vec<(usize, usize)> = data[1].split(",").enumerate().filter_map(|(index, text)| {
    if text == "x" {
      None
    } else {
      Some((index, text.parse().unwrap()))
    }
  }).collect();
  let earliest_bus_info = buses.iter().map(|(_index, interval)| {
    let wait = (interval - (earliest_timestamp % interval)) % interval;
    (wait, interval)
  }).min_by_key(|(wait, _)| *wait).unwrap();
  println!("Part 1: {}", earliest_bus_info.0 * earliest_bus_info.1);
  let last_bus = buses[buses.len() - 1];
  let product = buses.iter().fold(1, |acc, (_, v)| acc * v);
  let subsequent_timestamp: usize = buses.iter().map(|(index, interval)| {
    let rest = buses.iter().filter_map(|(i, x)| {
      if i == index {
        None
      } else {
        Some(x)
      }
    }).fold(1, |acc, v| acc * v);
    let m = (last_bus.0 - index) % interval;
    if m > 0 {
      let modinv = modinverse(rest as isize, *interval as isize).unwrap() as usize;
      m * modinv * rest
    } else {
      m
    }
  }).sum::<usize>() % product - last_bus.0;
  println!("Part 2: {}", subsequent_timestamp);
}
