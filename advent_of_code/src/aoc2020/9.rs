use advent_of_code::get_str_array_from_file;

const PREAMBLE: usize = 25;

fn check_sum(list: &[i64], next_num: i64) -> bool {
  let sub_list: Vec<i64> = list.iter().map(|x| next_num - x).collect();
  list.iter().enumerate().any(|(index, num)| {

    match sub_list.iter().position(|x| x == num) {
      Some(sub_index) => sub_index != index,
      None => false
    }
  })
}

fn main() {
  let list: Vec<i64> = get_str_array_from_file(&vec!{"aoc2020", "data", "9.txt"})
    .iter()
    .map(|line| {
      line.parse::<i64>().unwrap()
    })
    .collect();
  let len = list.len();
  let weakness = (0..len - PREAMBLE).find_map(|i| {
    let next_num = list[i + PREAMBLE];
    match check_sum(&list[i..i + PREAMBLE], next_num) {
      false => Some(next_num),
      true => None
    }
  });
  println!("Part 1: {:?}", weakness);
}
