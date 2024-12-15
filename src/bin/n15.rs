const INPUT: &str = include_str!("input/n15.input");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Node {
    Wall,
    Empty,
    Box,
    Robot,

    // P2
    BoxLeft,
    BoxRight,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Pos {
    x: isize,
    y: isize,
}

impl Pos {
    fn add(&self, dir: &Dir) -> Pos {
        return Pos {
            x: self.x + dir.x,
            y: self.y + dir.y,
        };
    }
}

type Dir = Pos;
type Map = Vec<Vec<Node>>;

const UP: Dir = Dir { x: 0, y: -1 };
const DOWN: Dir = Dir { x: 0, y: 1 };
const LEFT: Dir = Dir { x: -1, y: 0 };
const RIGHT: Dir = Dir { x: 1, y: 0 };

fn input() -> (Vec<Vec<Node>>, Vec<Dir>) {
    let mut map = Vec::new();
    let mut directions = Vec::new();

    // Process map
    for line in INPUT.trim().split("\r\n\r\n").next().unwrap().lines() {
        let mut row = Vec::new();
        for node in line.chars() {
            match node {
                '#' => row.push(Node::Wall),
                'O' => row.push(Node::Box),
                '@' => row.push(Node::Robot),
                '.' => row.push(Node::Empty),
                _ => unreachable!("Invalid char {:?}", node),
            }
        }
        map.push(row);
    }

    // Process directions
    for line in INPUT.trim().split("\r\n\r\n").skip(1) {
        for mov in line.chars() {
            match mov {
                '<' => directions.push(LEFT),
                '>' => directions.push(RIGHT),
                '^' => directions.push(UP),
                'v' => directions.push(DOWN),
                '\r' => {}
                '\n' => {}
                _ => unreachable!("Invalid char {:?}", mov),
            }
        }
    }

    return (map, directions);
}

fn find_robot(map: &Map) -> Pos {
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == Node::Robot {
                return Pos {
                    x: x as isize,
                    y: y as isize,
                };
            }
        }
    }
    unreachable!();
}

fn main() {
    p1();
    p2();
}

fn p1() {
    let (mut map, dirs) = input();
    execute(&mut map, dirs);
    println!("Result1: {:?}", gpssum(&map));
}

fn execute(map: &mut Map, dirs: Vec<Dir>) {
    let mut robot = find_robot(map);
    for dir in dirs {
        robot = push_element_v2(map, dir, robot);
    }
}

// fn push_element(map: &mut Map, dir: Dir, pos: Pos) -> Pos {
//     // Walls don't move (hopefully)
//     if map[pos.y as usize][pos.x as usize] == Node::Wall {
//         return pos;
//     }
//     let target = Pos {
//         x: pos.x + dir.x,
//         y: pos.y + dir.y,
//     };

//     // If there is an element in that direction, push that element
//     if map[target.y as usize][target.x as usize] != Node::Empty {
//         let _ = push_element(map, dir, target);
//     }

//     // If there is still an element there, we can't move
//     if map[target.y as usize][target.x as usize] != Node::Empty {
//         return pos;
//     }

//     // Push ourselved
//     map[target.y as usize][target.x as usize] = map[(pos.y) as usize][(pos.x) as usize];
//     map[(pos.y) as usize][(pos.x) as usize] = Node::Empty;

//     return target;
// }

fn gpssum(map: &Map) -> usize {
    let mut sum = 0;
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == Node::Box || map[y][x] == Node::BoxLeft {
                sum += 100 * y + x;
            }
        }
    }

    return sum;
}

fn p2() {
    let (mut map, dirs) = input();
    map = widen(&map);

    execute(&mut map, dirs);
    println!("Result2: {:?}", gpssum(&map));
}

fn widen(map: &Map) -> Map {
    let mut result = Vec::new();
    for y in 0..map.len() {
        let mut row = Vec::new();
        for x in 0..map[y].len() {
            match map[y][x] {
                Node::Wall => {
                    row.push(Node::Wall);
                    row.push(Node::Wall);
                }
                Node::Box => {
                    row.push(Node::BoxLeft);
                    row.push(Node::BoxRight);
                }
                Node::Empty => {
                    row.push(Node::Empty);
                    row.push(Node::Empty);
                }
                Node::Robot => {
                    row.push(Node::Robot);
                    row.push(Node::Empty);
                }
                _ => unreachable!(),
            }
        }

        result.push(row);
    }

    return result;
}

fn push_element_v2(map: &mut Map, dir: Dir, pos: Pos) -> Pos {
    // Walls don't move (hopefully)
    if map[pos.y as usize][pos.x as usize] == Node::Wall {
        return pos;
    }

    let left = sided_pos(map, pos, false);
    let right = sided_pos(map, pos, true);

    let target_left = left.add(&dir);
    let target_right = right.add(&dir);

    if !can_push_element(&map, dir, target_left) || !can_push_element(&map, dir, target_right) {
        return left;
    }

    for target in [target_left, target_right] {
        // If self, ignore
        if target == left || target == right {
            continue;
        }

        // If there is an element in that direction, push that element
        if map[target.y as usize][target.x as usize] != Node::Empty {
            let _ = push_element_v2(map, dir, target);
        }

        // If there is still an element there, we can't move
        if map[target.y as usize][target.x as usize] != Node::Empty {
            unreachable!()
        }
    }

    // Push ourselved
    let tmpl = map[left.y as usize][left.x as usize];
    let tmpr = map[right.y as usize][right.x as usize];
    map[left.y as usize][left.x as usize] = Node::Empty; // Wipe first to make sure left/right moves work
    map[right.y as usize][right.x as usize] = Node::Empty;
    map[target_left.y as usize][target_left.x as usize] = tmpl;
    map[target_right.y as usize][target_right.x as usize] = tmpr;

    return target_left;
}

fn can_push_element(map: &Map, dir: Dir, pos: Pos) -> bool {
    // Empty spaces are ignore
    if map[pos.y as usize][pos.x as usize] == Node::Empty {
        return true;
    }

    // Walls don't move (hopefully)
    if map[pos.y as usize][pos.x as usize] == Node::Wall {
        return false;
    }

    let left = sided_pos(map, pos, false);
    let right = sided_pos(map, pos, true);

    let target_left = left.add(&dir);
    let target_right = right.add(&dir);

    for target in [target_left, target_right] {
        // If self, ignore
        if target == left || target == right {
            continue;
        }

        // If there is an element in that direction, push that element
        if map[target.y as usize][target.x as usize] != Node::Empty {
            if !can_push_element(map, dir, target) {
                return false;
            }
        }
    }

    return true;
}

fn sided_pos(map: &Map, pos: Pos, right: bool) -> Pos {
    match map[pos.y as usize][pos.x as usize] {
        Node::Box => return pos,
        Node::Empty => return pos,
        Node::BoxLeft => {
            return Pos {
                x: pos.x + right.then(|| 1).unwrap_or_else(|| 0),
                y: pos.y,
            }
        }
        Node::BoxRight => {
            return Pos {
                x: pos.x + right.then(|| 0).unwrap_or_else(|| -1),
                y: pos.y,
            }
        }
        Node::Robot => return pos,
        Node::Wall => return pos,
    }
}
