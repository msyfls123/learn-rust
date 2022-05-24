static DOOR_PUBLIC_KEY: usize = 13316116;
static CARD_PUBLIC_KEY: usize = 13651422;

fn get_loop_size(subject: usize, target: usize) -> Option<usize> {
    (0..).scan(subject, |state, _index| {
        *state = (*state * 7) % 20201227;
        Some(*state)
    }).enumerate().find_map(|(index, val)| {
        if val == target {
            println!("{} {} {}", target, index, val);
            Some(index + 2)
        } else {
            None
        }
    })
}

fn get_encryption_key(subject: usize, loop_size: usize) -> usize {
    (1..loop_size).fold(subject, |acc, _i| {
        (acc * subject) % 20201227
    })
}

fn main() {
    let door_loop_size = get_loop_size(7, DOOR_PUBLIC_KEY).unwrap();
    let card_loop_size = get_loop_size(7, CARD_PUBLIC_KEY).unwrap();
    let encryption_key = get_encryption_key(DOOR_PUBLIC_KEY, card_loop_size);
    println!("Part 1: {}", encryption_key);
}