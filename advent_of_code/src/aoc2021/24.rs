use std::{collections::{HashSet}};

use advent_of_code::get_str_array_from_file;

type Unit = [isize; 4];

/**
 * Failed unit
 */
type Cache = HashSet<Unit>;

const SORTED_LARGE_INPUTS: [isize;9] = [9,8,7,6,5,4,3,2,1];
const SORTED_SMALL_INPUTS: [isize;9] = [1,2,3,4,5,6,7,8,9];

#[derive(Debug, PartialEq, Eq)]
struct InstructionResult {
    unit: Unit,
    consumed: bool
}

fn split(text: &str) -> Vec<&str> {
    text.split(" ").collect()
}

fn instruction(unit: &Unit, seps: &Vec<&str>, input: isize) -> InstructionResult {
    let get = |v: &str| {
        match v.parse::<isize>() {
            Ok(num) => num,
            Err(_) => {
                let key = v.chars().next().unwrap() as u8 - 119;
                unit[key as usize]
            }
        }
    };
    let set = |v: &str, data: isize| -> Unit {
        let key = (v.chars().next().unwrap() as u8 - 119) as usize;
        let mut new_unit = *unit;
        new_unit[key] = data;
        new_unit
    };
    let mut consumed = false;

    let new_unit = match seps[0] {
        "inp" => {
            consumed = true;
            set(seps[1], input)
        },
        "add" => {
            set(
                seps[1],
                get(seps[1]) + get(seps[2])
            )
        },
        "mul" => {
            set(
                seps[1],
                get(seps[1]) * get(seps[2])
            )
        },
        "div" => {
            let result = get(seps[1]) / get(seps[2]);
            set(seps[1], result)
        },
        "mod" => {
            set(
                seps[1],
                get(seps[1]) % get(seps[2])
            )
        },
        "eql" => {
            let result = if get(seps[1]) == get(seps[2]) {
                1
            } else { 0 };
            set(seps[1], result)
        },
        _ => panic!("cannot believe it")
    };
    // println!("{} - {}\ninput:{:#?}\noutput:{:#?}\n\n", text, input, unit, new_unit);
    InstructionResult {
        unit: new_unit,
        consumed,
    }
}

#[test]
fn test_instruction() {
    assert_eq!(
        instruction(&[0,0,0,0], &split("inp x"), 5),
        InstructionResult {
            unit: [0,5,0,0],
            consumed: true,
        }
    );
    assert_eq!(
        instruction(&[3,1,0,0], &split("add x w"), 5),
        InstructionResult {
            unit: [3,4,0,0],
            consumed: false,
        }
    );
    assert_eq!(
        instruction(&[0,0,3,2], &split("mul y z"), 5),
        InstructionResult {
            unit: [0,0,6,2],
            consumed: false,
        }
    );
}

fn find_sat_input(
    instructions: &Vec<Vec<&str>>,
    mut unit: Unit,
    mut index: usize,
    failed_cache: &mut [Cache],
    is_large: bool
) -> Option<Vec<isize>> {
    let (init_unit, init_index) = (unit, index);
    if failed_cache[init_index].contains(&init_unit) {
        return None
    }
    let total = instructions.len();
    while index < total {
        let instruction_text = &instructions[index];
        let is_inp = instruction_text[0] == "inp";
        let inputs = if is_large {
            SORTED_LARGE_INPUTS
        } else {
            SORTED_SMALL_INPUTS
        };
        if is_inp {
            for input in inputs {
                InstructionResult { unit, .. } = instruction(&unit, &instruction_text, input);
                if let Some(res) = find_sat_input(instructions, unit, index + 1, failed_cache, is_large) {
                    return Some([vec!{input}, res].concat())
                }
                // debug, show first two inp results
                if index < 20 {
                    println!("fail {} {}", index, input);
                }
            }
            // all inputs failed, mark this init_unit as invalid
            failed_cache[init_index].insert(init_unit);
            return None;
        } else {
            InstructionResult { unit, .. } = instruction(&unit, &instruction_text, 0);
        }
        index += 1;
    }
    if unit[3] == 0 {
        Some(vec!{})
    } else {
        failed_cache[init_index].insert(init_unit);
        None
    }
}

fn main() {
    let data = get_str_array_from_file(&vec!{"aoc2021", "data", "24.txt"});
    let instructions: Vec<Vec<&str>> = data.iter().map(|text| text.split(" ").collect()).collect();
    let mut cache = vec![Cache::new(); instructions.len()];
    let largest = find_sat_input(&instructions, [0,0,0,0], 0, &mut cache, true);
    println!("Part 1: {:?}", largest.map(|nums| {
        nums.iter().map(|num| num.to_string()).collect::<String>()
    }));

    let mut cache = vec![Cache::new(); instructions.len()];
    // will take a long long time...
    let smallest = find_sat_input(&instructions, [0,0,0,0], 0, &mut cache, false);
    println!("Part 2: {:?}", smallest.map(|nums| {
        nums.iter().map(|num| num.to_string()).collect::<String>()
    }));
}
