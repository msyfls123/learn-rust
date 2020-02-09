use std::collections::HashMap;
use super::super::get_str_array_from_file;

type MapType = HashMap<char, i64>;

#[derive(Debug)]
enum Value {
  Register(char),
  Int(i64),
}

fn get_raw_value(text: &str) -> Value {
  match text.parse::<i64>() {
    Ok(v) => Value::Int(v),
    _ => match text.parse::<char>() {
      Ok(v) => Value::Register(v),
      _ => panic!(format!("cannot parse text {}", text)),
    },
  }
}

#[derive(Debug)]
struct Program<'a> {
    map: MapType,
    instructions: &'a Vec<Vec<&'a str>>,
    current_index: usize,
    send_times: usize,
    received_values: Vec<i64>,
    deadlock: bool,
    id: usize,
}

impl<'a> Program<'a> {
    fn new(id: usize, instructions:  &'a Vec<Vec<&str>>) -> Self {
        let mut map = HashMap::new();
        map.insert('p', id as i64);
        Program{
            id,
            instructions,
            map,
            current_index: 0,
            send_times: 0,
            received_values: vec!{},
            deadlock: false,
        }
    }

    fn get(&self, text: &str) -> i64 {
        match get_raw_value(text) {
          Value::Int(v) => v,
          Value::Register(c) => match self.map.get(&c) {
            Some(&v) => v,
            None => 0,
          }
        }
    }

    fn update_current_index(&mut self, diff: i64) {
        self.current_index = ((self.current_index as i64) + diff) as usize;
    }

    fn set(&mut self, text: &str, value: i64) {
        match get_raw_value(text) {
          Value::Register(c) => {
            let entry = self.map.entry(c).or_insert(0);
            *entry = value;
          },
          _ => (),
        }
    }

    fn send(&mut self, other: &mut Self, key: &str) {
        self.send_times += 1;
        other.received_values.insert(0, self.get(key));
    }

    fn run(&mut self, other: &mut Self) {
        let ins = &self.instructions[self.current_index];
        match ins[0] {
            "snd" => {
                self.send(other, ins[1]);
                self.update_current_index(1);
            },
            "set" => {
                self.set(ins[1], self.get(ins[2]));
                self.update_current_index(1);
            },
            "add" => {
                let value = self.get(ins[1]) + self.get(ins[2]);
                self.set(ins[1], value);
                self.update_current_index(1);
            },
            "mul" => {
                let value = self.get(ins[1]) * self.get(ins[2]);
                self.set(ins[1], value);
                self.update_current_index(1);
            },
            "mod" => {
                let value = self.get(ins[1]) % self.get(ins[2]);
                self.set(ins[1], value);
                self.update_current_index(1);
            },
            "rcv" => {
                match self.received_values.pop() {
                    Some(v) => {
                        self.deadlock = false;
                        self.set(ins[1], v);
                        self.update_current_index(1);
                    },
                    None => {
                        self.deadlock = true;
                    },
                };
            },
            "jgz" => {
                if self.get(ins[1]) > 0 {
                    self.update_current_index(self.get(ins[2]))
                } else {
                    self.update_current_index(1)
                }
            },
            _ => ()
        }
    }
}

pub fn resolve() {
    let array = get_str_array_from_file(&["aoc2017", "day18_data.txt"].to_vec());
    let instructions: Vec<Vec<&str>> = array.iter().map(
        |x| x.split(" ").collect()
    ).collect();
    let mut program0 = Program::new(0, &instructions);
    let mut program1 = Program::new(1, &instructions);
    // let programs = vec![&program0, &program1];
    let mut current = 0;
    while !(program0.deadlock &&
        program1.deadlock &&
        program0.received_values.is_empty() &&
        program1.received_values.is_empty()) {
        if current == 0 {
            program0.run(&mut program1);
            if program0.deadlock {
                current = 1
            }
        } else {
            program1.run(&mut program0);
            if program1.deadlock {
                current = 0
            } 
        };
    }
    println!("Part 2: {}", program1.send_times);
}