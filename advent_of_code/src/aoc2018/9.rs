use std::collections::HashMap;

fn get_position(
  size: usize,
  start: i32,
  step: i32,
  to_insert: bool,
) -> usize {
  let size_i32 = size as i32;
  let dist = start + step;
  let check = if to_insert {
    dist > 0 && dist <= size_i32
  } else {
    dist >= 0 && dist < size_i32
  };
  if check {
    dist as usize
  } else {
    if dist < 0 {
      get_position(size, dist + size_i32, 0, to_insert)
    } else {
      get_position(size, dist - size_i32, 0, to_insert)
    }
  }
}

fn get_score(count: usize, last: usize) -> usize {
  let mut circle = vec!{0};
  let mut play_id = 0;
  let mut marble = 0;
  let mut index = 0;
  let mut scores: HashMap<usize, usize> = HashMap::new();
  loop {
    let len = circle.len();
    if len == 0 || marble == last {
      break
    }
    play_id = (play_id + 1) % count;
    marble += 1;
    if marble % 23 == 0 {
      index = get_position(len, index as i32, -7, false);
      let removed = circle.remove(index);
      if index == len - 1 {
        println!("last");
        index = 0;
      }
      let entry = scores.entry(play_id).or_insert(0);
      *entry += removed + marble;
    } else {
      index = get_position(len, index as i32, 2, true);
      circle.insert(index, marble);
    }
  }
  let high_score = scores.iter().map(|s| s.1).max().unwrap();
  *high_score
}

fn main() {
  println!("{}", get_score(10, 1618));
  println!("{}", get_score(13, 7999));
  println!("{}", get_score(17, 1104));
  println!("{}", get_score(21, 6111));
  println!("{}", get_score(30, 5807));
  println!("Part 1: the winning Elf's score is {}.", get_score(424, 71144));
}
