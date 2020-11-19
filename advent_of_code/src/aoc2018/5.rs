use advent_of_code::get_str_array_from_file;
use std::convert::TryInto;

fn react_polymer(text: &String, skip_num: u8) -> Vec<u8> {
  text.as_bytes().iter().fold(vec!{}, |mut acc, &x| {
    let i_x: i32 = x.try_into().unwrap();
    let i_skip: i32 = skip_num.try_into().unwrap();
    let diff = i_x - i_skip;
    if diff == 0 || diff == 32 {
      acc
    } else {
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
      acc
    }
  })
}

fn main() {
  let array = get_str_array_from_file(&vec!{"aoc2018", "data", "5.txt"});
  let text = array.first().unwrap();
  let result = react_polymer(&text, 0);
  println!("Part 1: {}", result.len());
  let result2 = (('A' as u8)..=('Z' as u8)).map(|x| react_polymer(&text, x).len()).min().unwrap();
  println!("Part 2: {}", result2);
}
