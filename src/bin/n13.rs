use std::collections::{HashMap, HashSet};

use regex::Regex;

const INPUT: &str = include_str!("input/n13.input");
type counter = usize;
const COST_A: counter = 3;
const COST_B: counter = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Counter {
    a: counter,
    b: counter,
}

#[derive(Debug, Default, Clone)]
struct Coord {
    x: counter,
    y: counter,
}

#[derive(Debug, Default, Clone)]
struct Machine {
    a: Coord,
    b: Coord,
    prize: Coord,
}

fn input() -> Vec<Machine> {
    let mut res = Vec::new();
    let mut state = 0;

    let mut current = Machine::default();
    for line in INPUT.trim().lines() {
        match state {
            0 => {
                let re = Regex::new(r"Button A: X\+(?<x>\d+), Y\+(?<y>\d+)").unwrap();
                let result = re.captures(line).expect(line);

                current = Machine::default();
                current.a = Coord {
                    x: result.name("x").unwrap().as_str().parse().unwrap(),
                    y: result.name("y").unwrap().as_str().parse().unwrap(),
                };
            }
            1 => {
                let re = Regex::new(r"Button B: X\+(?<x>\d+), Y\+(?<y>\d+)").unwrap();
                let result = re.captures(line).unwrap();

                current.b = Coord {
                    x: result.name("x").unwrap().as_str().parse().unwrap(),
                    y: result.name("y").unwrap().as_str().parse().unwrap(),
                };
            }
            2 => {
                let re = Regex::new(r"Prize: X=(?<x>\d+), Y=(?<y>\d+)").unwrap();
                let result = re.captures(line).unwrap();

                current.prize = Coord {
                    x: result.name("x").unwrap().as_str().parse().unwrap(),
                    y: result.name("y").unwrap().as_str().parse().unwrap(),
                };
                res.push(current.clone());
            }
            3 => {
                state = 0;
                continue;
            }
            _ => unreachable!(),
        }
        state += 1;
    }

    return res;
}

fn main() {
    p1();
    p2();
}

fn p1() {
    let mut total: usize = 0;
    for m in input() {
        let m = m.clone();

        let tokens = math(&m);
        total += tokens.unwrap_or(0) as usize;
    }

    println!("Result 1: {}", total)
}

fn p2() {
    let mut total: usize = 0;
    for m in input() {
        let mut m = m.clone();
        m.prize.x += 10000000000000;
        m.prize.y += 10000000000000;

        let tokens = math(&m);
        total += tokens.unwrap_or(0) as usize;
    }

    println!("Result 2: {}", total)
}

fn math(m: &Machine) -> Option<usize> {
    let y = (m.prize.y as f64 - (m.prize.x * m.a.y) as f64 / m.a.x as f64);
    let y = y / (m.b.y as f64 - (m.b.x * m.a.y) as f64 / m.a.x as f64);
    let x = (m.prize.x as f64 - y * m.b.x as f64) / m.a.x as f64;

    let (x, y) = (x.round() as usize, y.round() as usize);
    let pos = Coord {
        x: x * m.a.x + y * m.b.x,
        y: x * m.a.y + y * m.b.y,
    };
    if pos.x == m.prize.x && pos.y == m.prize.y {
        return Some(x * COST_A + y * COST_B);
    }

    return None;
}
