#[macro_use] extern crate lazy_static;
extern crate regex;

use advent_of_code::get_str_array_from_file;
use regex::{ Regex };
use std::collections::HashMap;

type Instruction = [char; 2];

fn get_instruction(text: &str) -> Instruction {
  lazy_static! {
    static ref RE_INSTRUCTION: Regex = Regex::new(r"Step\s(?P<before>[A-Z])\smust\sbe\sfinished\sbefore\sstep\s(?P<after>[A-Z])\scan\sbegin").unwrap();
  }
  let captured = RE_INSTRUCTION.captures(text).unwrap();
  let before = captured.name("before").unwrap().as_str().parse::<char>().unwrap();
  let after = captured.name("after").unwrap().as_str().parse::<char>().unwrap();
  [before, after]
}

fn generate_order(
  map: &mut HashMap<char, Vec<char>>,
  list: &mut Vec<char>
) -> Vec<char> {
  if map.keys().len() > 0 {
    let current = map.iter().filter_map(|x| {
      if x.1.len() == 0 {
        Some(*x.0)
      } else {
        None
      }
    }).min().unwrap();
    list.push(current);
    map.remove(&current);
    map.iter_mut().for_each(|(_, value)| {
      if value.contains(&current) {
        value.retain(|&x| x != current);
      }
    });
    generate_order(map, list)
  } else {
    list.to_vec()
  }
}

fn parallel(
  map: &mut HashMap<char, Vec<char>>,
  running: Vec<(char, usize)>,
  time: usize,
  limit: usize,
  extra_duration: usize,
) -> usize {
  let mut new_running = running.clone();
  // remove runned from map
  new_running = new_running.iter().filter(|&(current, count)| {
    let result = *count != 0;
    if !result {
      map.remove(current);
      map.iter_mut().for_each(|(_, value)| {
        if value.contains(current) {
          value.retain(|&x| x != *current);
        }
      });
    }
    result
  }).map(|x| *x).collect();
  // there are nothing left, return time
  if map.keys().len() + new_running.len() == 0 {
    return time
  }
  let mut waiting: Vec<char> = map.iter().filter_map(|x| {
    if x.1.len() == 0 && !new_running.iter().any(|&y| y.0 == *x.0) {
      Some(*x.0)
    } else {
      None
    }
  }).collect();
  waiting.sort();
  // count rest number for running
  let rest = (limit - new_running.len()).min(waiting.len());
  // push waiting to running list
  &waiting[0..rest].iter().for_each(|&current| {
    new_running.push((current, current as usize - 64 + extra_duration));
  });
  // run every running (count--)
  new_running = new_running.iter().map(|&(step, count)| (step, count- 1)).collect();
  parallel(map, new_running, time + 1, limit, extra_duration)
}

fn main() {
  let array = get_str_array_from_file(&vec!{"aoc2018", "data", "7.txt"});
  let instructions: Vec<Instruction> = array.iter().map(|t| get_instruction(t)).collect();

  let mut map: HashMap<char, Vec<char>> = HashMap::new();
  instructions.iter().for_each(|&[before, after]| {
    map.entry(before).or_insert(vec!{});
    let after_entry = map.entry(after).or_insert(vec!{});
    (*after_entry).push(before);
  });
  let map = map;
  let order: String = generate_order(&mut map.clone(), &mut vec!{}).iter().collect();
  println!("Part 1: the steps in my instructions be completed in order of {}.", order);
  let duration = parallel(&mut map.clone(), vec!{}, 0, 5, 60);
  println!("Part 2: it will take {} seconds to complete all of the steps.", duration)
}
