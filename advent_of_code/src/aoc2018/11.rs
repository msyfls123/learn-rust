
static PUZZLE_INPUT: i32 = 9995;
static LENGTH: usize = 300;

fn calc_power_level(x: i32, y: i32, grid_serial_number: i32) -> i32 {
  let rack_id = x + 10;
  let mut power_level = rack_id * y;
  power_level += grid_serial_number;
  power_level *= rack_id;
  power_level = (power_level / 100) % 10;
  power_level - 5
}

fn main() {
  let grid: Vec<Vec<i32>> = (1..=LENGTH).map(|y| {
    (1..=LENGTH).map(|x| {
      calc_power_level(x as i32, y as i32, PUZZLE_INPUT)
    }).collect::<Vec<i32>>()
  }).collect();
  let result: (Option<(usize, usize)>, i32) = (0..usize::pow(LENGTH, 2)).fold((None, 0), |acc, index| {
    let x = index % LENGTH;
    let y = index / LENGTH;
    if x < LENGTH - 2 && y < LENGTH - 2 {
      let power: i32 = (x..=x + 2).map(|ix| {
        (y..=y + 2).map(|iy| {
          grid[iy][ix]
        }).sum::<i32>()
      }).sum();
      if power > acc.1 {
        return (Some((x + 1, y + 1)), power);
      }
    }
    acc
  });
  println!("Part 1: the X,Y,size identifier of the square with the largest total power is {:?}.", result.0.unwrap());
}
