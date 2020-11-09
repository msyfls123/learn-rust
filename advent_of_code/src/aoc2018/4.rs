#[macro_use] extern crate lazy_static;
extern crate regex;

use regex::{ Regex };
use core::cmp::*;
use advent_of_code::get_str_array_from_file;

#[derive(Debug, Eq)]
enum Event {
  Shift(i32),
  FallsAsleep,
  WakeUp,
}

#[derive(Debug, Eq)]
struct Record {
  month: i32,
  day: i32,
  minute: i32,
  event: Event,
}

impl Ord for Record {
  fn cmp(&self, other: &Self) -> Ordering {
    (self.month, self.day, self.minute).cmp(&(other.month, other.day, other.minute))
  }
}

impl PartialOrd for Record {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
      Some(self.cmp(other))
  }
}

impl PartialEq for Event {
  fn eq(&self, other: &Self) -> bool {
      match self {
        Event::FallsAsleep => *other == Event::FallsAsleep,
        Event::WakeUp => *other == Event::WakeUp,
        Event::Shift(id) => {
          match other {
            Event::Shift(id2) => id == id2,
            _ => false,
          }
        }
      }
  }
}

impl PartialEq for Record {
  fn eq(&self, other: &Self) -> bool {
      (self.month, self.day, self.minute, &self.event) == (other.month, other.day, other.minute, &other.event)
  }
}

fn get_record(text: &str) -> Record {
  lazy_static! {
    static ref RE: Regex = Regex::new(r"\[(?P<year>\d{4})-(?P<month>\d{2})\-(?P<day>\d{2})\s(?P<hour>\d{2}):(?P<minute>\d{2})\]\s(?P<event>.*)$").unwrap();
    static ref RE_EVENT: Regex = Regex::new(r"Guard\s#(?P<id>\d+)").unwrap();
  }
  let captured = RE.captures(text).unwrap();
  let month = captured.name("month").unwrap().as_str().parse::<i32>().unwrap();
  let mut day = captured.name("day").unwrap().as_str().parse::<i32>().unwrap();
  let hour = captured.name("hour").unwrap().as_str().parse::<i32>().unwrap();
  let mut minute = captured.name("minute").unwrap().as_str().parse::<i32>().unwrap();
  let event_text = captured.name("event").unwrap().as_str();

  // TODO: should use chrono to parse date and modify it
  // https://rust-lang-nursery.github.io/rust-cookbook/datetime/parse.html
  if hour > 0 {
    day += 1;
    minute = 0;
  }

  let event: Event;
  match RE_EVENT.captures(event_text) {
    Some(cap) => {
      event = Event::Shift(cap.name("id").unwrap().as_str().parse::<i32>().unwrap());
    },
    None => {
      event = if event_text == "falls asleep" {
        Event::FallsAsleep
      } else {
        Event::WakeUp
      };
    }
  };

  Record {
    month,
    day,
    minute,
    event,
  }
}

fn main() {
  let array = get_str_array_from_file(&vec!{"aoc2018", "data", "4.txt"});
  let mut data: Vec<Record> = array.iter().map(|x| get_record(x)).collect();
  data.sort();
  println!("{:?}", data);
}
