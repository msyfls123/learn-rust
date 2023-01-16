use std::{collections::HashMap, convert::TryInto};

type Unit = [isize; 4];

/**
 * One inp step to next result
 */
type OperationCache = HashMap<Unit, Unit>;

/**
 * Every inp step to next (or final) result
 */
type ProgramCache = HashMap<usize, OperationCache>;

#[derive(Debug, PartialEq, Eq)]
struct InstructionResult {
    unit: Unit,
    consumed: bool
}

fn instruction(unit: Unit, text: &str, input: isize) -> InstructionResult {
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
        unit.iter().enumerate().map(|(index, val)| {
            if index == key {
                data
            } else {
                *val
            }
        }).collect::<Vec<isize>>().try_into().unwrap()
    };
    let mut consumed = false;
    let seps: Vec<&str> = text.split(" ").collect();

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
    InstructionResult {
        unit: new_unit,
        consumed,
    }
}

#[test]
fn test_instruction() {
    assert_eq!(
        instruction([0,0,0,0], "inp x", 5),
        InstructionResult {
            unit: [0,5,0,0],
            consumed: true,
        }
    );
    assert_eq!(
        instruction([3,1,0,0], "add x w", 5),
        InstructionResult {
            unit: [3,4,0,0],
            consumed: false,
        }
    );
    assert_eq!(
        instruction([0,0,3,2], "mul y z", 5),
        InstructionResult {
            unit: [0,0,6,2],
            consumed: false,
        }
    );
}

fn main() {
}
