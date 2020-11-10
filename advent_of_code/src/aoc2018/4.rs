#[macro_use] extern crate lazy_static;
extern crate regex;
extern crate chrono;

use core::cmp::*;
use regex::{ Regex };
use chrono::{ NaiveDateTime, Timelike, Datelike, Duration };
use advent_of_code::get_str_array_from_file;

#[derive(Debug, Eq)]
enum Event {
  Shift(i32),
  FallsAsleep,
  WakeUp,
}

#[derive(Debug, Eq)]
struct Record {
  datetime: NaiveDateTime,
  event: Event,
}

impl Ord for Record {
  fn cmp(&self, other: &Self) -> Ordering {
    self.datetime.cmp(&other.datetime)
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
      self.datetime == other.datetime
  }
}

fn get_record(text: &str) -> Record {
  lazy_static! {
    static ref RE_DATETIME: Regex = Regex::new(r"\[(?P<datetime>\d{4}-\d{2}-\d{2}\s\d{2}:\d{2})\]\s(?P<event>.*)$").unwrap();
    static ref RE_EVENT: Regex = Regex::new(r"Guard\s#(?P<id>\d+)").unwrap();
  }
  let captured = RE_DATETIME.captures(text).unwrap();
  let datetime_text = captured.name("datetime").unwrap().as_str();
  let mut datetime = NaiveDateTime::parse_from_str(datetime_text, "%Y-%m-%d %H:%M").unwrap();
  let event_text = captured.name("event").unwrap().as_str();

  if datetime.hour() > 0 {
    datetime = datetime.with_hour(0).unwrap().with_minute(0).unwrap() + Duration::days(1);
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
    datetime,
    event,
  }
}

fn main() {
  let array = get_str_array_from_file(&vec!{"aoc2018", "data", "4.txt"});
  let mut data: Vec<Record> = array.iter().map(|x| get_record(x)).collect();
  data.sort();
  let found: Vec<&Record> = data.iter().filter(|record| {
    record.datetime.month() == 9 && record.datetime.day() == 1
  }).collect();
  println!("{:?}", found);
}
