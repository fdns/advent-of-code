use std::collections::HashSet;

const INPUT: &str = include_str!("input/n10.input");

fn main() {
    p1();
    p2();
}

type Map = Vec<Vec<u32>>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Pos {
    x: usize,
    y: usize,
}

fn find_starts(map: &Map) -> Vec<Pos> {
    let mut result = Vec::new();
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == 0 {
                result.push(Pos { x: x, y: y });
            }
        }
    }
    return result;
}

fn walk(map: &Map, pos: &Pos, next: u32, trail_ends: &mut HashSet<Pos>) {
    if map[pos.y][pos.x] == 9 && next == 10 {
        trail_ends.insert(*pos);
        return;
    }

    let scan_pos = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    for (dx, dy) in scan_pos {
        if (pos.x as i32 + dx) < 0
            || (pos.y as i32 + dy) < 0
            || (pos.y as i32 + dy) as usize >= map.len()
            || (pos.x as i32 + dx) as usize >= map[(pos.y as i32 + dy) as usize].len()
        {
            continue;
        }
        let (x, y) = ((pos.x as i32 + dx) as usize, (pos.y as i32 + dy) as usize);
        if map[y][x] == next {
            walk(map, &Pos { x: x, y: y }, next + 1, trail_ends);
        }
    }
}

fn walk_path(map: &Map, pos: &Pos, next: u32, trail_ends: &mut HashSet<Vec<Pos>>, path: &Vec<Pos>) {
    if map[pos.y][pos.x] == 9 && next == 10 {
        trail_ends.insert(path.to_vec());
        return;
    }

    let scan_pos = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    for (dx, dy) in scan_pos {
        if (pos.x as i32 + dx) < 0
            || (pos.y as i32 + dy) < 0
            || (pos.y as i32 + dy) as usize >= map.len()
            || (pos.x as i32 + dx) as usize >= map[(pos.y as i32 + dy) as usize].len()
        {
            continue;
        }
        let (x, y) = ((pos.x as i32 + dx) as usize, (pos.y as i32 + dy) as usize);
        if map[y][x] == next {
            let pos = Pos { x: x, y: y };
            let mut path = path.clone();

            path.push(pos);
            walk_path(map, &pos, next + 1, trail_ends, &path);
        }
    }
}

fn input() -> Map {
    let mut map = Vec::new();
    for line in INPUT.trim().lines() {
        map.push(Vec::new());
        let pos = map.len() - 1;
        for c in line.chars() {
            map[pos].push(c.to_digit(10).unwrap());
        }
    }

    return map;
}

fn p1() {
    let map = input();
    let starts = find_starts(&map);
    let mut scores = 0;
    for start in starts {
        let mut result = HashSet::new();
        walk(&map, &start, 1, &mut result);
        scores += result.len();
    }

    println!("Result 1: {}", scores)
}
fn p2() {
    let map = input();
    let starts = find_starts(&map);
    let mut scores = 0;
    for start in starts {
        let mut result = HashSet::new();
        walk_path(&map, &start, 1, &mut result, &vec![start]);
        scores += result.len();
    }

    println!("Result 2: {}", scores)
}
