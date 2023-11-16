use std::fmt::Debug;

use itertools::Itertools;

trait Operation: Fn(usize) -> usize {}

impl<F> Operation for F where F: Fn(usize) -> usize { }

impl std::fmt::Debug for dyn Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Operation {{...}}")
    }
}

trait Test: Fn(usize) -> bool {}

impl<F> Test for F where F: Fn(usize) -> bool { }

impl std::fmt::Debug for dyn Test {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Test {{...}}")
    }
}

#[derive(Debug)]
struct Monkey {
    id: usize,
    staring_items: Vec<usize>,
    operation: Box<dyn Operation>,
    test: Box<dyn  Test>,
    true_dest: usize,
    false_dest: usize,
    inspect_count: usize,
}

#[derive(Debug, PartialEq, Eq)]
struct ThrownInfo {
    id: usize,
    worry_level: usize
}

impl Monkey {
    fn round(&mut self) -> Vec<ThrownInfo> {
        self.inspect_count += self.staring_items.len();
        let throw_info_list = self.staring_items.iter().map(|&item| {
            let mut worry_level = (self.operation)(item);
            worry_level /= 3;
            if (self.test)(worry_level) {
                ThrownInfo { worry_level, id: self.true_dest }
            } else {
                ThrownInfo { worry_level, id: self.false_dest }
            }

        }).collect();
        self.staring_items.clear();
        throw_info_list
    }

    fn accept(&mut self, thrownInfo: &ThrownInfo) {
        self.staring_items.push(thrownInfo.worry_level)
    }
}

#[test]
fn test_round() {
    let mut monkey = Monkey {
        id: 0,
        staring_items: vec!{79, 98},
        operation: Box::new(|x| x * 19),
        test: Box::new(|x| x % 23 == 0),
        true_dest: 2,
        false_dest: 3,
        inspect_count: 0
    };

    let expected_thrown_info_list = vec!{
        ThrownInfo { id: 3, worry_level: 500 },
        ThrownInfo { id: 3, worry_level: 620 },
    };

    assert_eq!(monkey.round(), expected_thrown_info_list);
}

fn create_monkeys() -> Vec<Monkey> {
    vec!{
        Monkey {
            id: 0,
            staring_items: vec!{89, 74},
            operation: Box::new(|x| x * 5),
            test: Box::new(|x| x % 17 == 0),
            true_dest: 4,
            false_dest: 7,
            inspect_count: 0
        },
        Monkey {
            id: 1,
            staring_items: vec!{75, 69, 87, 57, 84, 90, 66, 50},
            operation: Box::new(|x| x + 3),
            test: Box::new(|x| x % 7 == 0),
            true_dest: 3,
            false_dest: 2,
            inspect_count: 0
        },
        Monkey {
            id: 2,
            staring_items: vec!{55},
            operation: Box::new(|x| x + 7),
            test: Box::new(|x| x % 13 == 0),
            true_dest: 0,
            false_dest: 7,
            inspect_count: 0
        },
        Monkey {
            id: 3,
            staring_items: vec!{69, 82, 69, 56, 68},
            operation: Box::new(|x| x + 5),
            test: Box::new(|x| x % 2 == 0),
            true_dest: 0,
            false_dest: 2,
            inspect_count: 0
        },
        Monkey {
            id: 4,
            staring_items: vec!{72, 97, 50},
            operation: Box::new(|x| x + 2),
            test: Box::new(|x| x % 19 == 0),
            true_dest: 6,
            false_dest: 5,
            inspect_count: 0
        },
        Monkey {
            id: 5,
            staring_items: vec!{90, 84, 56, 92, 91, 91},
            operation: Box::new(|x| x * 19),
            test: Box::new(|x| x % 3 == 0),
            true_dest: 6,
            false_dest: 1,
            inspect_count: 0
        },
        Monkey {
            id: 6,
            staring_items: vec!{63, 93, 55, 53},
            operation: Box::new(|x| x * x),
            test: Box::new(|x| x % 5 == 0),
            true_dest: 3,
            false_dest: 1,
            inspect_count: 0
        },
        Monkey {
            id: 7,
            staring_items: vec!{50, 61, 52, 58, 86, 68, 97},
            operation: Box::new(|x| x + 4),
            test: Box::new(|x| x % 11 == 0),
            true_dest: 5,
            false_dest: 4,
            inspect_count: 0
        },
    }
}

fn main() {
    let mut monkeys = create_monkeys();
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let thrown_info_list = monkeys[i].round();
            for info in thrown_info_list {
                monkeys[info.id].accept(&info)
            }
        }
    }
    let (first, second) = monkeys.iter()
        .map(|m| m.inspect_count)
        .sorted()
        .rev()
        .take(2).collect_tuple().unwrap();
    println!("Part 1: {}", first * second);
}