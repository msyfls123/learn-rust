use std::{ops::{Add, Sub}, fmt::Display, collections::{HashSet, hash_map::RandomState, HashMap}};
use std::iter::FromIterator;

use advent_of_code::{split_lines, get_group_str_from_file, get_str_array_from_file};
use itertools::Itertools;

type PointSet = HashSet<Beacon, RandomState>;
type DistanceMap = HashMap<(Beacon, usize), PointSet>;

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

fn map_directions(point: &Beacon) -> Vec<Beacon> {
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

fn calc_distance_map(start_points: &Vec<Beacon>, end_points: &Vec<Beacon>) -> DistanceMap {
    let mut map: DistanceMap = HashMap::new();
    start_points.iter().enumerate().for_each(|(i, p)| {
        end_points.iter().for_each(|next_p| {
            map_directions(p).iter().map(|cp| {
                next_p.to_owned() - cp.to_owned()
            }).enumerate().for_each(|(index, dist)| {
                let entry = map.entry((dist, index)).or_insert(HashSet::new());
                entry.insert(next_p.to_owned());
            })
        })
    });
    map
}

#[test]
fn test_normalize_beacons() {
    let a_list = get_lists(&split_lines(r#"404,-588,-901
    528,-643,409
    -838,591,734
    390,-675,-793
    -537,-823,-458
    -485,-357,347
    -345,-311,381
    -661,-816,-575
    -876,649,763
    -618,-824,-621
    553,345,-567
    474,580,667
    -447,-329,318
    -584,868,-557
    544,-627,-890
    564,392,-477
    455,729,728
    -892,524,684
    -689,845,-530
    423,-701,434
    7,-33,-71
    630,319,-379
    443,580,662
    -789,900,-551
    459,-707,401"#));
    let b_list = get_lists(&split_lines(r#"686,422,578
    605,423,415
    515,917,-361
    -336,658,858
    95,138,22
    -476,619,847
    -340,-569,-846
    567,-361,727
    -460,603,-452
    669,-402,600
    729,430,532
    -500,-761,534
    -322,571,750
    -466,-666,-811
    -429,-592,574
    -355,545,-477
    703,-491,-529
    -328,-685,520
    413,935,-424
    -391,539,-444
    586,-435,557
    -364,-763,-893
    807,-499,-711
    755,-354,-619
    553,889,-390"#));
    let map = calc_distance_map(&b_list, &a_list);
    let max = map.iter().max_by(|x, y| x.1.len().cmp(&y.1.len()));
    assert_eq!(max.map(|x| x.1.len()), Some(12));
    let normalized_beacons = get_normalize_beacons(&b_list, &a_list);
    for point in normalized_beacons {
        println!("{}", point);
    }
}

fn get_normalize_beacons(start_points: &Vec<Beacon>, end_points: &Vec<Beacon>) -> Vec<Beacon> {
    let map = calc_distance_map(start_points, end_points);
    let (distances, size) = map.iter()
        .max_by(|x, y| x.1.len().cmp(&y.1.len()))
        .unwrap();
    // TODO split this func
    // 1. calc only 12 overlapping beacons then normalize
    // 2. normalize with a -> b -> c -> 0
    println!("distances {} : rotate {}: count {}", distances.0, distances.1, size.len());
    start_points.iter().map(|p| map_directions(p)[distances.1].to_owned() + distances.0.to_owned()).collect()
}

fn get_full_list_beacons(list: &Vec<Vec<Beacon>>) -> Vec<Beacon> {
    let base_beacons = list[0].to_owned();
    let mut full_list = vec!{};
    for next_beacons in list[1..].iter() {
        let normalized_beacons = get_normalize_beacons(next_beacons, &base_beacons);
        for beacon in normalized_beacons {
            if !full_list.contains(&beacon) {
                full_list.push(beacon);
            }
        }
    }
    full_list
}

fn main() {
    let data = get_group_str_from_file(&vec!{"aoc2021", "data", "19.txt"});
    let res = get_str_array_from_file(&vec!{"aoc2021", "data", "19.res.txt"});
    let res_list = get_lists(&res);
    let beacons_list: Vec<Vec<Beacon>> = data.iter().map(|group| {
        get_lists(&group[1..].to_vec())
    }).collect();
    let full_list_beacons = get_full_list_beacons(&beacons_list);
    for (index, beacons) in beacons_list.iter().enumerate() {
        println!("{}: {}", index, beacons.len());
    }
    for point in &full_list_beacons {
        if !res_list.contains(point) {
            println!("{}", point);
        }
    }
    println!("{:?}", full_list_beacons.len());
}
