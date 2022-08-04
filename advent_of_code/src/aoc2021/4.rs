use advent_of_code::get_group_str_from_file;
use itertools::Itertools;

type Board = Vec<Vec<usize>>;

struct Winner {
  board: Board,
  unmarked: Vec<usize>,
}

fn check_winner(
  board: &Board,
  drawn: &Vec<&usize>,
) -> Option<Winner> {
  let width = board[0].len();
  let height = board.len();
  let row_checked = (0..height).any(|y| {
    (0..width).all(|x| drawn.contains(&&board[y][x]))
  });
  let column_checked = (0..width).any(|x| {
    (0..height).all(|y| drawn.contains(&&board[y][x]))
  });
  if row_checked || column_checked {
    let unmarked = board.iter().flatten().filter_map(|x| {
      if drawn.contains(&x) {
        None
      } else {
        Some(x.to_owned())
      }
    }).collect_vec();
    Some(Winner {
      board: board.to_owned(),
      unmarked,
    })
  } else {
    None
  }
}

fn main() {
  let data = get_group_str_from_file(&vec!{"aoc2021", "data", "4.txt"});
  let numbers = data[0][0].split(",").map(|v| v.parse::<usize>().unwrap()).collect_vec();
  let boards = data.iter().skip(1).map(|rows| {
    rows.iter().map(|row| row.split_whitespace().map(|v| v.parse::<usize>().unwrap()).collect_vec()).collect_vec()
  }).collect_vec();

  let (winner, num) = (0..numbers.len()).find_map(|i| {
    let drawn = numbers.iter().take(i + 1).collect_vec();
    let option_winner = boards.iter().find_map(|board| check_winner(board, &drawn));
    match option_winner {
      Some(winner) => Some((winner, numbers[i])),
      None => None,
    }
  }).unwrap();
  let score = winner.unmarked.iter().sum::<usize>() * num;
  println!("Part 1: {}", score);
}
