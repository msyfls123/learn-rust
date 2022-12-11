use std::collections::HashMap;

type RoundPawnMap = HashMap<usize, HashMap<Player,usize>>;

#[derive(Debug, Hash, Eq, Clone, Copy)]
struct Player {
    space: usize,
    score: usize
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score && self.space == other.space
    }
}

impl Player {
    fn roll(&self, roll_spaces: &Vec<usize>) -> Self {
        let stop_space = self.space + roll_spaces.iter().sum::<usize>();
        let new_space = if stop_space % 10 == 0 {
            10
        } else {
            stop_space % 10
        };
        Self {
            score: self.score + new_space,
            space: new_space,
        }
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
            player1 = player1.roll(&roll_spaces);
        } else {
            player2 = player2.roll(&roll_spaces);
        }
        round += 1;
    }
    round * 3 * [player1.score, player2.score].iter().find(|&x| *x < 1000).unwrap()
}

#[test]
fn test_deterministic_game() {
    assert_eq!((deterministic_game(4, 8)), 739785);
}

fn get_round_pawn_map(rounds: usize, limit: usize, start_position: usize) -> RoundPawnMap {
    let mut map = HashMap::new();
    map.insert(0, HashMap::from([(Player {
        score: 0,
        space: start_position
    }, 1)]));
    for prev in 0..rounds {
        let current = prev + 1;
        let mut round_map = HashMap::new();
        for (&player, &count) in map.get(&prev).unwrap().iter().filter(|(&player, _)| player.score < limit) {
            for i in 1..=3 {
                for j in 1..=3 {
                    for k in 1..=3 {
                        *round_map.entry(player.roll(&vec!{i + j + k})).or_insert(0) += count;
                    }
                }
            }
        }
        map.insert(current, round_map);
    }
    map
}

fn main() {
    let score = deterministic_game(1, 10);
    println!("Part 1: {}", score);

    let player1_map = get_round_pawn_map(10, 21, 1);
    let player2_map = get_round_pawn_map(10, 21, 10);
    let player1_win_universes = player1_map.iter().filter(|(&round, _)| round > 0).fold(0, |total, (round, map)| {
        total + map.iter().filter(|(&x, _)| x.score>= 21).map(|(_, v)| *v).sum::<usize>() * player2_map.get(&(round - 1)).map_or(
            1,
            |map2| map2.iter().filter(|(&x, _)| x.score < 21).map(|(_, v)| *v).sum(),
        )
    });
    let player2_win_universes = player2_map.iter().filter(|(&round, _)| round > 0).fold(0, |total, (round, map)| {
        total + map.iter().filter(|(&x, _)| x.score>= 21).map(|(_, v)| *v).sum::<usize>() * player1_map.get(&(round)).map_or(
            1,
            |map1| map1.iter().filter(|(&x, _)| x.score < 21).map(|(_, v)| *v).sum(),
        )
    });
    println!("Part 2: {}", player1_win_universes.max(player2_win_universes));
}