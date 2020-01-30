const FACTOR_A: u64 = 16807;
const FACTOR_B: u64 = 48271;

fn get_generated_value(value: u64, factor: u64) -> u64 {
  value * factor % (u64::pow(2, 31) - 1)
}

fn get_lowest_16bits(value: u64) -> u64 {
  (((value as u32)  << 16) >> 16 as u64).into()
}

fn main() {
  let mut a: u64 = 703;
  let mut b: u64 = 516;
  let result = (0..40_000_000).fold(
    0,
    |acc, _| {
      a = get_generated_value(a, FACTOR_A);
      b = get_generated_value(b, FACTOR_B);
      if get_lowest_16bits(a) == get_lowest_16bits(b) {
        return acc + 1
      }
      acc
    }
  );
  println!("Part 1: {}", result);
}