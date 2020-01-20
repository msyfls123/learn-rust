use advent_of_code::get_str_array_from_file;
use std::collections::HashMap;

fn main() {
  let path_list = vec!{"aoc2017", "day12_data.txt"};
  let array = get_str_array_from_file(&path_list);
  let pairs: Vec<Vec<&str>> = array.iter().map(|x| x.split(" <-> ").collect()).collect();
  let mut map = HashMap::new();
  for pair in pairs.into_iter() {
    // match &pair[..] {
    //   [] => println!("empty"),
    //   [key, value] => println!("{}: {}", key, value),
    //   _ => unreachable!()
    // }
    if let [key, value] = &pair[..] {
      let list: Vec<i64> = value.split(", ").map(|x| x.parse::<i64>().unwrap()).collect();
      map.insert(key.parse::<i64>().unwrap(), list);
    }
  }
  let mut contacts = map.get(&0).unwrap().clone();
  let mut current_contacts = contacts.clone();
  contacts.insert(0, 0);
  while !current_contacts.is_empty() {
    let copied_current_contacts = current_contacts.clone();
    current_contacts.clear();
    for contact in copied_current_contacts.into_iter() {
      for person in map.get(&contact).unwrap().into_iter() {
        if !contacts.iter().any(|x| x == person) {
          current_contacts.push(*person);
          contacts.push(*person);
        }
      }
    }
  }
  println!("Part 1: {}", contacts.len());
}