use advent_of_code::get_str_array_from_file;

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

fn main() {
  let data = get_str_array_from_file(&vec!{"aoc2021", "data", "8.txt"});
  let notes: Vec<Entry> = data.iter().map(|line| {
    let (patterns, output) = line.split_once(" | ").unwrap();
    Entry {
      patterns: patterns.split(" ").map(|x| x.to_owned()).collect(),
      output: output.split(" ").map(|x| x.to_owned()).collect(),
    }
  }).collect();
  let unique_segment_numbers: Vec<usize> = [1, 4, 7, 8].iter().map(|d| {
    SEVEN_SEGMENT_DISPLAY_TUPLES.iter().find(|(digit, _)| digit == d).unwrap().1
  }).collect();

  let count_unique_digits: usize = notes.iter().map(|entry| {
    entry.output.iter().filter(|value| unique_segment_numbers.contains(&value.len())).count()
  }).sum();

  println!("Part 1: {}", count_unique_digits);
}
