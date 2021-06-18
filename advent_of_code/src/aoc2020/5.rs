use advent_of_code::get_str_array_from_file;
use std::collections::HashMap;

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
  let seats: Vec<usize> = data.iter().map(|text| {
    let (row, column) = text.split_at(7);
    get_num(row, 'B') * 8 + get_num(column, 'R')
  }).collect();
  let highest = seats.iter().max().unwrap();
  println!("Part 1: What is the highest seat ID on a boarding pass - {}", highest);
  let mut seat_map: HashMap<usize, bool> = HashMap::new();
  seats.iter().for_each(|&seat| {
    seat_map.insert(seat, true);
  });
  let empty_seat = (0..128usize * 8).find(|seat| {
    match seat.checked_sub(8) {
      None => false,
      Some(back) => {
        seat_map.contains_key(&back) && !seat_map.contains_key(seat) && seat_map.contains_key(&(seat + 8))
      }
    }
  }).unwrap();
  println!("Part 2: What is the ID of your seat - {}", empty_seat);
}
