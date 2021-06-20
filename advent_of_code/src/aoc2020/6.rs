use itertools::Itertools;
use advent_of_code::get_str_from_file;

fn main() {
  let data: Vec<String> = get_str_from_file(&vec!{"aoc2020", "data", "6.txt"})
    .lines()
    .map(|line| line.to_string()).collect();
  let answers: Vec<Vec<bool>> = data.into_iter()
    .group_by(|line| line == "")
    .into_iter()
    .filter_map(|(_key, group)| {
      let lines: Vec<String> = group.collect();
      if lines == vec!{String::from("")} {
        return None
      }
      Some(lines)
    })
    .map(|lines| {
      let mut questions = vec!{false; 26};
      lines.iter().for_each(|line| {
        line.chars().for_each(|c| {
          questions[c as usize - 97] = true;
        })
      });
      questions
    })
    .collect();
    let sum_of_counts: usize = answers.iter().map(|answer| {
      answer.iter().filter(|&x| *x).count()
    }).sum();
    println!("Part 1: the sum of those counts is {}.", sum_of_counts);
}
