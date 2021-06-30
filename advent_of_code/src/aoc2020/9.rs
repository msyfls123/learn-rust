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

fn check_difference(list: &[i64], next_num: i64) -> Option<(usize, usize)> {
  let plus_list: Vec<i64> = list.iter().map(|x| next_num + x).collect();
  list.iter().enumerate().find_map(|(index, num)| {
    match &plus_list[0..index].iter().position(|x| x == num) {
      Some(plus_index) => Some((plus_index + 1, index)),
      None => None
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
  }).unwrap();
  println!("Part 1: {}", weakness);
  let sum_list: Vec<i64> = list.iter().scan(0, |state, &x| {
    *state += x;
    Some(*state)
  }).collect();
  match check_difference(&sum_list, weakness) {
    Some((lower_index, upper_index)) => {
      let slice = &list[lower_index..=upper_index];
      let smallest = slice.iter().min().unwrap();
      let largest = slice.iter().max().unwrap();
      println!("Part 2: {}", smallest + largest);
    },
    None => {},
  };
}
