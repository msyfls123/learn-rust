const FACTOR_A: u64 = 16807;
const FACTOR_B: u64 = 48271;

fn get_generated_value(value: u64, factor: u64, divisor: u64) -> u64 {
  let result = value * factor % (u64::pow(2, 31) - 1);
  if result % divisor == 0 {
    result
  } else {
    get_generated_value(result, factor, divisor)
  }
}

fn get_lowest_16bits(value: u64) -> u64 {
  (((value as u32)  << 16) >> 16 as u64).into()
}

fn main() {
  let a: u64 = 703;
  let b: u64 = 516;
  let result = (0..40_000_000).fold(
    (0, a, b),
    |(acc, a, b), _| {
      let _a = get_generated_value(a, FACTOR_A, 1);
      let _b = get_generated_value(b, FACTOR_B, 1);
      if get_lowest_16bits(_a) == get_lowest_16bits(_b) {
        return (acc + 1, _a, _b)
      }
      (acc, _a, _b)
    }
  );
  println!("Part 1: {}", result.0);
  let result2 = (0..5_000_000).fold(
    (0, a, b),
    |(acc, a, b), _| {
      let _a = get_generated_value(a, FACTOR_A, 4);
      let _b = get_generated_value(b, FACTOR_B, 8);
      if get_lowest_16bits(_a) == get_lowest_16bits(_b) {
        return (acc + 1, _a, _b)
      }
      (acc, _a, _b)
    }
  );
  println!("Part 2: {}", result2.0);
}