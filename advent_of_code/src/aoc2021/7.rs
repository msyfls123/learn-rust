#![feature(int_abs_diff)]
use std::collections::HashMap;

use advent_of_code::{get_str_from_file, get_str_array_from_file};

type Positions = HashMap<usize, usize>;

#[test]
fn test_calc_fuel_cost() {
  let positions = [16,1,2,0,4,2,7,1,2,14];
  let mut pos_map: Positions = HashMap::new();
  positions.iter().for_each(|&pos| {
    *(pos_map.entry(pos).or_insert(0)) += 1;
  });
  assert_eq!(calc_fuel_cost(&pos_map, 2), 37);
}

fn calc_fuel_cost(positions: &Positions, crab: usize) -> usize {
  positions.iter().map(|(value, count)| count * value.abs_diff(crab)).sum()
}

fn calc_fuel_seq_cost(positions: &Positions, crab: usize) -> usize {
  positions.iter().map(|(value, count)| {
    let diff = value.abs_diff(crab);
    count * (diff + 1) * diff / 2
  }).sum()
}

fn main() {
  let data: Vec<usize> = get_str_array_from_file(&vec!{"aoc2021", "data", "7.txt"})[0]
    .split(",")
    .map(|x| x.parse().unwrap())
    .collect();

  let mut positions: Positions = HashMap::new();
  data.iter().for_each(|&pos| {
    *(positions.entry(pos).or_insert(0)) += 1;
  });

  let min = positions.keys().min().unwrap().to_owned();
  let max = positions.keys().max().unwrap().to_owned();
  let cheapest_outcome = (min..max)
    .map(|crab| (crab, calc_fuel_cost(&positions, crab)))
    .min_by_key(|&(_, cost)| cost);
  println!("Part 1: {:?}", cheapest_outcome);

  let least_fuel = (min..max)
    .map(|crab| (crab, calc_fuel_seq_cost(&positions, crab)))
    .min_by_key(|&(_, cost)| cost);
  println!("Part 2: {:?}", least_fuel);
}
