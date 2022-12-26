use std::convert::TryInto;

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

type Room = [usize; 2];
type Hallway = [usize; 7];

#[derive(Debug, PartialEq, Eq, Hash, Default, Clone, Copy)]
struct State {
    hallway: Hallway,
    side_rooms: [Room; 4],
}

impl State {
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
        let target = self.side_rooms[r_idx][a_pos];
        let hallway = self.new_hallway((r_idx, a_pos), h_idx, target, false);
        let side_rooms = self.new_rooms((r_idx, a_pos), h_idx, target, true);;
        Some(Self {
            hallway,
            side_rooms,
        })
    }

    fn hallway_to_room(&self, h_idx: usize, (r_idx, a_pos): (usize, usize)) -> Option<Self> {
        if !self.room_clear(r_idx, a_pos, false) {
            return None;
        }
        if !self.hallway_clear(r_idx, h_idx, true) {
            return None;
        }
        let target = self.hallway[h_idx];
        if r_idx + 1 != target {
            return None;
        }
        let hallway = self.new_hallway((r_idx, a_pos), h_idx, target, true);
        let side_rooms = self.new_rooms((r_idx, a_pos), h_idx, target, false);;
        Some(Self {
            hallway,
            side_rooms,
        })
    }

    fn new_hallway(&self, (r_idx, a_pos): (usize, usize), h_idx: usize, target: usize, is_leaving: bool) -> Hallway {
        self.hallway.iter().enumerate().map(|(pos, &amph)| {
            if pos == h_idx {
                if is_leaving { 0 } else { target }
            } else {
                amph
            }
        }).collect::<Vec<usize>>().try_into().expect("hallway collect fail")
    }

    fn new_rooms(&self, (r_idx, a_pos): (usize, usize), h_idx: usize, target: usize, is_leaving: bool) -> [Room; 4] {
        self.side_rooms.iter().enumerate().map(|(ridx, &room)| {
            if ridx == r_idx {
                room.iter().enumerate().map(|(apos, &amph)| {
                    if apos == a_pos {
                        if is_leaving { 0 } else { target }
                    } else {
                        amph
                    }
                }).collect::<Vec<usize>>().try_into().expect("side_room collect fail")
            } else {
                room
            }
        }).collect::<Vec<Room>>().try_into().expect("side_rooms collect fail")
    }

    fn room_clear(&self, r_idx: usize, a_pos: usize, is_leaving: bool) -> bool {
        self.side_rooms[r_idx].iter().enumerate().filter(|&(pos, amph)| {
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
            self.hallway.iter().enumerate().filter(|&(pos, amph)| {
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
            self.hallway.iter().enumerate().filter(|&(pos, amph)| {
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
}

#[test]
fn test_room_to_hallway() {
    let start = State {
        hallway: [0,4,0,0,0,0,0],
        side_rooms: [
            [4,1],
            [2,3],
            [3,2],
            [1,0],
        ]
    };
    let end = State {
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
    let start = State {
        hallway: [0,0,4,0,0,0,0],
        side_rooms: [
            [4,1],
            [2,3],
            [3,2],
            [1,0],
        ]
    };
    let end = State {
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
    let start = State {
        hallway: [0,4,3,0,0,0,0],
        side_rooms: [
            [4,1],
            [2,2],
            [3,0],
            [0,1],
        ]
    };
    let end = State {
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
    let start = State {
        hallway: [0,4,0,0,2,0,0],
        side_rooms: [
            [4,1],
            [2,0],
            [3,3],
            [1,0],
        ]
    };
    let end = State {
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

fn main() {

}