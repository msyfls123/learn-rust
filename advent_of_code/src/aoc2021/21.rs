#[derive(Debug)]
struct Player {
    space: usize,
    score: usize
}

impl Player {
    fn roll(&mut self, roll_spaces: &Vec<usize>) {
        let stop_space = self.space + roll_spaces.iter().sum::<usize>();
        let new_space = if stop_space % 10 == 0 {
            10
        } else {
            stop_space % 10
        };
        self.score += new_space;
        self.space = new_space;
    }
}

fn deterministic_game(player1_pos: usize, player2_pos: usize) -> usize {
    let mut round = 0;
    let mut player1 = Player {
        score: 0,
        space: player1_pos,
    };
    let mut player2 = Player {
        score: 0,
        space: player2_pos,
    };

    while player1.score < 1000 && player2.score < 1000 {
        let roll_spaces = (1..=3).map(|x| x + round * 3)
            .collect();
        if round % 2 == 0 {
            player1.roll(&roll_spaces);
        } else {
            player2.roll(&roll_spaces);
        }
        round += 1;
    }
    round * 3 * [player1.score, player2.score].iter().find(|&x| *x < 1000).unwrap()
}

#[test]
fn test_deterministic_game() {
    assert_eq!((deterministic_game(4, 8)), 739785);
}

fn main() {
    let score = deterministic_game(1, 10);
    println!("Part 1: {}", score);
}