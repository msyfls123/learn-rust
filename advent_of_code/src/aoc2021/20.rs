use std::collections::HashMap;

use advent_of_code::get_group_str_from_file;

type Pixel = (isize, isize);
type Image = HashMap<Pixel, bool>;

struct Boundry {
    x_min: isize,
    x_max: isize,
    y_min: isize,
    y_max: isize,
}

fn parse_image<T>(lines: &Vec<T>) -> Image
where
  T: Into<String> + Clone,
{
    let mut map = HashMap::new();
    lines.iter().enumerate().for_each(|(y, str)| {
        let line: String = str.clone().into();
        line.chars().enumerate().for_each(|(x, v)| {
            map.insert((x as isize, y as isize), v == '#');
        })
    });
    map
}

fn print_image(image: &Image) {
    let boundry = find_boundry(image);
    for y in boundry.y_min-1..=boundry.y_max+1 {
        println!("");
        for x in boundry.x_min-1..=boundry.x_max+1 {
            match image.get(&(x, y)) {
                Some(true) => { print!("#")},
                _ => { print!(".") },
            }
        }
    }
    println!("");
}

fn find_boundry(image: &Image) -> Boundry {
    let x_min = image.keys().map(|(x, _)| x.to_owned()).min().unwrap();
    let x_max = image.keys().map(|(x, _)| x.to_owned()).max().unwrap();
    let y_min = image.keys().map(|(_, y)| y.to_owned()).min().unwrap();
    let y_max = image.keys().map(|(_, y)| y.to_owned()).max().unwrap();
    Boundry { x_min, x_max, y_min, y_max }
}

fn enhance_pixel(pixel: &Pixel, image: &Image, algorithom: &str, round: usize) -> bool {
    let will_flip = algorithom.chars().nth(0).unwrap() == '#';
    let default_pixel = if will_flip {
        round % 2 == 1
    } else {
        false
    };
    let (x, y) = pixel.to_owned();
    let corresponding_pixels: Vec<bool> = [
        (x - 1, y - 1),
        (x, y - 1),
        (x + 1, y - 1),
        (x - 1, y),
        (x, y),
        (x + 1, y),
        (x - 1, y + 1),
        (x, y + 1),
        (x + 1, y + 1),
    ].iter()
        .map(|curr| image.get(curr).map_or(round % 2 == 1, |x| *x))
        .collect();
    let text = corresponding_pixels.iter().map(|&x| {
        if x { '1' } else { '0' }
    }).collect::<String>();
    let index = usize::from_str_radix(&text, 2).unwrap();
    let res = algorithom.chars().nth(index).unwrap() == '#';
    // println!("{:?} {}: {}", pixel, index, res);
    res
}

fn enhance_image(image: &Image, algorithom: &str, round: usize) -> Image {
    let mut map = HashMap::new();
    let boundry = find_boundry(image);
    for y in boundry.y_min-1..=boundry.y_max+1 {
        for x in boundry.x_min-1..=boundry.x_max+1 {
            map.insert((x, y), enhance_pixel(&(x, y), image, algorithom, round));
        }
    }
    map
}

fn main() {
    let data = get_group_str_from_file(&vec!{"aoc2021", "data", "20.txt"});
    let orig_image = parse_image(&data[1]);
    let algorithom = data[0][0].to_owned();
    let mut image = orig_image.to_owned();
    print_image(&image);
    for round in 0..2 {
        image = enhance_image(&image, &algorithom, round);
        print_image(&image);
    }
    println!("Part 1: {}", image.values().filter(|&x| *x).count());
}