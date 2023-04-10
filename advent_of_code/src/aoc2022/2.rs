use advent_of_code::get_str_array_from_file;
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors
}

#[derive(Debug, PartialEq, Eq)]
struct Strategy {
    opponent: Shape,
    you: Shape,
}

impl From<&str> for Strategy {
    fn from(text: &str) -> Self {
        let (x, y): (&str, &str) = text.split_whitespace().collect_tuple().unwrap();
        let opponent = match x {
            "A" => Shape::Rock,
            "B" => Shape::Paper,
            "C" => Shape::Scissors,
            _ => panic!("parse opponent failed")
        };
        let you = match y {
            "X" => Shape::Rock,
            "Y" => Shape::Paper,
            "Z" => Shape::Scissors,
            _ => panic!("parse you failed")
        };
        Self {
            opponent,
            you
        }
    }
}

#[test]
fn test_from_string() {
    let strategy: Strategy = "A X".into();
    assert_eq!(
        strategy,
        Strategy {
            opponent: Shape::Rock,
            you: Shape::Rock
        },
    )
}

impl Strategy {
    fn from_round_end(text: &str) -> Self {
        use Shape::*;
        let (x, y): (&str, &str) = text.split_whitespace().collect_tuple().unwrap();
        let opponent = match x {
            "A" => Shape::Rock,
            "B" => Shape::Paper,
            "C" => Shape::Scissors,
            _ => panic!("parse opponent failed")
        };
        let you = match y {
            "X" => match opponent {
                Rock => Scissors,
                Paper => Rock,
                Scissors => Paper,
            },
            "Y" => opponent,
            "Z" => match opponent {
                Rock => Paper,
                Paper => Scissors,
                Scissors => Rock,
            },
            _ => panic!("parse you failed")
        };
        Self {
            opponent,
            you
        }
    }

    fn shape_score(&self) -> u32 {
        match self.you {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }

    fn round_outcome(&self) -> u32 {
        use Shape::*;
        if self.opponent == self.you {
            return 3;
        }
        match (&self.opponent, &self.you) {
            (Scissors, Rock) => 6,
            (Paper, Scissors) => 6,
            (Rock, Paper) => 6,
            _ => 0,
        }
    }

    fn score(&self) -> u32 {
        self.round_outcome() + self.shape_score()
    }
}

#[test]
fn test_score() {
    let strategy: Strategy = "A Y".into();
    assert_eq!(strategy.score(), 8);
    let strategy: Strategy = "B X".into();
    assert_eq!(strategy.score(), 1);
    let strategy: Strategy = "C Z".into();
    assert_eq!(strategy.score(), 6);
}

fn main() {
    let data = get_str_array_from_file(&vec!{"aoc2022", "data", "2.txt"});
    let strategies: Vec<Strategy> = data.iter().map(|text| text.as_str().into()).collect();
    let total_score: u32 = strategies.iter().map(|s| s.score()).sum();
    println!("Part 1: {}", total_score);

    let strategies: Vec<Strategy> = data.iter().map(|text| Strategy::from_round_end(text)).collect();
    let total_score: u32 = strategies.iter().map(|s| s.score()).sum();
    println!("Part 2: {}", total_score);
}
