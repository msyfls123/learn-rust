use advent_of_code::get_str_array_from_file;

const PERIODS: [usize; 6] = [
    20,
    60,
    100,
    140,
    180,
    220
];

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    Addx(isize),
    Noop
}

impl Instruction {
    fn from(text: &str) -> Self {
        match &text[0..4] {
            "noop" => Self::Noop,
            "addx" => {
                let value = text[5..].parse::<isize>().unwrap();
                Self::Addx(value)
            },
            _ => panic!("can not convert to Instruction")
        }
    }
}

#[test]
fn test_instruction_from_text() {
    assert_eq!(Instruction::Addx(5), Instruction::from("addx 5"));
    assert_eq!(Instruction::Addx(-6), Instruction::from("addx -6"));
    assert_eq!(Instruction::Noop, Instruction::from("noop"));
}

#[derive(Debug, Clone)]
struct Register {
    value: isize,
    cycles: usize,
}

impl Register {
    fn run(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Noop => {
                self.cycles += 1;
            },
            Instruction::Addx(x) => {
                self.cycles += 2;
                self.value += x;
            }
        }
    }
}

fn main() {
    let data = get_str_array_from_file(&vec!{"aoc2022", "data", "10.txt"});
    let instructions: Vec<Instruction> = data.iter().map(|text| Instruction::from(text)).collect();

    let mut register = Register {
        value: 1,
        cycles: 0,
    };
    let mut sum = 0;
    let mut index = 0;
    while register.cycles < PERIODS[5] {
        let prev_register = register.to_owned();
        let instruction = &instructions[index % instructions.len()];
        register.run(&instruction);

        for period in PERIODS {
            let prev_period = period - 1;
            if register.cycles == prev_period {
                sum += register.value * (period as isize)
            } else if register.cycles > prev_period && prev_register.cycles < prev_period {
                sum += prev_register.value * (period as isize)
            }
        }

        index += 1;
    }
    println!("Part 1: {}", sum);
}