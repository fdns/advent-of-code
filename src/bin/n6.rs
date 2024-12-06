use std::collections::HashSet;

const INPUT: &str = include_str!("input/n6.input");

fn main() {
    p1();
    p2();
}

type Direction = f64;
#[derive(Clone)]
enum MapElement {
    Empty,
    Wall,
    Guard(Direction),
}

type Map = Vec<Vec<MapElement>>;
#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
struct Position {
    x: usize,
    y: usize,
}

fn parse() -> Map {
    let mut map = Map::new();
    for line in INPUT.lines() {
        map.push(
            line.chars()
                .map(|x| match x {
                    '.' => MapElement::Empty,
                    '#' => MapElement::Wall,
                    '^' => MapElement::Guard(std::f64::consts::PI / 2.),
                    '>' => MapElement::Guard(0.),
                    '<' => MapElement::Guard(std::f64::consts::PI),
                    'v' => MapElement::Guard(std::f64::consts::PI * 3. / 2.),
                    _ => unreachable!("invalid character"),
                })
                .collect(),
        );
    }

    return map;
}

fn p1() {
    let mut map = parse();
    let mut tracker = HashSet::new();
    let mut guard = find_guard(&map);

    loop {
        if let MapElement::Guard(dir) = map[guard.y][guard.x] {
            // Add current position to tracker
            tracker.insert(guard);

            let next = next_position(&map, guard, dir);
            if next.unwrap().is_none() {
                break;
            }

            // Move guard
            let (pos, dir) = next.unwrap().unwrap();
            map[guard.y][guard.x] = MapElement::Empty;
            map[pos.y][pos.x] = MapElement::Guard(dir);
            guard = pos;
        }
    }

    println!("Result1: {}", tracker.len())
}

fn p2() {
    let map = parse();
    let mut tracker = 0;
    let guard = find_guard(&map);

    // Brute force each option to cover and cause a loop
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if let MapElement::Empty = map[x][y] {
                let mut map = map.clone();
                map[x][y] = MapElement::Wall;
                if !can_exit_map(&mut map, &guard) {
                    tracker += 1;
                }
            }
        }
    }

    println!("Result2: {}", tracker)
}

fn can_exit_map(map: &mut Map, guard: &Position) -> bool {
    let mut visited = HashSet::<(Position, Position)>::new();
    let mut guard = guard.clone();
    loop {
        if let MapElement::Guard(dir) = map[guard.y][guard.x] {
            let next = next_position(&map, guard, dir);
            if next.unwrap().is_none() {
                // Finished
                return true;
            }

            // Prevent loops
            if visited.contains(&(guard, next.unwrap().unwrap().0)) {
                return false;
            }
            visited.insert((guard, next.unwrap().unwrap().0));

            // Move guard
            let (pos, dir) = next.unwrap().unwrap();
            map[guard.y][guard.x] = MapElement::Empty;
            map[pos.y][pos.x] = MapElement::Guard(dir);
            guard = pos;
        }
    }
}

fn next_position(
    map: &Map,
    guard: Position,
    dir: Direction,
) -> Option<Option<(Position, Direction)>> {
    for i in [
        0.,
        std::f64::consts::PI / 2.,
        std::f64::consts::PI,
        std::f64::consts::PI * 3. / 2.,
        std::f64::consts::PI * 2.,
    ] {
        let dir = dir - i;
        let (x, y) = (
            ((guard.x as f64) + 1. * dir.cos()).round(),
            ((guard.y as f64) - 1. * dir.sin()).round(),
        );

        // Check if it's outside
        if x < 0. || y < 0. || y as usize >= map.len() || x as usize >= map[y as usize].len() {
            return Some(None);
        }

        // Check if wall
        let (x, y) = (x as usize, y as usize);
        if let MapElement::Empty = map[y][x] {
            return Some(Some((Position { x: x, y: y }, dir)));
        }
    }
    return None;
}

fn find_guard(map: &Map) -> Position {
    let current = map
        .iter()
        .map(|x| {
            x.iter().position(|y| {
                if let MapElement::Guard(_) = y {
                    true
                } else {
                    false
                }
            })
        })
        .enumerate()
        .filter(|(_, found)| found.is_some())
        .map(|(y, x)| (y, x.unwrap()))
        .collect::<Vec<_>>();
    let current = current.first().unwrap();

    return Position {
        x: current.1,
        y: current.0,
    };
}
