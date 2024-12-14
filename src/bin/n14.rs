use std::{
    collections::{HashMap, HashSet},
    io::{stdout, Write},
};

use regex::Regex;
const INPUT: &str = include_str!("input/n14.input");

fn main() {
    p1();
    p2();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Vec2 {
    x: isize,
    y: isize,
}

#[derive(Debug, Clone, Copy)]
struct Robot {
    pos: Vec2,
    vel: Vec2,
}

fn input() -> Vec<Robot> {
    let mut result = Vec::new();
    let reg = Regex::new(r"p=(?<x>\d+),(?<y>\d+) v=(?<vx>-?\d+),(?<vy>-?\d+)").unwrap();
    for line in INPUT.trim().lines() {
        let capt = reg.captures(line).unwrap();
        let pos = Vec2 {
            x: capt.name("x").unwrap().as_str().parse().unwrap(),
            y: capt.name("y").unwrap().as_str().parse().unwrap(),
        };
        let vel = Vec2 {
            x: capt.name("vx").unwrap().as_str().parse::<isize>().unwrap(),
            y: capt.name("vy").unwrap().as_str().parse::<isize>().unwrap(),
        };

        result.push(Robot { pos: pos, vel: vel });
    }
    return result;
}

fn p1() {
    let input = input();
    let bounds = Vec2 { x: 101, y: 103 };
    // let bounds = Vec2 { x: 11, y: 7 };
    let result = execute(&input, bounds, 100);
    let safety = safety(&result, bounds);
    println!("Result1: {}", safety);
}

fn execute(input: &Vec<Robot>, bounds: Vec2, time: isize) -> Vec<Robot> {
    let mut result = input.clone();

    for robot in result.iter_mut() {
        robot.pos.x = ((robot.pos.x + robot.vel.x * time) % bounds.x + bounds.x) % bounds.x;
        robot.pos.y = ((robot.pos.y + robot.vel.y * time) % bounds.y + bounds.y) % bounds.y;
    }

    return result;
}

fn safety(input: &Vec<Robot>, bounds: Vec2) -> usize {
    let mut count = HashMap::<Vec2, usize>::new();
    for robot in input {
        let cnt = count.entry(robot.pos).or_default();
        *cnt += 1;
    }
    let mut quads = [0; 4];
    for (pos, count) in count {
        // Determine the quadrant
        if pos.x < bounds.x / 2 {
            if pos.y < bounds.y / 2 {
                quads[0] += count;
            } else if pos.y > bounds.y / 2 {
                quads[1] += count;
            }
        } else if pos.x > bounds.x / 2 {
            if pos.y < bounds.y / 2 {
                quads[2] += count;
            } else if pos.y > bounds.y / 2 {
                quads[3] += count;
            }
        }
    }

    return quads.iter().fold(1, |acc, x| acc * x);
}

fn p2() {
    let bounds = Vec2 { x: 101, y: 103 };
    let mut current = input();
    for time in 1..1_000_000 {
        current = execute(&current, bounds, 1);

        // Check for trees
        if has_tree_like(&current) {
            print(&current, &bounds);
            println!("Tree like at {}", time);
            break;
        }

        if time % 10_000 == 0 {
            println!("Time: {}", time);
        }
    }
}

fn has_tree_like(input: &Vec<Robot>) -> bool {
    // Look for something like 1,3,5 lines
    let map = input.iter().map(|x| x.pos).collect::<HashSet<_>>();

    for robot in input {
        // shoudl start with enough space
        if robot.pos.x < 2 {
            continue;
        }

        // ..*.. (2, 0)
        // .***.. (1, 1) - (3, 1)
        // ******
        // Look for levels below
        let mut found = true;
        for dy in 1..3 {
            for x in (robot.pos.x - dy)..(robot.pos.x + dy + 1) {
                // x in 1..3
                let pos = Vec2 {
                    x: x,
                    y: robot.pos.y + dy,
                };
                if !map.contains(&pos) {
                    found = false;
                    break;
                }
            }
        }
        if found {
            return true;
        }
    }

    return false;
}

fn print(robot: &Vec<Robot>, bounds: &Vec2) {
    let map = robot.iter().map(|x| x.pos).collect::<HashSet<_>>();
    let mut stdout = stdout().lock();
    for y in 0..bounds.y {
        let mut buf = Vec::with_capacity(bounds.x as usize);
        for x in 0..bounds.x {
            if map.contains(&Vec2 { x: x, y: y }) {
                buf.push('*' as u8);
            } else {
                buf.push('.' as u8);
            }
        }
        buf.push('\n' as u8);
        stdout.write(&buf).unwrap();
    }
}
