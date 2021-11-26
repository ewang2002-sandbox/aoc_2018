use std::collections::{HashSet};
use std::fmt::{Display, Formatter, Write};

const STRAIGHT_VERT: char = '|';
const STRAIGHT_HORIZ: char = '-';
const CURVE_SLASH: char = '/';
const CURVE_BACKSLASH: char = '\\';
const INTERSECTION: char = '+';

const CART_UP: char = '^';
const CART_DOWN: char = 'v';
const CART_LEFT: char = '<';
const CART_RIGHT: char = '>';

type Cart = (char, u8, usize, usize);
type Coord = (usize, usize);

#[allow(dead_code)]
pub fn execute(input: &Vec<String>) -> (String, i64) {
    let char_input = input.iter().map(|x| x.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

    let mut track_system: Vec<Vec<Track>> = vec![];
    let mut active_carts: Vec<Cart> = vec![];
    for y in 0..char_input.len() {
        let mut row_track: Vec<Track> = vec![];
        for x in 0..char_input[y].len() {
            let track = Track::new(char_input[y][x], x, y);
            match char_input[y][x] {
                CART_UP | CART_DOWN | CART_RIGHT | CART_LEFT => {
                    active_carts.push((char_input[y][x], 0, x, y));
                }
                _ => {}
            }

            row_track.push(track);
        }

        track_system.push(row_track);
    }

    return (part1(&track_system, &active_carts), part2(input));
}

pub fn part1(track_system: &Vec<Vec<Track>>, active_carts: &Vec<Cart>) -> String {
    let mut all_active_coords: HashSet<Coord> = active_carts
        .iter()
        .map(|x| ((*x).2, (*x).3))
        .collect();

    let mut all_active_carts = active_carts.clone();


    let mut conflicting_coords: Option<Coord> = None;
    while conflicting_coords.is_none() {
        print_track(track_system, &all_active_carts);
        let mut next_active_carts: Vec<Cart> = vec![];
        let mut next_active_coords: HashSet<Coord> = HashSet::new();
        for (cart, dir, x, y) in all_active_carts {
            all_active_coords.remove(&(x, y));
            // determine the next track
            let next_track = match cart {
                CART_DOWN => &track_system[y + 1][x],
                CART_UP => &track_system[y - 1][x],
                CART_RIGHT => &track_system[y][x + 1],
                CART_LEFT => &track_system[y][x - 1],
                _ => panic!("invalid cart given: {}", cart)
            };

            // determine the next cart
            let next_cart = match next_track.raw_track_type {
                STRAIGHT_VERT | STRAIGHT_HORIZ => cart,
                CURVE_SLASH => match cart {
                    '^' => '>',
                    '<' => 'v',
                    _ => panic!("invalid cart {} given with dir {}", cart, CURVE_SLASH)
                },
                CURVE_BACKSLASH => match cart {
                    '>' => 'v',
                    '^' => '<',
                    _ => panic!("invalid cart {} given with dir {}", cart, CURVE_SLASH)
                },
                INTERSECTION => match dir {
                    // Left
                    0 => match cart {
                        'v' => '>',
                        '>' => '^',
                        '^' => '<',
                        '<' => 'v',
                        _ => panic!("invalid cart {}", cart)
                    },
                    // Straight
                    1 => cart,
                    // Right
                    2 => match cart {
                        'v' => '<',
                        '>' => 'v',
                        '^' => '>',
                        '<' => '^',
                        _ => panic!("invalid cart {}", cart)
                    },
                    _ => panic!("invalid cart {} given with dir", cart)
                },
                _ => panic!("invalid track {}", next_track.raw_track_type)
            };

            next_active_carts.push((
                next_cart,
                if next_track.raw_track_type == INTERSECTION { (dir + 1) % 3 } else { dir },
                next_track.x_pos,
                next_track.y_pos
            ));

            let coord: Coord = (next_track.x_pos, next_track.y_pos);
            if next_active_coords.contains(&coord) || all_active_coords.contains(&coord) {
                conflicting_coords = Some(coord);
                break;
            }

            next_active_coords.insert(coord);
        }

        all_active_carts = next_active_carts;
        all_active_coords = next_active_coords;
    }

    return format!("{},{}", conflicting_coords.unwrap().0, conflicting_coords.unwrap().1);
}


pub fn part2(input: &Vec<String>) -> i64 {
    return input.len() as i64;
}


fn print_track(track: &Vec<Vec<Track>>, carts: &Vec<Cart>) -> () {
    for y in 0..track.len() {
        for x in 0..track[y].len() {
            let res = carts.iter().find(|r| (**r).2 == x && (**r).3 == y);
            match res {
                Some(c) => print!("{}", (*c).0),
                None => print!("{}", track[y][x].raw_track_type)
            };
        }

        println!();
    }
}

pub struct Track {
    raw_track_type: char,
    x_pos: usize,
    y_pos: usize,
}

impl Track {
    pub fn new(track_char: char, x_pos: usize, y_pos: usize) -> Self {
        return Track { x_pos, y_pos, raw_track_type: track_char };
    }
}

impl Display for Track {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return f.write_char(self.raw_track_type);
    }
}