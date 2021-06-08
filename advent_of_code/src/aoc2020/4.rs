use itertools::Itertools;
use advent_of_code::get_str_from_file;

const FIELDS: [&str; 7] = [
  "byr",
  "iyr",
  "eyr",
  "hgt",
  "hcl",
  "ecl",
  "pid",
];

fn main() {
  let data: Vec<String> = get_str_from_file(&vec!{"aoc2020", "data", "4.txt"})
    .lines()
    .map(|line| line.to_string()).collect();
  let passports: Vec<Vec<String>> = data.into_iter()
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
  let valid_passports: Vec<Vec<String>> = passports.iter().filter_map(|passport| {
    match FIELDS.iter().all(|field| {
      passport.iter().any(|pair| pair.starts_with(field))
    }) {
      true => Some(passport.to_owned()),
      _ => None,
    }
  }).collect();
  println!("Part 1: {}", valid_passports.len());
}
