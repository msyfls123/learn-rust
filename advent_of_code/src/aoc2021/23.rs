#![feature(generic_const_exprs)]
use std::{convert::TryInto, fmt::Display, collections::{HashMap, BinaryHeap}, hash::Hash};
use chrono::{Local};
use std::cmp::Reverse;

/**
 * diagram
 * 
 * #############
 * #...........#
 * ###B#C#B#D###
 *   #A#D#C#A#
 *   #########
 * 
 * #############
 * #01.2.3.4.56#
 * ###1#1#1#1###
 *   #0#0#0#0#
 *   #########
 */

type Hallway = [usize; 7];

trait StateTrait {
    fn possible_edge_states(&self) -> Vec<(Self, usize)> where Self: Sized;
    fn energy(&self, room_info: (usize, usize), h_idx: usize, room_to_hallway: bool) -> usize;
    fn room_to_hallway(&self, room_info: (usize, usize), h_idx: usize) -> Option<Self> where Self: Sized;
    fn hallway_to_room(&self, h_idx: usize, room_info: (usize, usize)) -> Option<Self> where Self: Sized;
    fn new_hallway(&self, room_info: (usize, usize), h_idx: usize, target: usize, is_leaving: bool) -> Hallway;
    fn new_rooms(&self, room_info: (usize, usize), h_idx: usize, target: usize, is_leaving: bool) -> Vec<Vec<usize>>;
    fn room_clear(&self, r_idx: usize, a_pos: usize, is_leaving: bool) -> bool;
    fn hallway_clear(&self, r_idx: usize, h_idx: usize, is_leaving: bool) -> bool;
    fn hv(&self, h_idx: usize) -> char;
    fn rv(&self, r_idx: usize, a_pos: usize) -> char;
}

trait HasHallwayAndRooms {
    const SIZE: usize;
    fn get_hallway(&self) -> Hallway;
    fn get_rooms(&self) -> [[usize; Self::SIZE]; 4];
    fn new(hallway: Hallway, rooms: &Vec<Vec<usize>>) -> Self;
}

#[derive(Debug, PartialEq, Eq, Hash, Default, Clone, Copy, PartialOrd, Ord)]
struct StateFor2 {
    hallway: Hallway,
    side_rooms: [[usize; 2]; 4],
}

impl HasHallwayAndRooms for StateFor2 {
    const SIZE: usize = 2;
    fn get_rooms(&self) -> [[usize; Self::SIZE]; 4] {
        self.side_rooms
    }
    fn get_hallway(&self) -> Hallway {
        self.hallway
    }
    fn new(hallway: Hallway, rooms: &Vec<Vec<usize>>) -> Self {
        Self {
            hallway,
            side_rooms: rooms.iter().map(|v| v.clone().try_into().expect("side_room collect fail"))
                .collect::<Vec<[usize; Self::SIZE]>>().try_into().expect("side_room collect fail"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Default, Clone, Copy, PartialOrd, Ord)]
struct StateFor4 {
    hallway: Hallway,
    side_rooms: [[usize; 4]; 4],
}

impl HasHallwayAndRooms for StateFor4 {
    const SIZE: usize = 4;
    fn get_rooms(&self) -> [[usize; Self::SIZE]; 4] {
        self.side_rooms
    }
    fn get_hallway(&self) -> Hallway {
        self.hallway
    }
    fn new(hallway: Hallway, rooms: &Vec<Vec<usize>>) -> Self {
        Self {
            hallway,
            side_rooms: rooms.iter().map(|v| v.clone().try_into().expect("side_room collect fail"))
                .collect::<Vec<[usize; Self::SIZE]>>().try_into().expect("side_room collect fail"),
        }
    }
}

impl Display for StateFor2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#############\n")?;
        write!(
            f,
            "#{}{}.{}.{}.{}.{}{}#\n",
            self.hv(0),
            self.hv(1),
            self.hv(2),
            self.hv(3),
            self.hv(4),
            self.hv(5),
            self.hv(6),
        )?;
        write!(
            f,
            "###{}#{}#{}#{}###\n",
            self.rv(0, 1),
            self.rv(1, 1),
            self.rv(2, 1),
            self.rv(3, 1),
        )?;
        write!(
            f,
            "  #{}#{}#{}#{}#\n",
            self.rv(0, 0),
            self.rv(1, 0),
            self.rv(2, 0),
            self.rv(3, 0),
        )?;
        write!(
            f,
            "  #########\n",
        )
    }
}

