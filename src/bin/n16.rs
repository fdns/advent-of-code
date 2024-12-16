use std::{
    collections::{HashMap, HashSet, VecDeque},
    hash::Hash,
};

const INPUT: &str = include_str!("input/n16.input");

type Map = Vec<Vec<Node>>;

const COST_STRAIGHT: u32 = 1;
const COST_ROTATE: u32 = 1000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Node {
    Wall,
    Empty,
    Start,
    End,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Vec2 {
    x: isize,
    y: isize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct DirVec2 {
    dir: Vec2,
    pos: Vec2,
}

fn input() -> Map {
    let mut result = Vec::new();
    for line in INPUT.trim().lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            match c {
                '#' => row.push(Node::Wall),
                'S' => row.push(Node::Start),
                'E' => row.push(Node::End),
                '.' => row.push(Node::Empty),
                _ => unreachable!(),
            }
        }
        result.push(row);
    }

    return result;
}

fn find_node(map: &Map, node: Node) -> Vec2 {
    for y in 0..map.len() {
        for x in 0..map[y as usize].len() {
            if node == map[y][x] {
                return Vec2 {
                    x: x as isize,
                    y: y as isize,
                };
            }
        }
    }
    unreachable!()
}

fn shortest_path(map: &Map, start: &DirVec2, costs: &mut HashMap<DirVec2, u32>) {
    let mut process = VecDeque::new();
    process.push_back(*start);

    while let Some(dirpos) = process.pop_front() {
        let current_cost = costs.get(&dirpos).map(|x| *x).unwrap();

        if map[dirpos.pos.y as usize][dirpos.pos.x as usize] == Node::End {
            continue;
        }

        // Rotate
        for (dir, cost) in [
            (dirpos.dir, current_cost + COST_STRAIGHT),
            (rotate(&dirpos.dir, false), current_cost + COST_ROTATE),
            (rotate(&dirpos.dir, true), current_cost + COST_ROTATE),
        ] {
            let mut dv = DirVec2 {
                pos: Vec2 {
                    x: dirpos.pos.x + dir.x,
                    y: dirpos.pos.y + dir.y,
                },
                dir: dir,
            };
            if dv.dir != dirpos.dir {
                dv.pos = dirpos.pos;
            }

            if can_execute(map, &dv, cost, costs) {
                // Insert new cost
                costs.insert(dv, cost);
                // continue
                process.push_back(dv);
            } else {
            }
        }
    }
}

fn can_execute(map: &Map, dv: &DirVec2, cost: u32, costs: &mut HashMap<DirVec2, u32>) -> bool {
    match map[dv.pos.y as usize][dv.pos.x as usize] {
        Node::End | Node::Empty | Node::Start => {
            // If it's already known, continue only if it's lesser
            if let Some(x) = costs.get(dv) {
                if *x <= cost {
                    return false;
                }
            }
            return true;
        }
        Node::Wall => {
            return false;
        }
    }
}

fn main() {
    p1();
    p2();
}

fn p1() {
    let map = input();
    let start = find_node(&map, Node::Start);
    let end = find_node(&map, Node::End);
    let mut cache = HashMap::new();

    // Start facing east
    let dirvec = DirVec2 {
        pos: start,
        dir: Vec2 { x: 1, y: 0 },
    };
    cache.insert(dirvec, 0);
    shortest_path(&map, &dirvec, &mut cache);

    println!("Result 1: {:?}", minimal(&cache, &end));
}

fn best_seat_count(map: &Map, cache: &HashMap<DirVec2, u32>) -> usize {
    let mut result = HashSet::new();
    let end = find_node(map, Node::End);

    // We are going to walk backwards
    let mut process = VecDeque::new();
    for dirvec in minimal_dirvec(cache, &end) {
        process.push_back(dirvec);
    }

    while let Some(dv) = process.pop_front() {
        result.insert(dv.pos);
        let current = cache.get(&dv).unwrap();

        // Look for any previous possible point
        for (dir, cost) in [
            (dv.dir, current - COST_STRAIGHT),
            (rotate(&dv.dir, false), current - COST_ROTATE),
            (rotate(&dv.dir, true), current - COST_ROTATE),
        ] {
            let mut dv = DirVec2 {
                pos: Vec2 {
                    x: dv.pos.x - dir.x,
                    y: dv.pos.y - dir.y,
                },
                dir: dir,
            };
            if dv.dir != dv.dir {
                dv.pos = dv.pos;
            }

            if is_walkback(cache, &dv, cost) {
                process.push_back(dv);
            }
        }
    }

    return result.len();
}

fn is_walkback(costs: &HashMap<DirVec2, u32>, pos: &DirVec2, expected: u32) -> bool {
    if let Some(r) = costs.get(pos) {
        return *r <= expected;
    }
    return false;
}

fn p2() {
    let map = input();
    let start = find_node(&map, Node::Start);
    let mut cache = HashMap::new();

    // Start facing east
    let dirvec = DirVec2 {
        pos: start,
        dir: Vec2 { x: 1, y: 0 },
    };
    cache.insert(dirvec, 0);
    shortest_path(&map, &dirvec, &mut cache);
    println!("Result 2: {}", best_seat_count(&map, &cache));
    print_map(&map, &cache);
}

fn minimal(cost: &HashMap<DirVec2, u32>, pos: &Vec2) -> Option<u32> {
    let mut min = None;
    for (dirx, diry) in [(0, 1), (0, -1), (1, 0), (1, -1)] {
        if let Some(cost) = cost.get(&DirVec2 {
            pos: *pos,
            dir: Vec2 { x: dirx, y: diry },
        }) {
            min = Some(cost.min(min.unwrap_or(&u32::MAX)));
        }
    }

    return min.map(|x| *x);
}

fn minimal_dirvec(costs: &HashMap<DirVec2, u32>, pos: &Vec2) -> Vec<DirVec2> {
    let mut result = Vec::new();
    let mut min = None;
    for (dirx, diry) in [(0, 1), (0, -1), (1, 0), (1, -1)] {
        let dirvec = DirVec2 {
            pos: *pos,
            dir: Vec2 { x: dirx, y: diry },
        };
        if let Some(cost) = costs.get(&dirvec) {
            if result.is_empty() || cost == min.unwrap() {
                result.push(dirvec);
                min = Some(cost);
            } else if cost < min.unwrap() {
                result = vec![dirvec];
                min = Some(cost);
            }
        }
    }

    return result;
}

fn rotate(dir: &Vec2, right: bool) -> Vec2 {
    if right {
        return Vec2 {
            x: dir.y,
            y: -dir.x,
        };
    }
    return Vec2 {
        x: -dir.y,
        y: dir.x,
    };
}

fn print_map(map: &Map, cache: &HashMap<DirVec2, u32>) {
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            print!(
                "{}\t",
                minimal(
                    &cache,
                    &Vec2 {
                        x: x as isize,
                        y: y as isize
                    }
                )
                .unwrap_or(0)
            )
        }
        println!("");
    }
}
