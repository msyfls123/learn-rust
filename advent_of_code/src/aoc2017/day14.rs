use advent_of_code::aoc2017::day10::get_knot_hash;

fn main() {
  let input = "ugkiagan";
  let hashes: Vec<u128> = (0..128).into_iter().map(|x| get_knot_hash(&format!("{}-{}", input, x)[..])).collect();
  let total_used_squares = hashes.iter().fold(
    0,
    |acc, x| {
      let used_squares = format!("{:0128b}", x).split("").filter(|&x| x == "1").count();
      acc + used_squares
    }
  );
  println!("Part 1: {}", total_used_squares);
}