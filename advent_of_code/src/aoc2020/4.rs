use itertools::Itertools;
use advent_of_code::get_str_from_file;

fn main() {
  let data: Vec<String> = get_str_from_file(&vec!{"aoc2020", "data", "4.txt"})
    .lines()
    .map(|line| line.to_string()).collect();
  let chunks: Vec<Vec<String>> = data.into_iter()
    .group_by(|line| line == "")
    .into_iter()
    .filter_map(|(_key, group)| {
      let line: Vec<String> = group.collect();
      if line == vec!{String::from("")} {
        return None
      }
      Some(line)
    })
    .map(|line| {
      line.into_iter()
        .intersperse(String::from(" "))
        .collect::<String>()
        .split(" ")
        .map(|x| x.to_string())
        .collect()
    })
    .collect();
  println!("{:?}", chunks);
}
