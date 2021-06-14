use advent_of_code::get_str_array_from_file;

fn get_num(text: &str, upper: char) -> usize {
  let binary_str: String = text.chars().map(|x| {
    if x == upper {
      '1'
    } else {
      '0'
    }
  }).collect();
  usize::from_str_radix(&binary_str, 2).unwrap()
}

fn main() {
  let data = get_str_array_from_file(&vec!{"aoc2020", "data", "5.txt"});
  let highest = data.iter().map(|text| {
    let (row, column) = text.split_at(7);
    get_num(row, 'B') * 8 + get_num(column, 'R')
  }).max().unwrap();
  println!("Part 1: What is the highest seat ID on a boarding pass - {}", highest);
}
