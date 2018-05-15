extern crate learn;
// use learn::url;
// use learn::print;
// use learn::thread;
mod adventofcode;
mod impl_trait;
use adventofcode as aoc;

fn main() {
    // url::run();
    // print::run();
    // thread::process();
    aoc::a2017_2::resolve();
    let mut r = impl_trait::Rectangle {
        x: 2.0,
        y: 2.0,
        width: 47,
        height: 47,
    };

    assert!(r.is_square());

    r.update_y(42);
    assert!(!r.is_square());
}