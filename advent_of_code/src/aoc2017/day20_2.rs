extern crate regex;

use std::collections::HashMap;
use regex::Regex;
use super::super::get_str_array_from_file;


#[derive(Debug)]
enum Unit {
  DISTANCE,
  VELOCITY,
  ACCELERATION,
}

fn get_re(t: Unit) -> Regex {
  lazy_static! {
    static ref RE_DISTANCE: Regex = Regex::new(r"p=<([-]{0,1}\d+),([-]{0,1}\d+),([-]{0,1}\d+)>").unwrap();
    static ref RE_VELOCITY: Regex = Regex::new(r"v=<([-]{0,1}\d+),([-]{0,1}\d+),([-]{0,1}\d+)>").unwrap();
    static ref RE_ACCELERATION: Regex = Regex::new(r"a=<([-]{0,1}\d+),([-]{0,1}\d+),([-]{0,1}\d+)>").unwrap();
  }
  match t {
      Unit::DISTANCE => RE_DISTANCE.clone(),
      Unit::VELOCITY => RE_VELOCITY.clone(),
      _ => RE_ACCELERATION.clone(),
  }
}

fn get_numbers(text: &str, t: Unit) -> [i32;3] {
  let re = get_re(t);
  let cap = &re.captures(text).unwrap();
  let x = cap[1].parse::<i32>().unwrap();
  let y = cap[2].parse::<i32>().unwrap();
  let z = cap[3].parse::<i32>().unwrap();
  [x, y, z]
}

fn get_single_collision_times(
  x: &[f64],
  y: &[f64]
) -> Vec<i32> {
  let mut result: Vec<i32> = vec!{};
  let a = x[0] - y[0];
  let b = x[1] - y[1];
  let c = x[2] - y[2];
  if a == 0.0 {
    let x = (-c as f64) / (b as f64);
    return if x.fract() == 0.0 && x > 0.0 {
      result.push(x as i32);
      result
    } else if c == 0.0 {
      result.push(-1);
      result
    } else {
      result
    }
  };
  let delta = b * b - 4.0 * a * c;
  if delta >= 0.0 {
    let root = delta.sqrt();
    let x1 = (- b - root) / 2.0 / a;
    let x2 = (- b + root) / 2.0 / a;
    if x1.fract() == 0.0 && x1 > 0.0 {
      result.push(x1 as i32);
    };
    if x2.fract() == 0.0 && x2 > 0.0 {
      result.push(x2 as i32);
    };
    result
  } else {
    result
  }
}

fn equal(a: i32, b: i32) -> bool {
  a == b || a.is_negative() || b.is_negative()
}

fn get_collision_times(
  x: &[f64; 9],
  y: &[f64; 9]
) -> Vec<i32> {
  let a = get_single_collision_times(&x[0..3], &y[0..3]);
  let b = get_single_collision_times(&x[3..6], &y[3..6]);
  let c = get_single_collision_times(&x[6..9], &y[6..9]);
  // println!("{:?}, {:?}, {:?}", a, b, c);
  let mut result: Vec<i32> = vec!{};
  for &i in a.iter() {
    for &j in b.iter() {
      for &k in c.iter() {
        if equal(i, j) && equal(j, k) && equal(i, k) {
          result.push(i.max(j.max(k)));
        }
      }
    }
  };
  result
}

fn sva_to_abc([s, v, a]: [i32;3]) -> [f64;3] {
  let _a = (a as f64) / 2.0;
  let b = ((2 * v + a) as f64) / 2.0;
  let c = s as f64;
  [_a, b, c]
}

fn get_particle(text: &str) -> [f64;9] {
  let distances = get_numbers(text, Unit::DISTANCE);
  let velocities = get_numbers(text, Unit::VELOCITY);
  let accelerations = get_numbers(text, Unit::ACCELERATION);
  let svas: Vec<[i32;3]> = (0..3).map(|x| {
    [distances[x], velocities[x], accelerations[x]]
  }).collect();
  let svas = [
    sva_to_abc(svas[0]),
    sva_to_abc(svas[1]),
    sva_to_abc(svas[2]),
  ].concat();
  let mut array = [0.0; 9];
  let svas = &svas[..array.len()]; // panics if not enough data
  array.copy_from_slice(svas); 
  array
}

pub fn resolve() {
  let array = get_str_array_from_file(&vec!{"aoc2017", "day20_data.txt"});
  let particles: Vec<[f64;9]> = array.iter().map(|x| get_particle(x)).collect();
  let mut collision_map: HashMap<i32, Vec<[usize;2]>> = HashMap::new();
  let len = particles.len();
  for i in 0..len-1 {
    for j in (i+1)..len {
      let collisions = get_collision_times(&particles[i], &particles[j]);
      if collisions.len() > 0 {
        for &c in collisions.iter() {
          let entry = collision_map.entry(c).or_insert(vec!{});
          entry.push([i, j]);
        }
      }
    }
  };
  let mut times = collision_map.keys().map(|x| *x).collect::<Vec<i32>>();
  times.sort();
  let mut particle_checklist = vec!{true; 1000};
  for time in times.iter() {
    let mut ready_to_remove: Vec<usize> = vec!{};
    for [a, b] in collision_map.get(time).unwrap().iter() {
      if particle_checklist[*a] && particle_checklist[*b] {
        ready_to_remove.push(*a);
        ready_to_remove.push(*b);
      }
    };
    for &i in ready_to_remove.iter() {
      particle_checklist[i] = false;
    };
  };
  particle_checklist.retain(|&x| x);
  println!("Part 2: {}", particle_checklist.len());
}