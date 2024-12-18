use std::collections::{HashMap, VecDeque};

const INPUT: &str = include_str!("input/n18.input");

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Vec2 {
    x: usize,
    y: usize,
}

type MapVec = Vec<Vec2>;
type ObstacleMap = Vec<Vec<bool>>;

fn input() -> MapVec {
    let mut result = Vec::new();
    for line in INPUT.trim().lines() {
        let mut nums = line.split(",");
        result.push(Vec2 {
            x: nums.next().unwrap().parse().unwrap(),
            y: nums.next().unwrap().parse().unwrap(),
        });
    }

    return result;
}

fn insert_obstacle(map: &mut ObstacleMap, pos: &Vec2) {
    map[pos.y][pos.x] = true;
}
fn insert_obstacles(map: &mut ObstacleMap, pos: &MapVec) {
    for pos in pos {
        insert_obstacle(map, &pos);
    }
}

fn new_map(x: usize, y: usize) -> ObstacleMap {
    let mut result = ObstacleMap::with_capacity(y);
    for _ in 0..y {
        result.push((0..x).map(|_| false).collect());
    }

    return result;
}

fn main() {
    p1();
    p2();
}

fn p1() {
    let mut mapvec = input();
    mapvec.truncate(1024);
    let mut map = new_map(71, 71);
    insert_obstacles(&mut map, &mapvec);
    let distance = pathfind(&map);

    println!("Result 1: {}", distance.unwrap());
}

fn pathfind(map: &ObstacleMap) -> Option<i32> {
    let mut weights = HashMap::new();
    let mut queue = VecDeque::new();
    queue.push_back((Vec2 { x: 0, y: 0 }, 0));
    while let Some((pos, distance)) = queue.pop_front() {
        // ignore closer nodes
        if let Some(w) = weights.get(&pos) {
            if *w <= distance {
                continue;
            }
        }
        weights.insert(pos, distance);

        // Push next nodes
        for [x, y] in [[1, 0], [-1, 0], [0, 1], [0, -1]] {
            let a = pos.x.checked_add_signed(x);
            let b = pos.y.checked_add_signed(y);
            if a.is_some()
                && b.is_some()
                && b.unwrap() < map.len()
                && a.unwrap() < map[b.unwrap()].len()
            {
                let next = Vec2 {
                    x: a.unwrap(),
                    y: b.unwrap(),
                };

                if !map[next.y][next.x] {
                    queue.push_back((next, distance + 1));
                }
            }
        }
    }

    return weights
        .get(&Vec2 {
            y: map.len() - 1,
            x: map[map.len() - 1].len() - 1,
        })
        .map(|x| *x);
}

fn p2() {
    let mapvec = input();
    let mut map = new_map(71, 71);

    for i in 0..mapvec.len() {
        insert_obstacle(&mut map, &mapvec[i]);

        let distance = pathfind(&map);
        if distance == None {
            println!("Result 2: {:?}", mapvec[i]);
            break;
        }
    }
}
