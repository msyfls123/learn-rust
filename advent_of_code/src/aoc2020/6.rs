use itertools::Itertools;
use advent_of_code::get_str_from_file;

const MAX: u32 = 2u32.pow(26) - 1;

fn main() {
  let data: Vec<String> = get_str_from_file(&vec!{"aoc2020", "data", "6.txt"})
    .lines()
    .map(|line| line.to_string()).collect();
  let groups: Vec<Vec<String>> = data.into_iter()
    .group_by(|line| line == "")
    .into_iter()
    .filter_map(|(_key, group)| {
      let lines: Vec<String> = group.collect();
      if lines == vec!{String::from("")} {
        return None
      }
      Some(lines)
    })
    .collect();
  let answers: Vec<Vec<bool>> = groups.iter()
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
  let concrete_answers: Vec<u32> = groups.iter()
    .map(|lines| {
      let mut answer = MAX;
      lines.iter().for_each(|line| {
        let one_answer = line.chars().fold(0u32, |acc, c| {
          acc | (1 << (c as u32 - 97))
        });
        answer &= one_answer
      });
      answer
    })
    .collect();
  let sum_of_counts: usize = concrete_answers.iter().map(|answer| {
    format!("{:b}", answer).chars().filter(|&x| x == '1').count()
  }).sum();
  println!("Part 2: the sum of those counts which everyone answered 'yes' is {}", sum_of_counts);
}
