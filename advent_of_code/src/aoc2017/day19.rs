use advent_of_code::get_str_array_from_file;

type Direction = (i32, i32);
static SIBLINGS: [Direction; 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

#[derive(Debug)]
struct Point {
    position: (usize, usize),
    direction: Direction,
    at_end: bool,
    letters: Vec<String>,
}

impl Point {
    fn go(&mut self, direction: Direction, actual: bool) -> (usize, usize) {
        let x = ((self.position.0 as i32) + direction.0) as usize;
        let y = ((self.position.1 as i32) + direction.1) as usize;
        if actual {
            self.position = (x, y);
        };
        (x, y)
    }
}

fn is_same_direction(dir_a: Direction, dir_b: Direction) -> bool {
    match dir_a {
        (0, _) => dir_b.0 == 0,
        (_, 0) => dir_b.1 == 0,
        _ => false,
    }
}

fn run(diagram: &Vec<Vec<&str>>, point: &mut Point) {
    point.go(point.direction, true);
    point.at_end = SIBLINGS.iter().filter(|&x| {
        let next_position = point.go(*x, false);
        diagram[next_position.0][next_position.1] == " "
    }).count() == 3;
    let route = diagram[point.position.0][point.position.1];
    match route {
        "-" => (),
        "|" => (),
        " " => (),
        "+" => {
            for &sibling in &SIBLINGS {
                let next_position = point.go(sibling, false);
                let next_route = diagram[next_position.0][next_position.1];
                match next_route {
                    " " => (),
                    _ => {
                        if !is_same_direction(point.direction, sibling) {
                            point.direction = sibling;
                            break;
                        }
                    }
                };
            };
        },
        _ => {
            point.letters.push(String::from(route));
        }
    };
}

fn main() {
    let array = get_str_array_from_file(&vec!{"aoc2017", "day19_data.txt"});
    let diagram: Vec<Vec<&str>> = array.iter().map(|line| line.split("").filter(|&x| x != "").collect()).collect();
    let start_index = diagram[0].iter().position(|&x| x == "|").unwrap();
    let mut point = Point { position:(0, start_index), direction: (1, 0), at_end: false, letters: vec!{} };
    while !point.at_end {
        run(&diagram, &mut point);
    };
    println!("Part 1: {}", point.letters.join(""));
}