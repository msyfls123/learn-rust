use advent_of_code::get_group_str_from_file;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

type Range = (usize, usize);
// from field to position
type Positions = HashMap<usize, usize>;

#[derive(Debug, Clone)]
struct Field {
  name: String,
  ranges: Vec<Range>,
}

fn get_field(text: &str) -> Field {
  lazy_static! {
    static ref RE_RANGE: Regex = Regex::new(r"(\d+)-(\d+)").unwrap();
    static ref RE_NAME: Regex = Regex::new(r"(?P<name>.*):\s").unwrap();
  }
  let name = RE_NAME.captures(text).unwrap().name("name").unwrap().as_str().to_string();
  let ranges = RE_RANGE.captures_iter(text).map(|cap| {
    (cap[1].parse::<usize>().unwrap(), cap[2].parse::<usize>().unwrap())
  }).collect();
  Field {
    name,
    ranges,
  }
}

fn get_valid_field_bit(value: usize, fields: &Vec<Field>) -> usize {
  fields.iter().enumerate().fold(0, |acc, (index, field)| {
    let is_in_range = field.ranges.iter().any(|&(min, max)| {
      value >= min && value <= max
    });
    if is_in_range {
      return acc | 1 << index;
    }
    acc
  })
}

fn find_positions(
  positions: &mut Positions,
  mut indexes: Vec<Vec<usize>>
) {
  let only_one_indexes: Vec<(usize, usize)> = indexes
    .iter().enumerate()
    .filter_map(|(index, fields)| {
      if fields.len() == 1 {
        Some((index, fields[0]))
      } else { None }
    }).collect();
  if only_one_indexes.len() == 0 {
    return;
  }
  only_one_indexes.iter().for_each(|&(index, field_index)| {
    positions.insert(field_index, index);
    indexes = indexes.iter().map(|fields| {
      let mut c_fields = fields.clone();
      c_fields.retain(|&f| f != field_index);
      c_fields.to_owned()
    }).collect();
  });
  find_positions(positions, indexes);
}

fn main() {
  let notes = get_group_str_from_file(&vec!{"aoc2020", "data", "16.txt"});
  let nearby_ticket_values: Vec<Vec<usize>> = notes[2].iter().skip(1).map(|line| {
    line.split(",").map(|x| x.parse().unwrap()).collect::<Vec<usize>>()
  }).collect();
  let my_tickets: Vec<usize> = notes[1][1]
    .split(",").map(|x| x.parse().unwrap()).collect();
  let fields: Vec<Field> = notes[0].iter().map(|line| get_field(&line)).collect();
  let field_count = fields.len();
  let ranges: Vec<Range> = fields.iter().flat_map(|field| field.ranges.to_owned()).collect();
  let ticket_scanning_error_rate: usize = nearby_ticket_values.iter().flatten().filter(|&value| {
    !ranges.iter().any(|(min, max)| {
      value >= min && value <= max
    })
  }).sum();
  println!("Part 1: {}", ticket_scanning_error_rate);

  let valid_tickets: Vec<Vec<usize>> = nearby_ticket_values.iter().filter(|values| {
    values.iter().all(|value| {
      ranges.iter().any(|(min, max)| {
        value >= min && value <= max
      })
    })
  }).map(|v| v.to_owned()).collect();

  let field_indexes: Vec<Vec<usize>> = (0..field_count).map(|index| {
    let initial_bit = (1 << field_count) - 1;
    // if field at 0, 2, 3 is valid, we will got 13 for this value, and then merge all values' fields
    let bits = valid_tickets.iter().map(|ticket| ticket[index]).fold(initial_bit, |acc, value| {
      let bit = get_valid_field_bit(value, &fields);
      acc & bit
    });
    let str = format!("{:0>width$b}", bits, width = field_count);
    str.chars().enumerate().filter_map(|(index, bit)| {
      if bit == '1' {
        Some(field_count - 1 - index)
      } else {
        None
      }
    }).collect()
  }).collect();
  // println!("{:?}", field_indexes);

  let mut positions: Positions = HashMap::new();
  find_positions(&mut positions, field_indexes);
  // println!("{:?}", positions);

  let departure_values_mul = fields.iter().enumerate().filter_map(|(index, field)| {
    if field.name.starts_with("departure") {
      Some(index)
    } else {
      None
    }
  }).map(|index| my_tickets[*positions.get(&index).unwrap()]).fold(1, |acc, value| acc * value);
  println!("Part 2: {}", departure_values_mul);
}
