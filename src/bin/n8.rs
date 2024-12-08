use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("input/n8.input");

type Map = Vec<Vec<Element>>;
type Antennas = HashMap<char, Vec<(i32, i32)>>;

#[derive(PartialEq, Eq)]
enum Element {
    Empty,
    Antenna(char),
}

fn input() -> Map {
    let mut result = Vec::new();

    for line in INPUT.trim().lines() {
        let mut x = Vec::new();
        for pos in line.chars() {
            match pos {
                '.' => x.push(Element::Empty),
                antenna => x.push(Element::Antenna(antenna)),
            }
        }

        result.push(x);
    }

    return result;
}

fn antennas(map: &Map) -> Antennas {
    let mut result = HashMap::new();
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if let Element::Antenna(f) = map[y][x] {
                result
                    .entry(f)
                    .or_insert(Vec::new())
                    .push((x as i32, y as i32));
            }
        }
    }

    return result;
}

fn main() {
    p1();
    p2();
}

fn p1() {
    let map = input();
    let antennas = antennas(&map);

    let mut result = HashSet::<(i32, i32)>::new();
    for (_, pos) in antennas {
        for a in 0..pos.len() - 1 {
            for b in a + 1..pos.len() {
                if a == b {
                    continue;
                }

                let f1 = pos[a];
                let f2 = pos[b];
                let antinodes = calculate_antinodes(f1, f2);
                for antinode in antinodes {
                    if in_map(&map, antinode) {
                        result.insert(antinode);
                    }
                }
            }
        }
    }

    println!("Result1: {}", result.len());
}

fn calculate_antinodes(a: (i32, i32), b: (i32, i32)) -> [(i32, i32); 2] {
    fn antinode(x0: i32, x1: i32, flip: bool) -> i32 {
        let d = (x0 - x1).abs();
        let right = x0 > x1;
        return flip
            .then_some(x1 + d * right.then_some(-1).unwrap_or(1))
            .unwrap_or(x0 + d * right.then_some(1).unwrap_or(-1));
    }

    return [
        (antinode(a.0, b.0, false), antinode(a.1, b.1, false)),
        (antinode(a.0, b.0, true), antinode(a.1, b.1, true)),
    ];
}

fn in_map(map: &Map, (x, y): (i32, i32)) -> bool {
    return x >= 0
        && y >= 0
        && y < map.len().try_into().unwrap()
        && x < map[y as usize].len().try_into().unwrap();
}

fn p2() {
    let map = input();
    let antennas = antennas(&map);

    let mut result = HashSet::<(i32, i32)>::new();
    for (_, pos) in antennas {
        for a in 0..pos.len() - 1 {
            for b in a + 1..pos.len() {
                if a == b {
                    continue;
                }

                let f1 = pos[a];
                let f2 = pos[b];
                let dist = distance(f1, f2);

                let (mut x, mut y) = (f1.0, f1.1);
                loop {
                    result.insert((x, y));
                    x += dist.0;
                    y += dist.1;
                    if !in_map(&map, (x, y)) {
                        break;
                    }
                }

                let (mut x, mut y) = (f1.0, f1.1);
                loop {
                    result.insert((x, y));
                    x -= dist.0;
                    y -= dist.1;
                    if !in_map(&map, (x, y)) {
                        break;
                    }
                }
            }
        }
    }

    println!("Result2: {}", result.len());
}

fn distance(a: (i32, i32), b: (i32, i32)) -> (i32, i32) {
    let dx = b.0 - a.0;
    let dy = b.1 - a.1;
    return (dx, dy);
}
