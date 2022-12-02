use std::{ops::{Add, Sub}, fmt::Display, collections::{HashSet, hash_map::RandomState, HashMap}};

use advent_of_code::{split_lines, get_group_str_from_file, get_str_array_from_file, algorithm::get_shortest_path};
use chrono::{Local};
use itertools::Itertools;

type PointSet = HashSet<Beacon, RandomState>;
type TransformMap = HashMap<Transform, PointSet>;

type TransformGraph = HashMap<(usize, usize), Transform>;

#[derive(Debug, Clone, Eq, Hash)]
struct Transform {
    orientation_index: usize,
    relative_position: Beacon,
}

impl PartialEq for Transform {
    fn eq(&self, other: &Self) -> bool {
        self.orientation_index == other.orientation_index
        && self.relative_position == other.relative_position
    }
}

#[derive(Debug, Clone, Eq, Hash)]
struct Beacon {
    x: isize,
    y: isize,
    z: isize,
}

impl From<String> for Beacon {
    fn from(text: String) -> Self {
        let (x, y, z) = text.split(",").collect_tuple().unwrap();
        Beacon {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
            z: z.parse().unwrap(),
        }
    }
}

impl Add for Beacon {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Beacon {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl PartialEq for Beacon {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x
        && self.y == other.y
        && self.z == other.z
    }
}

impl Display for Beacon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{},{}", self.x, self.y, self.z)
    }
}

fn map_rotate_direction(point: &Beacon) -> Vec<Beacon> {
    let Beacon { x, y, z } = point.to_owned();
    vec!{
        Beacon {x, y, z},
        Beacon {x: -y, y: x, z: z},
        Beacon {x: -x, y: -y, z: z},
        Beacon {x: y, y: -x, z: z},
        Beacon {x: y, y: x, z: -z},
        Beacon {x: -x, y: y, z: -z},
        Beacon {x: -y, y: -x, z: -z},
        Beacon {x: x, y: -y, z: -z},
    }
}

fn map_orientations(point: &Beacon) -> Vec<Beacon> {
    let Beacon { x, y, z } = point.to_owned();
    vec!{
        Beacon { x, y, z },
        Beacon { x: y, y: z, z: x },
        Beacon { x: z, y: x, z: y },
    }.iter().flat_map(|p| map_rotate_direction(p)).collect()
}

fn get_lists<T>(list: &Vec<T>) -> Vec<Beacon>
where
  T: Into<String> + Clone,
{
    list.iter().map(|text| {
        let str = text.clone().into();
        str.into()
    }).collect()
}

fn find_possible_transform(start_points: &Vec<Beacon>, end_points: &Vec<Beacon>) -> Option<Transform> {
    let mut map: TransformMap = HashMap::new();
    start_points.iter().enumerate().for_each(|(i, p)| {
        end_points.iter().for_each(|next_p| {
            map_orientations(p).iter().map(|cp| {
                next_p.to_owned() - cp.to_owned()
            }).enumerate().for_each(|(index, dist)| {
                let entry = map.entry(Transform {
                    orientation_index: index,
                    relative_position: dist,
                }).or_insert(HashSet::new());
                entry.insert(next_p.to_owned());
            })
        })
    });
    map.iter().find_map(|(transform, beacons)| {
        if beacons.len() >= 12 {
            Some(transform.to_owned())
        } else {
            None
        }
    } )
}

#[test]
fn test_normalize_beacons() {
    let data = get_group_str_from_file(&vec!{"aoc2021", "data", "19.test.txt"});
    let res = get_str_array_from_file(&vec!{"aoc2021", "data", "19.res.txt"});
    let res_list = get_lists(&res);
    let beacons_list: Vec<Vec<Beacon>> = data.iter().map(|group| {
        get_lists(&group[1..].to_vec())
    }).collect();
    let transform_graph = calc_full_transform_graph(&beacons_list);
    let full_list_beacons = get_full_list_beacons(&beacons_list, &transform_graph);
    for point in &full_list_beacons {
        if !res_list.contains(point) {
            println!("{}", point);
        }
    }
    assert_eq!(full_list_beacons.len(), 79);
}

