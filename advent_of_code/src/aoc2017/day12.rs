use advent_of_code::get_str_array_from_file;
use std::collections::HashMap;

fn get_full_contacts(map: &HashMap<i64, Vec<i64>>, key: i64) -> Vec<i64> {
  let mut contacts = map.get(&key).unwrap().clone();
  let mut current_contacts = contacts.clone();
  contacts.insert(0, key);
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
  };
  contacts
}

fn without(list1: Vec<i64>, list2: Vec<i64>) -> Vec<i64> {
  list1.iter().filter(|x| !list2.iter().any(|y| y == *x)).map(|&x| x).collect()
}

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
  let contacts = get_full_contacts(&map, 0);
  let mut left_contacts: Vec<i64> = (0..2000).collect();
  let mut known_contacts: Vec<i64> = Vec::new();
  let mut group = Vec::new();
  while !left_contacts.is_empty() {
    let known_contact = get_full_contacts(&map, *left_contacts.first().unwrap());
    group.push(known_contact.clone());
    known_contacts.append(&mut known_contact.clone());
    left_contacts = without(left_contacts, known_contact);
    println!("{} left, {} known, {} groups", left_contacts.len(), known_contacts.len(), group.len());
  };
  println!("Part 1: {}", contacts.len());
  println!("Part 2: {}", group.len());
}