impl Display for StateFor4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#############\n")?;
        write!(
            f,
            "#{}{}.{}.{}.{}.{}{}#\n",
            self.hv(0),
            self.hv(1),
            self.hv(2),
            self.hv(3),
            self.hv(4),
            self.hv(5),
            self.hv(6),
        )?;
        for i in (1..=4) {
            write!(
                f,
                "###{}#{}#{}#{}###\n",
                self.rv(0, 4 - i),
                self.rv(1, 4 - i),
                self.rv(2, 4 - i),
                self.rv(3, 4 - i),
            )?;
        }
        write!(
            f,
            "  #########\n",
        )
    }
}

impl <T: HasHallwayAndRooms + Clone> StateTrait for T where [(); Self::SIZE]: {
    fn possible_edge_states(&self) -> Vec<(Self, usize)> {
        let mut states = vec!{};
        self.get_hallway().iter().enumerate().for_each(|(h_idx, &h_value)| {
            // empty hallway
            if h_value == 0 { return; }
            let r_idx = h_value - 1;
            self.get_rooms()[r_idx].iter().enumerate().any(|(a_pos, &r_value)| {
                if r_value != 0 { return false; }
                match self.hallway_to_room(h_idx, (r_idx, a_pos)) {
                    Some(state) => {
                        states.push((state, self.energy((r_idx, a_pos), h_idx, false)));
                    },
                    _ => {},
                }
                true
            });
        });
        self.get_rooms().iter().enumerate().for_each(|(r_idx, room)| {
            let mut room_iter = room.iter().enumerate().rev();
            room_iter.any(|(a_pos, &r_value)| {
                if r_value == 0 { return false; }
                self.get_hallway().iter().enumerate().for_each(|(h_idx, &h_value)| {
                    if h_value != 0 { return };
                    match self.room_to_hallway((r_idx, a_pos), h_idx) {
                        Some(|state) => {
                            states.push((state, self.energy((r_idx, a_pos), h_idx, true)));
                        },
                        _ => {},
                    }
                });
                true
            });
        });
        states
    }

    fn energy(&self, (r_idx, a_pos): (usize, usize), h_idx: usize, room_to_hallway: bool) -> usize {
        let target = if room_to_hallway {
            self.get_rooms()[r_idx][a_pos]
        } else {
            self.get_hallway()[h_idx]
        };
        let distance = match h_idx {
            0 => (r_idx * 2 + 2) + (Self::SIZE - a_pos),
            6 => (8 - r_idx * 2) + (Self::SIZE - a_pos),
            num => ((num as isize) * 2 - (r_idx as isize) * 2 - 3).abs() as usize + (Self::SIZE - a_pos)
        };
        usize::pow(10, (target - 1) as u32) * distance
    }
    /// r_idx: room index \
    /// a_pos: amphipod position in room \
    /// h_idx: hallway index
    fn room_to_hallway(&self, (r_idx, a_pos): (usize, usize), h_idx: usize) -> Option<Self> {
        if !self.room_clear(r_idx, a_pos, true) {
            return None;
        }
        if !self.hallway_clear(r_idx, h_idx, false) {
            return None;
        }
        let target = self.get_rooms()[r_idx][a_pos];
        let hallway = self.new_hallway((r_idx, a_pos), h_idx, target, false);
        let side_rooms = self.new_rooms((r_idx, a_pos), h_idx, target, true);
        Some(Self::new(
            hallway,
            &side_rooms,
        ))
    }

    fn hallway_to_room(&self, h_idx: usize, (r_idx, a_pos): (usize, usize)) -> Option<Self> {
        if !self.room_clear(r_idx, a_pos, false) {
            return None;
        }
        if !self.hallway_clear(r_idx, h_idx, true) {
            return None;
        }
        let target = self.get_hallway()[h_idx];
        if r_idx + 1 != target {
            return None;
        }
        let hallway = self.new_hallway((r_idx, a_pos), h_idx, target, true);
        let side_rooms = self.new_rooms((r_idx, a_pos), h_idx, target, false);
        Some(Self::new(
            hallway,
            &side_rooms,
        ))
    }

