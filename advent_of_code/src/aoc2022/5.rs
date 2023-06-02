#[macro_use] extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::fmt::Display;
use advent_of_code::{split_lines, get_group_str_from_file};

type Stack = Vec<char>;

#[derive(Debug, PartialEq, Eq)]
struct Procedure {
    count: usize,
    from: usize,
    to: usize,
}

impl Procedure {
    fn from_text(text: &String) -> Self {
        lazy_static! {
            static ref RE_STEP: Regex = Regex::new(r"move\s+(?P<count>\d+)\s+from\s+(?P<from>\d+)\s+to\s+(?P<to>\d+)").unwrap();
        }
        let captures = RE_STEP.captures(&text).unwrap();
        let count = captures.name("count").unwrap().as_str().parse().unwrap();
        let from = captures.name("from").unwrap().as_str().parse().unwrap();
        let to = captures.name("to").unwrap().as_str().parse().unwrap();
        Self {
            count,
            from,
            to,
        }
    }
}

#[test]
fn test_create_procedure() {
    let procedure = Procedure::from_text(&String::from("move 1 from 2 to 1"));
    let expected = Procedure {
        count: 1,
        from: 2,
        to: 1,
    };
    assert_eq!(procedure, expected);
}

#[derive(Debug, PartialEq, Eq)]
struct Supply {
    stacks: Vec<Stack>
}

impl Display for Supply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (idx, stack) in self.stacks.iter().enumerate() {
            if idx == 0 {
                continue;
            }
            write!(f, "{}: {:?}", idx, stack)?;
        }
        write!(f, "\n")
    }
}

impl Supply {
    fn from_lines(lines: &Vec<String>) -> Self {
        let (last_line, rest_lines) = lines.split_last().unwrap();
        let last_chars: Stack = last_line.chars().collect();
        let stack_count = last_line.split_whitespace().map(|t| t.parse::<usize>().unwrap()).max().unwrap();
        let mut stacks: Vec<Stack> = vec!{vec!{}; stack_count + 1};
        for line in rest_lines {
            for (idx, ch) in line.chars().enumerate() {
                match ch {
                    'A'..='Z' => {
                        let stack_idx = last_chars[idx].to_string().parse::<usize>().unwrap();
                        stacks[stack_idx].insert(0, ch);
                    },
                    _ => {}
                }
            }
        }
        Self {
            stacks
        }
    }

    fn rearrange(&mut self, procedure: &Procedure) {
        let Procedure { count, from, to } = procedure.to_owned();
        for _ in 0..*count {
            let ele = self.stacks[*from].pop();
            if ele.is_some() {
                self.stacks[*to].push(ele.unwrap());
            }
        }
    }

    fn get_top_crates(&self) -> String {
        self.stacks.iter().filter_map(|stack| stack.last()).collect()
    }
}

#[test]
fn test_supply_create() {
    let data = split_lines(r#"[D]        
[N] [C]    
[Z] [M] [P]
 1   2   3 "#);
    let supply = Supply::from_lines(&data);
    let expected = Supply {
        stacks: vec!{
            vec!{},
            vec!{'Z', 'N', 'D'},
            vec!{'M', 'C'},
            vec!{'P'},
        }
    };
    assert_eq!(supply, expected);
}

#[test]
fn test_rearrange() {
    let data = split_lines(r#"[D]        
[N] [C]    
[Z] [M] [P]
 1   2   3 "#);
    let mut supply = Supply::from_lines(&data);
    let procedure = Procedure::from_text(&String::from("move 3 from 1 to 3"));
    let expected = Supply {
        stacks: vec!{
            vec!{},
            vec!{},
            vec!{'M', 'C'},
            vec!{'P', 'D', 'N', 'Z'},
        }
    };
    supply.rearrange(&procedure);
    assert_eq!(supply, expected);
}

fn main() {
    let data = get_group_str_from_file(&vec!{"aoc2022", "data", "5.txt"});
    let mut supply = Supply::from_lines(&data[0]);
    let procedures: Vec<Procedure> = data[1].iter().map(|line| Procedure::from_text(&line)).collect();

    for procedure in procedures {
        supply.rearrange(&procedure);
    }

    println!("Part 1: {}", supply.get_top_crates());
}
