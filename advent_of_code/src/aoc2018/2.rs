use advent_of_code::get_str_array_from_file;

fn main() {
  let array = get_str_array_from_file(&vec!{"aoc2018", "data", "2.txt"});
  let data: Vec<(bool, bool)> = array.iter().map(|line| {
    let mut chars = line.chars().collect::<Vec<char>>();
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
  let checksum = data.iter().filter(|&x| x.0).count() * data.iter().filter(|&x| x.1).count();
  println!("Part 1: {}", checksum);
}