    fn new_hallway(&self, (r_idx, a_pos): (usize, usize), h_idx: usize, target: usize, is_leaving: bool) -> Hallway {
        self.get_hallway().iter().enumerate().map(|(pos, &amph)| {
            if pos == h_idx {
                if is_leaving { 0 } else { target }
            } else {
                amph
            }
        }).collect::<Vec<usize>>().try_into().expect("hallway collect fail")
    }

    fn new_rooms(&self, (r_idx, a_pos): (usize, usize), h_idx: usize, target: usize, is_leaving: bool) -> Vec<Vec<usize>> {
        self.get_rooms().iter().enumerate().map(|(ridx, &room)| {
            if ridx == r_idx {
                room.iter().enumerate().map(|(apos, &amph)| {
                    if apos == a_pos {
                        if is_leaving { 0 } else { target }
                    } else {
                        amph
                    }
                // }).collect::<Vec<usize>>().try_into().expect("side_room collect fail")
                }).collect::<Vec<usize>>()
            } else {
                room.to_vec()
            }
        }).collect::<Vec<Vec<usize>>>()
    }

    fn room_clear(&self, r_idx: usize, a_pos: usize, is_leaving: bool) -> bool {
        self.get_rooms()[r_idx].iter().enumerate().filter(|&(pos, amph)| {
            (
                if is_leaving {
                    pos > a_pos
                } else {
                    pos >= a_pos
                }
            ) && amph != &0
        }).count() == 0
    }

    fn hallway_clear(&self, r_idx: usize, h_idx: usize, is_leaving: bool) -> bool {
        if r_idx + 1 >= h_idx {
            // left hallway
            self.get_hallway().iter().enumerate().filter(|&(pos, amph)| {
                (
                    if is_leaving {
                        pos > h_idx
                    } else {
                        pos >= h_idx
                    }
                ) && pos <= r_idx + 1 && amph != &0
            }).count() == 0
        } else {
            // right hallway
            self.get_hallway().iter().enumerate().filter(|&(pos, amph)| {
                (
                    if is_leaving {
                        pos < h_idx
                    } else {
                        pos <= h_idx
                    }
                ) && pos > r_idx + 1 && amph != &0
            }).count() == 0
        }
    }

    fn hv(&self, h_idx: usize) -> char {
        let value = self.get_hallway()[h_idx];
        if value > 0 {
            (value + 64) as u8 as char
        } else {
            '.'
        }
        
    }

    fn rv(&self, r_idx: usize, a_pos: usize) -> char {
        let value =  self.get_rooms()[r_idx][a_pos];
        if value > 0 {
            (value + 64) as u8 as char
        } else {
            '.'
        }
    }

}

#[test]
fn test_room_to_hallway() {
    let start = StateFor2 {
        hallway: [0,4,0,0,0,0,0],
        side_rooms: [
            [4,1],
            [2,3],
            [3,2],
            [1,0],
        ]
    };
    let end = StateFor2 {
        hallway: [0,4,2,0,0,0,0],
        side_rooms: [
            [4,1],
            [2,3],
            [3,0],
            [1,0],
        ]
    };
    assert_eq!(start.room_to_hallway((2,1), 2), Some(end));
    assert_eq!(start.room_to_hallway((2,0), 2), None);
    assert_eq!(start.room_to_hallway((2,1), 1), None);
    let start = StateFor2 {
        hallway: [0,0,4,0,0,0,0],
        side_rooms: [
            [4,1],
            [2,3],
            [3,2],
            [1,0],
        ]
    };
    let end = StateFor2 {
        hallway: [0,0,4,0,0,2,0],
        side_rooms: [
            [4,1],
            [2,3],
            [3,0],
            [1,0],
        ]
    };
    assert_eq!(start.room_to_hallway((2,1), 5), Some(end));
    assert_eq!(start.room_to_hallway((2,0), 5), None);
    assert_eq!(start.room_to_hallway((2,1), 2), None);
}

