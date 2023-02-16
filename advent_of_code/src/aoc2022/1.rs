use advent_of_code::get_group_str_from_file;
use itertools::Itertools;
fn main() {
  let elves = get_group_str_from_file(&vec!{"aoc2022", "data", "1.txt"});
  let most_calories = elves.iter().map(|foods| {
    foods.iter().map(|food| food.parse::<usize>().unwrap()).sum::<usize>()
  }).max();
  println!("Part 1: {:?}", most_calories);
  let most_3_calories: usize = elves.iter().map(|foods| {
    foods.iter().map(|food| food.parse::<usize>().unwrap()).sum::<usize>()
  }).sorted().rev().take(3).sum();
  println!("Part 2: {}", most_3_calories);
}