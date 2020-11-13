#[macro_use] extern crate lazy_static;
extern crate regex;
extern crate chrono;

use core::cmp::*;
use std::collections::HashMap;
use std::convert::TryInto;
use regex::{ Regex };
use chrono::{ NaiveDateTime, Timelike, Duration };
use advent_of_code::get_str_array_from_file;

#[derive(Debug, Eq)]
enum Event {
  Shift(u32),
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
      event = Event::Shift(cap.name("id").unwrap().as_str().parse::<u32>().unwrap());
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

type SleepCountMap = HashMap<u32, [usize; 60]>;

fn count_sleep(records: &Vec<Record>) -> SleepCountMap {
  let mut map: SleepCountMap = HashMap::new();
  let mut current_guard: u32 = 0;
  let mut fall_asleep_minute: u32 = 0;
  records.iter().for_each(|record| {
    match record.event {
      Event::Shift(guard) => {
        if fall_asleep_minute > 0 {
          let entry = map.entry(current_guard).or_insert([0; 60]);
          (fall_asleep_minute..60).for_each(|x| {
            let index: usize = x.try_into().unwrap();
            (*entry)[index] += 1;
          });
        }
        current_guard = guard;
      },
      Event::FallsAsleep => {
        fall_asleep_minute = record.datetime.minute();
      }
      Event::WakeUp => {
        let entry = map.entry(current_guard).or_insert([0; 60]);
        let wakeup_minute = record.datetime.minute();
        (fall_asleep_minute..wakeup_minute).for_each(|x| {
          let index: usize = x.try_into().unwrap();
          (*entry)[index] += 1;
        });
        fall_asleep_minute = 0;
      }
    }
  });
  map
}

fn main() {
  let array = get_str_array_from_file(&vec!{"aoc2018", "data", "4.txt"});
  let mut records: Vec<Record> = array.iter().map(|x| get_record(x)).collect();
  records.sort();

  let map = count_sleep(&records);

  let most_sleep_guard = map.keys().map(|k| {
    let values = map.get(k).unwrap();
    let sum: usize = values.iter().sum();
    (k, sum, values)
  }).max_by_key(|x| x.1).unwrap();
  let most_sleep_minute: u32 = most_sleep_guard.2.iter().enumerate().max_by_key(|x| x.1).unwrap().0.try_into().unwrap();
  println!("Part 1: {}", most_sleep_guard.0 * most_sleep_minute);

  let most_frequently_asleep_guard = map.keys().map(|k| {
    let values = map.get(k).unwrap();
    let max_pair = values.iter().enumerate().max_by_key(|(_, x)| x.clone()).unwrap();
    (k, max_pair)
  }).max_by_key(|x| x.1.1).unwrap();
  let most_frequently_asleep_minute: u32 = most_frequently_asleep_guard.1.0.try_into().unwrap();
  println!("Part 2: {}", most_frequently_asleep_guard.0 * most_frequently_asleep_minute);
}
