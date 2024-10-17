extern crate regex;
#[macro_use] extern crate lazy_static;

use std::ops::Range;
use std::{collections::HashSet, convert::TryInto};
use advent_of_code::get_str_array_from_file;
use regex::Regex;

type Point = (isize, isize);

struct Report {
    sensor: (isize, isize),
    beacon: (isize, isize),
}

impl Report {
    fn get_distance(&self) -> usize {
        ((self.sensor.0 - self.beacon.0).abs() + (self.sensor.1 - self.beacon.1).abs()).try_into().unwrap()
    }

    fn get_x_range(&self, y: isize) -> Range<isize> {
        let dy = (self.sensor.1 - y).abs();
        let distance = self.get_distance() as isize;
        if distance < dy {
            Range::default()
        } else {
            let dx = distance - dy;
            (self.sensor.0 - dx..self.sensor.0 + dx + 1)
        }
    }
}

#[test]
fn test_get_x_range() {
    let report = Report {
        beacon: (2, 10),
        sensor: (8, 7),
    };
    assert_eq!(report.get_x_range(10), Range::from(2..15));
}

fn parse(input: &str) -> Report {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"Sensor\sat\sx=([-\d]+),\sy=([-\d]+):\sclosest\sbeacon\sis\sat\sx=([-\d]+),\sy=([-\d]+)").unwrap();
    }
    let caps = RE.captures(input).unwrap();
    let sensor = (caps[1].parse().unwrap(), caps[2].parse().unwrap());
    let beacon = (caps[3].parse().unwrap(), caps[4].parse().unwrap());
    Report { sensor, beacon }
}

#[test]
fn test_parse() {
    let input = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15";
    let report = parse(input);
    assert_eq!(report.sensor, (2, 18));
    assert_eq!(report.beacon, (-2, 15));
}

fn find_clear_points(
    report: &Report,
    y: isize,
) -> Vec<Point> {
    let Report { sensor, beacon } = report;
    let dy = (sensor.1 - y).abs();
    let distance = (sensor.1 - beacon.1).abs() + (sensor.0 - beacon.0).abs();

    if distance < dy {
        vec![]
    } else {
        let dx = distance - dy;
        (sensor.0 - dx..=sensor.0 + dx)
            .map(|x| (x, y))
            .collect()
    }
}

#[test]
fn test_find_clear_points() {
    let report = Report { sensor: (8,7), beacon: (2, 10) };
    let points = find_clear_points(&report, 10);
    assert_eq!(points, (2..=14).map(|x| (x, 10)).collect::<Vec<_>>());
}

fn find_all_clear_points(reports: &Vec<Report>, y: isize) -> HashSet<Point> {
    let mut points: HashSet<Point> = reports.iter()
        .flat_map(|report| {
            find_clear_points(report, y)
        })
        .collect();
    for Report { beacon, .. } in reports {
        points.remove(beacon);
    }
    points
}

fn main() {
    let data = get_str_array_from_file(&vec!["aoc2022", "data", "15.txt"]);
    let reports: Vec<Report> = data.iter().map(|x| parse(x)).collect();
    let clear_points = find_all_clear_points(&reports, 2_000_000);

    println!("Part 1: {}", clear_points.len());
    
}
