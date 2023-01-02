#![feature(generic_const_exprs)]
use std::{convert::TryInto, fmt::Display, collections::{HashMap, BinaryHeap}, default};
use chrono::{Local};

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
    const RoomSize: usize;
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

#[derive(Clone, Copy, PartialEq, Eq)]
struct Situation {
    energy: usize,
    state: StateFor2,
}

impl Ord for Situation {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.energy.cmp(&self.energy)
            .then_with(|| self.state.cmp(&other.state))
    }
}

impl PartialOrd for Situation {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
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

impl <T: HasHallwayAndRooms + Clone> StateTrait for T where [(); Self::SIZE]: {
    const RoomSize: usize = Self::SIZE;
    fn possible_edge_states(&self) -> Vec<(Self, usize)> {
        let hallway_states: Vec<(Self, usize)> = self.get_hallway().iter().enumerate().flat_map(|(h_idx, &value)| {
            if value == 0 { return vec!{}; }
            let rooms: [[usize; Self::SIZE]; 4] = self.get_rooms();
            rooms.iter().enumerate().flat_map(|(r_idx, room)| {
                room.iter().enumerate().filter_map(move |(a_pos, &r_value)| {
                    if r_value != 0 { return None; }
                    // not destination room
                    if r_idx + 1 != value { return None; }
                    self.hallway_to_room(h_idx, (r_idx, a_pos)).map(|state| (
                        state,
                        self.energy((r_idx, a_pos), h_idx, false)
                    ))
                })
            }).collect()
        }).collect();
        let room_states: Vec<(Self, usize)> = self.get_rooms().iter().enumerate().flat_map(|(r_idx, room)| {
            room.iter().enumerate().flat_map(|(a_pos, &r_value)| {
                if r_value == 0 { return vec!{}; }
                self.get_hallway().iter().enumerate().filter_map(|(h_idx, &h_value)| {
                    if h_value != 0 { return None };
                    self.room_to_hallway((r_idx, a_pos), h_idx).map(|state| (
                        state,
                        self.energy((r_idx, a_pos), h_idx, true)
                    ))
                }).collect()
            }).collect::<Vec<(Self, usize)>>()
        }).collect();
        [hallway_states, room_states].concat()
    }

    fn energy(&self, (r_idx, a_pos): (usize, usize), h_idx: usize, room_to_hallway: bool) -> usize {
        let target = if room_to_hallway {
            self.get_rooms()[r_idx][a_pos]
        } else {
            self.get_hallway()[h_idx]
        };
        let distance = match h_idx {
            0 => (r_idx * 2 + 2) + (2 - a_pos),
            6 => (8 - r_idx * 2) + (2 - a_pos),
            num => ((num as isize) * 2 - (r_idx as isize) * 2 - 3).abs() as usize + (2 - a_pos)
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

fn shortest_path(start: StateFor2, end: &StateFor2) -> Option<usize> {
    let mut dist = HashMap::new();
    let mut heap = BinaryHeap::new();
    dist.insert(start, 0);
    heap.push(Situation { energy: 0, state: start });
    while let Some(Situation { energy, state }) = heap.pop() {
        if &state == end { return Some(energy); }

        if &energy > dist.get(&state).or(Some(&usize::MAX)).unwrap() { continue; }

        for (new_state, new_energy) in state.possible_edge_states() {
            let next = Situation {
                energy: new_energy + energy,
                state: new_state,
            };

            if &next.energy < dist.get(&next.state).or(Some(&usize::MAX)).unwrap() {
                heap.push(next);
                dist.insert(next.state, next.energy);
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
    let shorest_path = shortest_path(start, &end);
    println!("{:?}", shorest_path);
    let duration = Local::now() - now;
    println!("Transform graph cost: {:?}", duration.to_std().unwrap());
}