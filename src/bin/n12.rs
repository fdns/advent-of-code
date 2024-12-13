use std::collections::HashSet;

const INPUT: &str = include_str!("input/n12.input");

type Node = char;
type Map = Vec<Vec<Node>>;
type Region = HashSet<Pos>;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Pos {
    x: usize,
    y: usize,
}

fn input() -> Map {
    return INPUT.trim().lines().map(|x| x.chars().collect()).collect();
}

fn main() {
    p1();
    p2();
}
fn p1() {
    let map = input();
    let mut regions = Vec::<Region>::new();
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            let pos = Pos { x: x, y: y };
            if !regions
                .iter()
                .fold(false, |acc, set| acc || set.contains(&pos))
            {
                let mut region = Region::new();
                build_region(&map, map[pos.y][pos.x], &pos, &mut region);
                regions.push(region);
            }
        }
    }

    let mut cost = 0;
    for region in regions {
        cost += area(&region) * perimeter(&region, &map);
    }

    println!("Result1: {}", cost);
}

fn p2() {
    let map = input();
    let mut regions = Vec::<Region>::new();
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            let pos = Pos { x: x, y: y };
            if !regions
                .iter()
                .fold(false, |acc, set| acc || set.contains(&pos))
            {
                let mut region = Region::new();
                build_region(&map, map[pos.y][pos.x], &pos, &mut region);
                regions.push(region);
            }
        }
    }

    let mut cost = 0;
    for region in regions {
        cost += area(&region) * sides(&region, &map);
    }

    println!("Result2: {}", cost);
}

fn area(region: &Region) -> usize {
    return region.len();
}

fn perimeter(region: &Region, map: &Map) -> usize {
    let mut result = 0;
    for pos in region {
        result += 4;

        let movement = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        for next in movement {
            if let Some(pos) = next_pos(&pos, map, next) {
                if region.contains(&pos) {
                    result -= 1;
                }
            }
        }
    }

    return result;
}

fn sides(region: &Region, map: &Map) -> usize {
    let mut visited = HashSet::new();
    let movement = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut result = 0;

    // For each direction
    // 1. Find if direction is visible for pos
    // 2. Find all joined pos that have this visible
    // 3. Add to visited set
    // Return count of visited set
    // fn visible(pos: &Pos, side: (a, b)) -> bool {
    //     if let Some(pos) = next_pos(&pos, map, side) {
    //         return true;
    //     }
    //     return false;
    // }

    for visibility in movement {
        for pos in region {
            // If blocked, continue
            if let Some(pos) = next_pos(&pos, map, visibility) {
                if region.contains(&pos) {
                    continue;
                }
            }

            // if already visited, continue
            let entry = (pos.clone(), visibility);
            if visited.contains(&entry) {
                continue;
            }
            visited.insert(entry);

            // Add new visibility line
            result += 1;

            // See where this can be visible, which is a 90 and - 90 rotation
            for rotation in [1, -1] {
                let direction = (visibility.1 * rotation, -visibility.0 * rotation);
                let mut current = pos.clone();
                while let Some(pos) = next_pos(&current, map, direction) {
                    if !region.contains(&pos) {
                        break;
                    }
                    // If there is a blocking view, break
                    if let Some(viewing) = next_pos(&pos, map, visibility) {
                        if region.contains(&viewing) {
                            break;
                        }
                    }
                    visited.insert((pos.clone(), visibility));
                    current = pos.clone();
                }
            }
        }
    }

    return result;
}

fn build_region(map: &Map, node: Node, pos: &Pos, result: &mut Region) {
    if result.contains(&pos) {
        return;
    }

    if map[pos.y as usize][pos.x as usize] != node {
        return;
    }

    result.insert(pos.clone());

    let movement = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    for next in movement {
        if let Some(pos) = next_pos(&pos, map, next) {
            build_region(map, node, &pos, result);
        }
    }
}

fn next_pos(pos: &Pos, map: &Map, (x, y): (i32, i32)) -> Option<Pos> {
    let (x, y) = (pos.x as isize + x as isize, pos.y as isize + y as isize);
    if x < 0 || y < 0 || y as usize >= map.len() || x as usize >= map[y as usize].len() {
        return None;
    }
    return Some(Pos {
        x: x as usize,
        y: y as usize,
    });
}