fn calc_full_transform_graph(list: &Vec<Vec<Beacon>>) -> TransformGraph {
    let mut map = HashMap::new();
    for (i, list_a) in list.iter().enumerate() {
        for (j, list_b) in list.iter().enumerate() {
            if i != j {
                match find_possible_transform(list_a, list_b) {
                    Some(transform) => {
                        map.insert((i, j), transform);
                    },
                    _ => {},
                }
            }
        }
    }
    map
}

fn get_normalize_beacons(start_points: &Vec<Beacon>, transform: &Transform) -> Vec<Beacon> {
    let Transform { orientation_index, relative_position } = transform;
    start_points.iter().map(|p| map_orientations(p)[*orientation_index].to_owned() + relative_position.to_owned()).collect()
}

fn get_full_list_beacons(list: &Vec<Vec<Beacon>>, transform_graph: &TransformGraph) -> HashSet<Beacon> {
    
    let edges: Vec<(usize, usize)> = transform_graph.keys().map(|x| x.to_owned()).collect();
    list.iter().enumerate().flat_map(|(index, beacons)| {
        let shorest_path = get_shortest_path(index, 0, &edges).expect(&format!("not found valid path: {} to 0", index));
        shorest_path.iter().fold(beacons.to_owned(), |curr, path| {
            get_normalize_beacons(&curr, transform_graph.get(path).unwrap())
        })
    }).collect()
    
}

fn calc_distances_of_scanners(list: &Vec<Vec<Beacon>>, transform_graph: &TransformGraph) -> PointSet {
    let edges: Vec<(usize, usize)> = transform_graph.keys().map(|x| x.to_owned()).collect();
    let length = list.len();
    (0..length).map(|i| {
        let base_beacon = Beacon { x: 0, y: 0, z: 0 };
        if i == 0 {
            base_beacon
        } else {
            let shorest_path = get_shortest_path(i, 0, &edges).expect(&format!("not found valid path: {} to 0", i));
            shorest_path.iter().fold(base_beacon, |curr, path| {
                get_normalize_beacons(&vec!{curr}, transform_graph.get(path).unwrap())[0].to_owned()
            })
        }
    }).collect()
}

fn find_largest_distance(points: &PointSet) -> isize {
    let mut max = 0;
    for (i, point) in points.iter().enumerate() {
        for (j, point2) in points.iter().enumerate() {
            if i != j {
                let diff = point.to_owned() - point2.to_owned();
                let distance = isize::abs(diff.x) + isize::abs(diff.y) + isize::abs(diff.z);
                if distance > max {
                    max = distance
                }
            }
        }
    }
    max
}

#[test]
fn test_find_largest_distance() {
    let data = get_group_str_from_file(&vec!{"aoc2021", "data", "19.test.txt"});
    let beacons_list: Vec<Vec<Beacon>> = data.iter().map(|group| {
        get_lists(&group[1..].to_vec())
    }).collect();
    let transform_graph = calc_full_transform_graph(&beacons_list);
    let scanners = calc_distances_of_scanners(&beacons_list, &transform_graph);
    let largest_distance = find_largest_distance(&scanners);
    assert_eq!(largest_distance, 3621);
}

fn main() {
    let now = Local::now();
    let data = get_group_str_from_file(&vec!{"aoc2021", "data", "19.txt"});
    let beacons_list: Vec<Vec<Beacon>> = data.iter().map(|group| {
        get_lists(&group[1..].to_vec())
    }).collect();
    let transform_graph = calc_full_transform_graph(&beacons_list);
    let duration = Local::now() - now;
    println!("Transform graph cost: {:?}", duration.to_std().unwrap());
    let full_list_beacons = get_full_list_beacons(&beacons_list, &transform_graph);
    println!("Part 1: {}", full_list_beacons.len());
    let scanners = calc_distances_of_scanners(&beacons_list, &transform_graph);
    let largest_distance = find_largest_distance(&scanners);
    println!("Part 2: {}", largest_distance);
}
