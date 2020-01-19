use advent_of_code::get_str_array_from_file;

fn main() {
  let path_list = vec!{"aoc2017", "day12_data.txt"};
  let array = get_str_array_from_file(path_list);
  println!("{:?}", array.len());
}