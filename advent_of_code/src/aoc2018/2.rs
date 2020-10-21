use advent_of_code::get_str_array_from_file;

fn find_common_letters(vec1: &Vec<char>, vec2: &Vec<char>) -> Vec<char> {
  vec1.iter().enumerate().filter_map(|(i, &x)| {
    if x == vec2[i] {
      Some(x)
    } else {
      None
    }
  }).collect()
}

fn main() {
  let array = get_str_array_from_file(&vec!{"aoc2018", "data", "2.txt"});
  let raw_data: Vec<Vec<char>> = array.iter().map(|line| {
    line.chars().collect::<Vec<char>>()
  }).collect();
  let result1: Vec<(bool, bool)> = raw_data.iter().map(|data| {
    let mut chars = data.clone();
    chars.sort();
    let mut three = false;
    let mut two = false;
    let len = chars.len();
    chars.iter().enumerate().fold((0, '-'), |(count, prev), (index, c)| {
      if prev != *c {
        if count == 3 { three = true }
        if count == 2 { two = true }
        (1, *c)
      } else {
        if index == len - 1 {
          let count = count + 1;
          if count == 3 { three = true }
          if count == 2 { two = true }
          (count, prev)
        } else {
          (count + 1, prev)
        }
      }
    });
    (three, two)
  }).collect();
  let checksum = result1.iter().filter(|&x| x.0).count() * result1.iter().filter(|&x| x.1).count();
  println!("Part 1: {}", checksum);
  let result2: String = raw_data.iter().enumerate().find_map(|(i, vec1)| {
    let len = vec1.len();
    raw_data.iter().skip(i + 1).find_map(|vec2| {
      let common_letters = find_common_letters(vec1, vec2);
      if common_letters.len() == len - 1 {
        Some(common_letters)
      } else {
        None
      }
    })
  }).unwrap().iter().collect();
  println!("Part 2: {}", result2);
}
