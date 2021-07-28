use advent_of_code::get_group_str_from_file;
use lazy_static::lazy_static;
use regex::Regex;

type Range = (usize, usize);

fn get_range(text: &str) -> Vec<Range> {
  lazy_static! {
    static ref RE: Regex = Regex::new(r"(\d+)-(\d+)").unwrap();
  }
  RE.captures_iter(text).map(|cap| {
    (cap[1].parse().unwrap(), cap[2].parse().unwrap())
  }).collect()
}

fn main() {
  let notes = get_group_str_from_file(&vec!{"aoc2020", "data", "16.txt"});
  let nearby_ticket_values: Vec<usize> = notes[2].iter().skip(1).flat_map(|line| {
    line.split(",").map(|x| x.parse().unwrap()).collect::<Vec<usize>>()
  }).collect();
  let ranges: Vec<Range> = notes[0].iter().flat_map(|line| get_range(&line)).collect();
  let ticket_scanning_error_rate: usize = nearby_ticket_values.iter().filter(|&value| {
    !ranges.iter().any(|(min, max)| {
      value >= min && value <= max
    })
  }).sum();
  println!("Part 1: {}", ticket_scanning_error_rate);
}
