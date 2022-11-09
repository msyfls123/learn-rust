use std::{convert::TryInto, slice::Chunks};

use advent_of_code::get_str_from_file;
use itertools::Itertools;

#[derive(Debug)]
struct Literal {
  length: usize,
  version: usize,
  type_id: usize,
  bits: Vec<[char; 4]>
}

#[derive(Debug)]
struct Operator {
  length: usize,
  version: usize,
  type_id: usize,
  sub_packets: Vec<Packet>
}

#[derive(Debug)]
enum Packet {
  Literal(Literal),
  Operator(Operator),
}

fn get_length(p: &Packet) -> usize {
  match p {
    Packet::Literal(i) => i.length,
    Packet::Operator(i) => i.length,
  }
}

fn hex_to_bits(text: &str) -> String {
  text.chars().map(|c| format!("{:04b}", c.to_digit(16).unwrap())).join("")
}

#[test]
fn test_hex_to_bits() {
  assert_eq!(&hex_to_bits("8A004"), "10001010000000000100");
}

fn parse_chars_to_int<T>(chars: &T) -> usize
where T: IntoIterator<Item = char> + ?Sized + Clone, {
  usize::from_str_radix(&chars.clone().into_iter().collect::<String>(), 2).unwrap()
}

fn parse_packet(text: &str) -> Packet {
  let text: Vec<char> = text.chars().collect();
  let mut bits = text.chunks(3);
  match bits.next() {
    Some(version) => {
      match bits.next() {
        Some(&['1', '0', '0']) => {
          let rest_str: String = bits.flatten().collect();
          let l_bits = parse_literal_bits(&rest_str);
          let length = l_bits.len() * 5 + 6;
          // let length = if length % 4 == 0 {
          //   length
          // } else {
          //   length + 4 - (length % 4)
          // };
          Packet::Literal(Literal {
            length,
            version: parse_chars_to_int(&version.to_owned()),
            type_id: 4,
            bits: l_bits,
          })
        },
        Some(type_id) => {
          let rest_str: String = bits.flatten().collect();
          let (type_length, sub_packets) = parse_operator(&rest_str);
          let length = sub_packets.iter().map(|p| get_length(p)).sum::<usize>() + 6 + type_length;
          // let length = if length % 4 == 0 {
          //   length
          // } else {
          //   length + 4 - (length % 4)
          // };
          Packet::Operator(Operator {
            type_id: parse_chars_to_int(&type_id.to_owned()),
            length,
            sub_packets,
            version: parse_chars_to_int(&version.to_owned()),
          })
        },
        None => panic!("unknown type id")
      }
    },
    None => panic!("unknown version")
  }
}

fn parse_literal_bits(text: &str) -> Vec<[char; 4]> {
  let mut is_last = false;
  let bits = text.chars()
    .collect_vec()
    .chunks(5)
    .take_while(|x| {
      if is_last {
        false
      } else if x[0] == '0' {
        is_last = true;
        true
      } else {
        true
      }
    })
    .map(|chars| chars[1..].try_into().unwrap())
    .collect();
  bits
}

fn parse_operator(text: &str) -> (usize, Vec<Packet>) {
  let mut chars = text.chars();
  let mut packets: Vec<Packet> = vec!{};
  match chars.nth(0).unwrap() {
    '0' => {
      let total = parse_chars_to_int(&chars.clone().take(15));
      chars.nth(14);
      while packets.iter().map(|p| get_length(p)).sum::<usize>() < total {
        let packet = parse_packet(&chars.to_owned().collect::<String>());
        chars.nth(get_length(&packet) - 1);
        packets.push(packet);
      }
      (16, packets)
    },
    '1' => {
      let total = parse_chars_to_int(&chars.clone().take(11));
      
      chars.nth(10);
      while packets.len() < total {
        let packet = parse_packet(&chars.to_owned().collect::<String>());
        chars.nth(get_length(&packet) - 1);
        packets.push(packet);
      }
      (12, packets)
    },
    _ => panic!("unknown length type id")
  }
}

fn get_version_count_of_packet(p: &Packet) -> usize {
  match p {
    Packet::Literal(l) => l.version,
    Packet::Operator(o) => o.version + o.sub_packets.iter().map(|sp| {
      get_version_count_of_packet(sp)
    }).sum::<usize>()
  }
}

fn main() {
  let data = get_str_from_file(&vec!{"aoc2021", "data", "16.txt"});
  let text = hex_to_bits(&data);
  let packet = parse_packet(&text);
  println!("{}", get_version_count_of_packet(&packet));
}
