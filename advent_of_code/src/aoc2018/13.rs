use advent_of_code::get_str_array_from_file;

struct Cart<'a> {
  speed: (isize, isize),
  position: (isize, isize),
  met_intersection: usize,
  map: &'a Vec<Vec<char>>
}

impl Cart<'_> {
  fn meet_intersection(&self) -> (isize, isize) {
    let turns = (self.met_intersection + 2) % 4;
    let mut speed = self.speed.clone();
    (0..turns).for_each(|_| {
      speed = (speed.1, -speed.0)
    });
    speed
  }
  fn moves(&mut self) {
    let (x, y) = self.position;
    let (vx, vy) = self.speed;
    self.position = (x + vx, y + vy);
    let (x, y) = self.position;
    match self.map[y as usize][x as usize] {
      '+' => {
        self.met_intersection += 1;
        self.speed = self.meet_intersection();
      },
      '/' => {
        if vx != 0 {
          self.speed = (-vy, vx)
        } else {
          self.speed = (vy, -vx)
        }
      },
      '\\' => {
        if vx != 0 {
          self.speed = (vy, -vx)
        } else {
          self.speed = (-vy, vx)
        }
      },
      _ => {},
    }
  }
}

fn main() {
  let array = get_str_array_from_file(&vec!["aoc2018", "data", "13.txt"]);
  let map: Vec<Vec<char>> = array.iter().map(|text| text.chars().collect()).collect();
  println!("{:?}", map);
}
