use std::collections::HashMap;
use advent_of_code::get_str_array_from_file;

type MapType = HashMap<char, i64>;

#[derive(Debug)]
enum Value {
  Register(char),
  Int(i64),
  Empty,
}

fn get_raw_value(text: &str) -> Value {
  match text.parse::<i64>() {
    Ok(v) => Value::Int(v),
    _ => match text.parse::<char>() {
      Ok(v) => Value::Register(v),
      _ => Value::Empty,
    },
  }
}

fn get_value(map: &MapType, text: &str) -> i64 {
  match get_raw_value(text) {
    Value::Empty => 0,
    Value::Int(v) => v,
    Value::Register(c) => match map.get(&c) {
      Some(&v) => v,
      None => 0,
    }
  }
}

fn set_value(map: &mut MapType, text: &str, value: i64) {
  match get_raw_value(text) {
    Value::Register(c) => {
      let entry = map.entry(c).or_insert(0);
      *entry = value;
    },
    _ => (),
  }
}

fn do_instruction(
  instruction: &Vec<&str>,
  mut map: MapType,
  current_index: &mut usize,
  sound: &mut Option<i64>,
  recovered_sound: &mut Option<i64>
) -> MapType {
  let gv = get_value;
  let sv = set_value;
  match instruction[0] {
    "snd" => {
      *sound = Some(get_value(&map, instruction[1]));
      *current_index += 1;
    },
    "set" => {
      let value = gv(&map, instruction[2]);
      sv(&mut map, instruction[1], value);
      *current_index += 1;
    },
    "add" => {
      let t1 = instruction[1];
      let t2 = instruction[2];
      let value = gv(&map, t1) + get_value(&map, t2);
      sv(&mut map, t1, value);
      *current_index += 1;
    },
    "mul" => {
      let t1 = instruction[1];
      let t2 = instruction[2];
      let value = gv(&map, t1) * get_value(&map, t2);
      sv(&mut map, t1, value);
      *current_index += 1;
    },
    "mod" => {
      let t1 = instruction[1];
      let t2 = instruction[2];
      let value = gv(&map, t1) % get_value(&map, t2);
      sv(&mut map, t1, value);
      *current_index += 1;
    },
    "rcv" => {
      let v1 = gv(&map, instruction[1]);
      if v1 != 0 {
        *recovered_sound = match sound {
            Some(v) => Some(*v),
            None => None,
        };
      };
      *current_index += 1;
    },
    "jgz" => {
      let v1 = gv(&map, instruction[1]);
      let v2 = gv(&map, instruction[2]);
      if v1 > 0 {
        *current_index = (*current_index as i64 + v2) as usize;
      } else {
        *current_index += 1;
      };
    }
    _ => ()
  };
  map
}

fn main() {
  let array = get_str_array_from_file(&["aoc2017", "day18_data.txt"].to_vec());
  let instructions: Vec<Vec<&str>> = array.iter().map(
    |x| x.split(" ").collect()
  ).collect();
  let mut map: MapType = HashMap::new();
  let mut current_index: usize = 0;
  let mut sound: Option<i64> = None;
  let mut recovered_sound: Option<i64> = None;
  while recovered_sound.is_none() {
    map = do_instruction(
      &instructions[current_index],
      map,
      &mut current_index,
      &mut sound,
      &mut recovered_sound
    );
  };
  println!("Part 1: {}", recovered_sound.unwrap());
}