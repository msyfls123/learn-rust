use advent_of_code::get_str_array_from_file;
use std::convert::TryInto;

fn main() {
  let array = get_str_array_from_file(&vec!{"aoc2018", "data", "5.txt"});
  let text = array.first().unwrap();
  let result: Vec<u8> = text.as_bytes().iter().fold(vec!{}, |mut acc, &x| {
    if let Some(&last) = acc.last() {
      let i_last: i32 = last.try_into().unwrap();
      let i_x: i32 = x.try_into().unwrap();
      if (i_last - i_x).abs() == 32 {
        acc.pop();
      } else {
        acc.push(x);
      }
    } else {
      acc.push(x);
    }
    return acc;
  });
  println!("Part 1: {}", result.len());
}
