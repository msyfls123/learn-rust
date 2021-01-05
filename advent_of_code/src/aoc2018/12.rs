use advent_of_code::get_str_array_from_file;
use std::collections::HashMap;

type Map = HashMap<i32, char>;
type NoteMap = HashMap<String, char>;

fn get_map_limit(map: &Map, is_max: bool) -> i32 {
  if is_max {
    *map.iter().filter_map(|(k, v)| {
      match v {
        '#' => Some(k),
        _ => None,
      }
    }).max().unwrap()
  } else {
    *map.iter().filter_map(|(k, v)| {
      match v {
        '#' => Some(k),
        _ => None,
      }
    }).min().unwrap()
  }
}

fn generation(
  map: &Map,
  notes: &NoteMap
) -> Map {
  let mut new_map: Map = HashMap::new();
  let min = get_map_limit(&map, false);
  let max = get_map_limit(&map, true);
  (min - 2..max + 2).for_each(|k| {
    let text: String = (k - 2..=k + 2).map(|i| {
      match map.get(&i) {
        Some(v) => *v,
        None => '.',
      }
    }).collect();
    let new_value = match notes.get(&text) {
      Some(v) => *v,
      None => '.',
    };
    // if old_value != new_value {
    //   println!("{}: {} => {}", k, old_value, new_value);
    // }
    new_map.insert(k, new_value);
  });
  new_map
}

fn main() {
  let array = get_str_array_from_file(&vec!["aoc2018", "data", "12.txt"]);
  let initial_state = array[0].split(": ").collect::<Vec<&str>>()[1].chars();
  let notes: Vec<(String, char)> = array[1..].iter().map(|text| {
    let note: Vec<&str> = text.split(" => ").collect();
    (String::from(note[0]), note[1].parse::<char>().unwrap())
  }).collect();

  let mut note_map: NoteMap = HashMap::new();
  notes.iter().for_each(|(text, c)| {
    note_map.insert(text.to_string(), *c);
  });

  let mut map: Map = HashMap::new();
  initial_state.enumerate().for_each(|(i, c)| {
    map.insert(i as i32, c);
  });

  (0..20).for_each(|_index| {
    // println!("===== Round {} =====", index + 1);
    map = generation(&map, &note_map);
  });
  let sum_of_plants: i32 = map.iter().filter_map(|(index, v)| {
    match v {
      '#' => Some(index),
      _ => None
    }
  }).sum();
  println!("Part 1: the sum of the numbers of all pots which contain a plant is {}.", sum_of_plants);
}
