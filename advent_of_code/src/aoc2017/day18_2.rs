use std::collections::HashMap;
use advent_of_code::get_str_array_from_file;

type MapType = HashMap<char, i64>;

#[derive(Debug)]
struct Program<'a> {
    map: MapType,
    instructions: &'a Vec<Vec<&'a str>>,
    current_index: usize,
    send_times: usize,
    received_values: Vec<i64>,
    id: usize,
}

impl<'a> Program<'a> {
    fn new(id: usize, instructions:  &'a Vec<Vec<&str>>) -> Self {
        Program{
            id,
            instructions,
            map: HashMap::new(),
            current_index: 0,
            send_times: 0,
            received_values: vec!{},
        }
    }

    fn send(&mut self, other: &mut Self) {
        self.send_times += 1;
        other.received_values.push(1);
    }
}

fn main() {
    let array = get_str_array_from_file(&["aoc2017", "day18_data.txt"].to_vec());
    let instructions: Vec<Vec<&str>> = array.iter().map(
        |x| x.split(" ").collect()
    ).collect();
    let mut program0 = Program::new(0, &instructions);
    let mut program1 = Program::new(1, &instructions);
    program0.send(&mut program1);
    println!("{:?}", program1);
    println!("{:?}", program0);
}