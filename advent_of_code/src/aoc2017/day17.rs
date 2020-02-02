fn step(buffer: &Vec<usize>, input: usize) -> Vec<usize> {
  let len = buffer.len();
  let position = buffer.iter().position(|&x| x == len - 1).unwrap();
  let forward_position = (position + input) % len;
  [&buffer[0..=forward_position], &[len], &buffer[forward_position + 1..]].concat()
}

fn main() {
  let puzzle_input: usize = 369;
  let initial_circular_buffer = vec!{0usize};
  let circular_buffer = (0..2017).fold(
    initial_circular_buffer,
    |acc, _| {
      step(&acc, puzzle_input)
    }
  );
  let position_2017 = circular_buffer.iter().position(|&x| x == 2017).unwrap();
  println!("Part 1: {}", circular_buffer[position_2017 + 1]);
}