#[test]
fn test_hallway_to_room() {
    let start = StateFor2 {
        hallway: [0,4,3,0,0,0,0],
        side_rooms: [
            [4,1],
            [2,2],
            [3,0],
            [0,1],
        ]
    };
    let end = StateFor2 {
        hallway: [0,4,0,0,0,0,0],
        side_rooms: [
            [4,1],
            [2,2],
            [3,3],
            [0,1],
        ]
    };
    assert_eq!(start.hallway_to_room(2, (2,1)), Some(end));
    assert_eq!(start.hallway_to_room(2, (3,0)), None);
    assert_eq!(start.hallway_to_room(1, (2,1)), None);
    let start = StateFor2 {
        hallway: [0,4,0,0,2,0,0],
        side_rooms: [
            [4,1],
            [2,0],
            [3,3],
            [1,0],
        ]
    };
    let end = StateFor2 {
        hallway: [0,4,0,0,0,0,0],
        side_rooms: [
            [4,1],
            [2,2],
            [3,3],
            [1,0],
        ]
    };
    assert_eq!(start.hallway_to_room(4, (1,1)), Some(end));
    assert_eq!(start.hallway_to_room(4, (1,0)), None);
    assert_eq!(start.hallway_to_room(1, (3,1)), None);
    assert_eq!(start.hallway_to_room(4, (3,1)), None);
}

#[test]
fn test_energy() {
    let start = StateFor2 {
        hallway: [0,4,0,0,2,0,0],
        side_rooms: [
            [4,1],
            [2,0],
            [3,3],
            [1,0],
        ]
    };
    assert_eq!(start.energy((1, 1), 4, false), 40);
    assert_eq!(start.energy((3, 0), 6, true), 4);
}

fn shortest_path<T: Eq + Hash + Ord + Clone + Copy + StateTrait + Display>(start: T, end: &T) -> Option<usize> {
    let mut dist = HashMap::new();
    let mut heap = BinaryHeap::new();
    dist.insert(start, 0);
    heap.push(Reverse((0, start)));
    let mut step = 0;
    while let Some(item) = heap.pop() {
        let (energy, state) = item.0;
        step += 1;
        // audit ...
        if step % 10000 == 0 {
            println!("{}Energy: {}\n\n----------\n\n", state, energy);
        }

        if &state == end { return Some(energy); }

        if &energy > dist.get(&state).or(Some(&usize::MAX)).unwrap() { continue; }

        for (new_state, new_energy) in state.possible_edge_states() {
            let next = (
                new_energy + energy,
                new_state,
            );

            if &next.0 < dist.get(&next.1).or(Some(&usize::MAX)).unwrap() {
                heap.push(Reverse(next));
                dist.insert(next.1, next.0);
            }
        }
    }
    None
}


fn main() {
    let now = Local::now();
    let start = StateFor2 {
        hallway: [0,0,0,0,0,0,0],
        side_rooms: [
            [4,4],
            [1,3],
            [1,2],
            [2,3],
        ]
    };
    let end = StateFor2 {
        hallway: [0,0,0,0,0,0,0],
        side_rooms: [
            [1,1],
            [2,2],
            [3,3],
            [4,4],
        ]
    };
    let shorest_path1 = shortest_path(start, &end);
    println!("Part 1: {:?}", shorest_path1);
    let duration = Local::now() - now;
    println!("Transform graph cost: {:?}", duration.to_std().unwrap());

    let start = StateFor4 {
        hallway: [0,0,0,0,0,0,0],
        side_rooms: [
            [4,4,4,4],
            [3,2,3,1],
            [2,1,2,3],
            [2,3,1,1],
        ]
    };
    let end = StateFor4 {
        hallway: [0,0,0,0,0,0,0],
        side_rooms: [
            [1,1,1,1],
            [2,2,2,2],
            [3,3,3,3],
            [4,4,4,4],
        ]
    };
    let shorest_path2 = shortest_path(start, &end);
    println!("Part 1: {:?}", shorest_path1);
    println!("Part 2: {:?}", shorest_path2);
}