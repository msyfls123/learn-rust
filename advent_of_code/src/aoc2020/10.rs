use advent_of_code::get_str_array_from_file;

struct Difference {
  j1: i64,
  j2: i64,
  j3: i64
}

struct Adapter {
  prev: i64,
  difference: Difference
}

fn update_difference(difference: &Difference, index: i64) -> Difference {
  match index {
    1 => Difference { j1: difference.j1 + 1, ..*difference },
    2 => Difference { j2: difference.j2 + 1, ..*difference },
    3 => Difference { j3: difference.j3 + 1, ..*difference },
    _ => panic!("not covered"),
  }
}

fn main() {
  let joltage_ratings: Vec<i64> = get_str_array_from_file(&vec!{"aoc2020", "data", "10.txt"})
    .iter()
    .map(|line| {
      line.parse().unwrap()
    })
    .collect();
  let mut joltage_ratings_sorted = joltage_ratings.clone();
  joltage_ratings_sorted.sort();
  let difference = joltage_ratings_sorted.iter().fold(
    Adapter {
      prev: 0,
      difference: Difference {
        j1: 0,
        j2: 0,
        j3: 1,
      }
    },
    |adapter, &x| {
      Adapter {
        prev: x,
        difference: update_difference(&adapter.difference, x - adapter.prev),
      }
    }
  ).difference;
  println!("Part 1: {}", difference.j1 * difference.j3);
}
