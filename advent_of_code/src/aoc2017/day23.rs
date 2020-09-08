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
  mul_called_times: &mut usize
) -> MapType {
  let gv = get_value;
  let sv = set_value;
  match instruction[0] {
    "set" => {
      let value = gv(&map, instruction[2]);
      sv(&mut map, instruction[1], value);
      *current_index += 1;
    },
    "sub" => {
      let t1 = instruction[1];
      let t2 = instruction[2];
      let value = gv(&map, t1) - gv(&map, t2);
      sv(&mut map, t1, value);
      *current_index += 1;
    }
    "mul" => {
      let t1 = instruction[1];
      let t2 = instruction[2];
      let value = gv(&map, t1) * gv(&map, t2);
      sv(&mut map, t1, value);
      *mul_called_times += 1;
      *current_index += 1;
    },
    "mod" => {
      let t1 = instruction[1];
      let t2 = instruction[2];
      let value = gv(&map, t1) % get_value(&map, t2);
      sv(&mut map, t1, value);
      *current_index += 1;
    },
    "jnz" => {
      let v1 = gv(&map, instruction[1]);
      let v2 = gv(&map, instruction[2]);
      if v1 != 0 {
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
  let array = get_str_array_from_file(&["aoc2017", "day23_data.txt"].to_vec());
  let instructions: Vec<Vec<&str>> = array.iter().map(
    |x| x.split(" ").collect()
  ).collect();
  // Part 1
  let mut map: MapType = HashMap::new();
  let mut current_index: usize = 0;
  let mut mul_called_times: usize = 0;
  let instructions_len = instructions.len();
  while current_index < instructions_len {
    map = do_instruction(
      &instructions[current_index],
      map,
      &mut current_index,
      &mut mul_called_times,
    );
  };
  println!("Part 1: {}", mul_called_times);
  // Part 2
  let mut map: MapType = HashMap::new();
  map.entry('a').or_insert(1);
  let mut current_index: usize = 0;
  let mut mul_called_times: usize = 0;
  let instructions_len = instructions.len();
  while current_index < instructions_len {
    map = do_instruction(
      &instructions[current_index],
      map,
      &mut current_index,
      &mut mul_called_times,
    );
    println!("{}", *map.entry('g').or_insert(0));
    println!("{}", *map.entry('h').or_insert(0));
  };
}
