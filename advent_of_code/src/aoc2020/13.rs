use advent_of_code::get_str_array_from_file;

fn main() {
  let data = get_str_array_from_file(&vec!{"aoc2020", "data", "13.txt"});
  let earliest_timestamp: usize = data[0].parse().unwrap();
  let buses: Vec<usize> = data[1].split(",").filter_map(|text| {
    if text == "x" {
      None
    } else {
      Some(text.parse().unwrap())
    }
  }).collect();
  let earliest_bus_info = buses.iter().map(|interval| {
    let wait = (interval - (earliest_timestamp % interval)) % interval;
    (wait, interval)
  }).min_by_key(|(wait, _)| *wait).unwrap();
  println!("Part 1: {}", earliest_bus_info.0 * earliest_bus_info.1);
}
