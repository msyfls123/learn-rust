use std::collections::HashMap;

use advent_of_code::get_str_array_from_file;
use itertools::Itertools;
use num::ToPrimitive;

type DigitMap = HashMap<Vec<char>, usize>;

static SEVEN_SEGMENT_DISPLAY_TUPLES: [(usize, usize); 10] = [
  (0, 6),
  (1, 2),
  (2, 5),
  (3, 5),
  (4, 4),
  (5, 5),
  (6, 6),
  (7, 3),
  (8, 7),
  (9, 6),
];

struct Entry {
  patterns: Vec<String>,
  output: Vec<String>
}

fn intersect(a: &str, b: &str) -> Vec<char> {
  a.chars().into_iter().filter(|x| b.chars().into_iter().any(|y| x == &y)).collect()
}

fn difference(a: &str, b: &str) -> Vec<char> {
  a.chars().into_iter().filter(|x| b.chars().into_iter().all(|y| x != &y)).collect()
}

fn is_same(a: &str, b: &str) -> bool {
  a.len() == b.len() && difference(a, b).len() == 0
}

fn get_chars(text: &str) -> Vec<char> {
  text.chars().sorted().collect()
}

fn careful_analysis(patterns: &Vec<String>) -> DigitMap {
  let (d_1, d_4, d_7, d_8) = [1, 4, 7, 8].iter().map(get_segment_count).map(|count| patterns.iter().find(|pat| pat.len() == count).unwrap()).collect_tuple().unwrap();
  let cf = d_1.clone();
  let mut count_6_digits: Vec<&String> = patterns.iter().filter(|p| p.len() == 6).collect();
  let d_6 = count_6_digits.iter().find(|v| intersect(d_1, v).len() == 1).unwrap().to_owned();
  count_6_digits.retain(|&v| v != d_6);
  let bd: String = difference(d_4, d_1).iter().collect();
  let d_9 = count_6_digits.iter().find(|&&v| intersect(v, &bd).len() == 2).unwrap().to_owned();
  count_6_digits.retain(|&v| v != d_9);
  let d_0 = count_6_digits[0];
  let mut count_5_digits: Vec<&String> = patterns.iter().filter(|p| p.len() == 5).collect();
  let d_5 = count_5_digits.iter().find(|&&v| intersect(v, &bd).len() == 2).unwrap().to_owned();
  count_5_digits.retain(|&v| v != d_5);
  let d_3 = count_5_digits.iter().find(|v| intersect(v, &cf).len() == 2).unwrap().to_owned();
  count_5_digits.retain(|&v| v != d_3);
  let d_2 = count_5_digits[0];

  let mut digit_map = HashMap::new();
  [
    d_0,
    d_1,
    d_2,
    d_3,
    d_4,
    d_5,
    d_6,
    d_7,
    d_8,
    d_9,
  ].iter().enumerate().for_each(|(index, d)| {
    digit_map.insert(get_chars(d), index);
  });
  digit_map
}

fn get_output(entry: &Entry) -> usize {
  let digit_map = careful_analysis(&entry.patterns);
  let output_digits: String = entry.output.iter().map(|v| {
    char::from_digit(digit_map.get(&get_chars(v)).unwrap().to_u32().unwrap(), 10).unwrap()
  }).collect();
  // println!("{:?}, {}", entry.output, output_digits);
  output_digits.parse::<usize>().unwrap()
}

fn get_segment_count(num: &usize) -> usize {
  SEVEN_SEGMENT_DISPLAY_TUPLES.iter().find(|(digit, _)| digit == num).unwrap().1
}

fn main() {
  let data = get_str_array_from_file(&vec!{"aoc2021", "data", "8.txt"});
  let notes: Vec<Entry> = data.iter().map(|line| {
    let (patterns, output) = line.split_once(" | ").unwrap();
    Entry {
      patterns: patterns.split(" ").map(|x| x.to_owned()).collect(),
      output: output.split(" ").map(|x| x.to_owned()).collect(),
    }
  }).collect();
  let unique_segment_numbers: Vec<usize> = [1, 4, 7, 8].iter().map(get_segment_count).collect();

  let count_unique_digits: usize = notes.iter().map(|entry| {
    entry.output.iter().filter(|value| unique_segment_numbers.contains(&value.len())).count()
  }).sum();

  println!("Part 1: {}", count_unique_digits);

  let all_output_values: Vec<usize> = notes.iter().map(|entry| get_output(entry)).collect();
  println!("Part 2: {}", all_output_values.iter().sum::<usize>());
}
