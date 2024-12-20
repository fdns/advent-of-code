use std::{
    collections::{HashMap, VecDeque},
    usize,
};

const INPUT: &str = include_str!("input/n20.input");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Vec2 {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Node {
    Wall,
    Empty,
    Start,
    End,
}

type Map = Vec<Vec<Node>>;

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

struct Vehicle {
    cost: usize,
    pos: Vec2,
    is_phased: bool,
    phase_remaining: u32,
    cheat_start_pos: Option<Vec2>,
    cheat_end_pos: Option<Vec2>,
}

type MapCacheEntry = (Option<Vec2>, Option<Vec2>);
type MapCache = HashMap<(Vec2, MapCacheEntry), usize>;

fn execute(
    map: &Map,
    start_phase_value: u32,
    max_cost: usize,
    direct_path_cost: &HashMap<Vec2, usize>,
) -> MapCache {
    let start = find_node(map, Node::Start);
    let mut queue = VecDeque::new();
    let mut map_cache = MapCache::new();
    queue.push_back(Vehicle {
        cost: 0,
        pos: start,
        is_phased: false,
        phase_remaining: start_phase_value,
        cheat_start_pos: None,
        cheat_end_pos: None,
    });

    let mut iter = 0;
    let end = find_node(map, Node::End);
    while let Some(vehicle) = queue.pop_front() {
        iter += 1;
        if iter % 1000000 == 0 {
            println!("{} left", queue.len());
            iter = 0;
        }

        // Ignore paths that are too long
        if vehicle.cost > max_cost {
            continue;
        }

        // Validation check, if we are in a wall, we have to be phased
        if map[vehicle.pos.y][vehicle.pos.x] == Node::Wall && !vehicle.is_phased {
            continue;
        }

        // Check if it has not already been visited
        let key = (
            vehicle.pos,
            (vehicle.cheat_start_pos, vehicle.cheat_end_pos),
        );
        if let Some(cost) = map_cache.get(&key) {
            if *cost <= vehicle.cost {
                continue;
            }
        }

        map_cache.insert(key, vehicle.cost);

        // If we are at the end, continue
        if map[vehicle.pos.y][vehicle.pos.x] == Node::End && key.1 .1.is_some() {
            continue;
        }

        // If we have already phased, we can just shortcut with our direct_path_cost
        if vehicle.phase_remaining == 0 {
            let end_key = (end, key.1);
            let end_cost = map_cache.get(&end_key);
            let cost = vehicle.cost
                + direct_path_cost
                    .get(&vehicle.pos)
                    .expect(format!("Position not found {:?}", vehicle.pos).as_str());
            if end_cost.is_none() || *end_cost.unwrap() > cost {
                map_cache.insert(end_key, cost);
            }
            continue;
        }

        // Try to move
        for dir in [[0, 1], [0, -1], [1, 0], [-1, 0]] {
            let (next_x, next_y) = (
                vehicle.pos.x.checked_add_signed(dir[0]),
                vehicle.pos.y.checked_add_signed(dir[1]),
            );
            // Check next position legal
            if next_x.is_none()
                || next_y.is_none()
                || next_y.unwrap() >= map.len()
                || next_x.unwrap() >= map[next_y.unwrap()].len()
            {
                continue;
            }
            let next = Vec2 {
                x: next_x.unwrap(),
                y: next_y.unwrap(),
            };

            // If we are phased, we just try to move
            if vehicle.is_phased {
                let phase_remaining = vehicle.phase_remaining - 1;
                let mut end_pos = None;
                if phase_remaining == 0 {
                    end_pos = Some(next);
                }
                queue.push_back(Vehicle {
                    pos: next,
                    cost: vehicle.cost + 1,
                    is_phased: phase_remaining > 0,
                    cheat_end_pos: end_pos,
                    cheat_start_pos: vehicle.cheat_start_pos,
                    phase_remaining: phase_remaining,
                });
                // We also try to unphase forcefully
                if phase_remaining > 0 {
                    queue.push_back(Vehicle {
                        pos: next,
                        cost: vehicle.cost + 1,
                        is_phased: false,
                        cheat_end_pos: Some(next),
                        cheat_start_pos: vehicle.cheat_start_pos,
                        phase_remaining: 0,
                    });
                }
            } else if map[next.y][next.x] == Node::Wall && vehicle.phase_remaining > 0 {
                // Vehicle can phase through wall
                queue.push_back(Vehicle {
                    pos: next,
                    cost: vehicle.cost + 1,
                    is_phased: true,
                    cheat_end_pos: None,
                    cheat_start_pos: Some(vehicle.pos),
                    phase_remaining: vehicle.phase_remaining - 1,
                })
            } else if map[next.y][next.x] != Node::Wall {
                // Normal walk
                queue.push_back(Vehicle {
                    pos: next,
                    cost: vehicle.cost + 1,
                    is_phased: false,
                    cheat_end_pos: vehicle.cheat_end_pos,
                    cheat_start_pos: vehicle.cheat_start_pos,
                    phase_remaining: vehicle.phase_remaining,
                });

                // try to just start phasing now
                if vehicle.phase_remaining > 0 {
                    queue.push_back(Vehicle {
                        pos: next,
                        cost: vehicle.cost + 1,
                        is_phased: true,
                        cheat_end_pos: None,
                        cheat_start_pos: Some(vehicle.pos),
                        phase_remaining: vehicle.phase_remaining - 1,
                    })
                }
            }
        }
    }

    return map_cache;
}

fn calculate_raw_costs(map: &Map) -> HashMap<Vec2, usize> {
    let mut result = HashMap::new();
    let mut queue = VecDeque::new();
    queue.push_back((find_node(map, Node::End), 0));

    // Go from the end to the start
    while let Some((pos, cost)) = queue.pop_front() {
        // Early return
        if let Some(c) = result.get(&pos) {
            if *c <= cost {
                continue;
            }
        }
        result.insert(pos, cost);

        if map[pos.y][pos.x] == Node::Start {
            continue;
        }

        // Try to move
        for dir in [[0, 1], [0, -1], [1, 0], [-1, 0]] {
            let next = Vec2 {
                x: pos.x.checked_add_signed(dir[0]).unwrap(),
                y: pos.y.checked_add_signed(dir[1]).unwrap(),
            };
            if map[next.y][next.x] != Node::Wall {
                queue.push_back((next, cost + 1));
            }
        }
    }

    return result;
}

fn find_node(map: &Map, node: Node) -> Vec2 {
    for y in 0..map.len() {
        for x in 0..map[y as usize].len() {
            if node == map[y][x] {
                return Vec2 { x, y };
            }
        }
    }
    unreachable!()
}

fn p1() {
    let minimum_avings = 100;
    let map = input();
    let costs = calculate_raw_costs(&map);
    let initial_cost = costs.get(&find_node(&map, Node::Start)).unwrap();
    // println!("Initial cost: {}", initial_cost);

    let result = execute(&map, 2, *initial_cost - minimum_avings, &costs);

    let end = find_node(&map, Node::End);
    let mut count = HashMap::<usize, Vec<(Option<Vec2>, Option<Vec2>)>>::new();
    let mut total = 0;
    for ((pos, cheat), cost) in &result {
        if *pos == end && initial_cost.checked_sub(*cost).is_some() {
            let savings = initial_cost - cost;
            count.entry(savings).or_default().push(*cheat);

            if savings >= minimum_avings {
                total += 1;
            }
        }
    }
    // println!("Count: {:?}", count.get(&64).unwrap_or(&vec![]).len());
    // println!("Count: {:?}", count.get(&8).unwrap_or(&vec![]).len());
    println!("Result 1: {}", total);
}

fn p2() {
    let minimum_avings = 100;
    let starting_phase = 20;

    let map = input();
    let costs = calculate_raw_costs(&map);
    let initial_cost = costs.get(&find_node(&map, Node::Start)).unwrap();
    // println!("Initial cost: {}", initial_cost);

    let result = execute(&map, starting_phase, *initial_cost - minimum_avings, &costs);

    let end = find_node(&map, Node::End);
    let mut count = HashMap::<usize, Vec<MapCacheEntry>>::new();
    let mut total = 0;
    for ((pos, cheat), cost) in &result {
        if cheat.0.is_some() && cheat.1.is_none() {
            // Remove invalid'ish nodes
            continue;
        }
        if *pos == end && initial_cost.checked_sub(*cost).is_some() {
            let savings = initial_cost - cost;
            count.entry(savings).or_default().push(*cheat);

            if savings >= minimum_avings {
                total += 1;
            }
        }
    }
    // let count: HashMap<usize, HashSet<MapCacheEntry>> = count
    //     .iter()
    //     .map(|(k, x)| (*k, HashSet::from_iter(x.iter().cloned())))
    //     .collect();

    // println!(
    //     "Count 52 (exp 31): {:?}",
    //     count.get(&52).unwrap_or(&HashSet::new()).len()
    // ); // 19 in example
    // println!(
    //     "Count 50 (exp 32): {:?}",
    //     count.get(&50).unwrap_or(&HashSet::new()).len()
    // ); // 32 in example
    // println!(
    //     "Count 74 (exp 4): {:?}",
    //     count.get(&74).unwrap_or(&HashSet::new()).len()
    // );
    // println!(
    //     "Count 72 (exp 22): {:?}",
    //     count.get(&72).unwrap_or(&HashSet::new()).len()
    // );
    // println!("Count 50: {:?}", count.get(&76).unwrap_or(&HashSet::new())); // 3 in example
    println!("Result 2: {}", total);
}

fn main() {
    p1();
    p2();
}
