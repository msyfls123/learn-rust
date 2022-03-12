use std::{collections::HashMap};

use itertools::Itertools;

type CupMap = HashMap<usize, usize>;

static LAELING: [usize; 9] = [5, 6, 2, 8, 9, 3, 1, 4, 7];
// static LAELING: [usize; 9] = [3, 8, 9, 1, 2, 5, 4, 6, 7];

fn get_following_cups(cup_map: &CupMap, current: usize, num: usize) -> Vec<usize> {
  (0..num).fold(vec!{}, |mut acc: Vec<usize>, _| {
    let this = if acc.len() > 0 {
      acc.last().unwrap().to_owned()
    } else {
      current
    };
    acc.push(cup_map.get(&this).unwrap().to_owned());
    acc
  })
}

fn move_round(cup_map: &mut CupMap, current: usize, total: usize) -> usize {
  let picked_cups = (0..3).fold(vec!{}, |mut acc: Vec<usize>, _| {
    let this = if acc.len() > 0 {
      acc.last().unwrap().to_owned()
    } else {
      current
    };
    acc.push(cup_map.get(&this).unwrap().to_owned());
    acc
  });
  let next_cup = *cup_map.get(picked_cups.last().unwrap()).unwrap();
  cup_map.insert(current, next_cup);

  let destination_cup = (1..).map(|v| {
    if current > v {
      current - v
    } else {
      total - (v - current)
    }
  }).filter(|x| !picked_cups.contains(x)).next().unwrap();
  let next_cup = cup_map.get(&destination_cup).unwrap().to_owned();
  cup_map.insert(destination_cup, *picked_cups.first().unwrap());
  cup_map.insert(*picked_cups.last().unwrap(), next_cup);
  
  *cup_map.get(&current).unwrap()
}

fn play(total: usize, moves: usize) -> CupMap {
  let mut cup_map = HashMap::new();
  let mut current = LAELING[0];
  let cups: Vec<usize> = (1..=total).map(|num| {
    if num <= LAELING.len() {
      LAELING[num - 1]
    } else {
      num
    }
  }).collect();
  cups.iter().enumerate().for_each(|(index, &cup)| {
    let next_value = cups[(index + 1) % total];
    cup_map.insert(cup, next_value);
  });

  for i in 0..moves {
    if i % 100000 == 0 {
      println!("Round {}", i + 1); 
    }
    current = move_round(&mut cup_map, current, total)
  }
  cup_map
}

fn main() {
  let map1 = play(9, 100);
  let result1 = get_following_cups(&map1, 1, 8).iter().map(|x| x.to_string()).collect_vec().join("");
  println!("Part 1: {}", result1);
  let map2 = play(1_000_000, 10_000_000);
  let result2 = get_following_cups(&map2, 1, 2);
  println!("Part 2: {}", result2[0] * result2[1]);
}
