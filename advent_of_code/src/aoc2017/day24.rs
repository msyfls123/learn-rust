use advent_of_code::get_str_array_from_file;

type Component = Vec<usize>;

fn max(
  components: &Vec<Component>,
  start: usize,
  used: &Vec<usize>
) -> usize {
  let starts: Vec<(usize, &Component)> = components.iter().enumerate().filter(|(i, x)| {
    (x[0] == start || x[1] == start) && !used.contains(&i)
  }).collect();
  if starts.len() > 0 {
    starts.iter().map(|(i, comp)| {
      let next_start = if comp[0] == start {
        comp[1]
      } else {
        comp[0]
      };
      max(
        &components,
        next_start,
        &[&used[..], &[*i]].concat()
      ) + (comp.iter().sum::<usize>())
    }).max().unwrap()
  } else {
    0
  }
}

fn main() {
  let array = get_str_array_from_file(&vec!{"aoc2017/day24_data.txt"});
  let components: Vec<Component> = array.iter().map(|x| {
    x.split("/").map(|y| {
      y.parse::<usize>().unwrap()
    }).collect()
  }).collect();
  println!("{:?}", components);
  println!("Part 1: {}", max(&components, 0, &vec!{}));
}
