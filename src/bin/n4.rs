const INPUT: &str = include_str!("input/n4.input");

fn main() {
    p1();
    p2();
}

type Map = Vec<Vec<Code>>;

#[derive(PartialEq)]
enum Code {
    X,
    M,
    A,
    S,
}

struct Direction(isize, isize);

fn p1() {
    let map = load();
    let mut count = 0;

    for x in 0..map.len() {
        for y in 0..map[x].len() {
            for a in [-1, 0, 1] {
                for b in [-1, 0, 1] {
                    // skip not moving
                    if a == 0 && b == 0 {
                        continue;
                    }
                    if search_pos(
                        &map,
                        x as isize,
                        y as isize,
                        &[Code::X, Code::M, Code::A, Code::S],
                        &Direction(a, b),
                    ) {
                        count += 1;
                    }
                }
            }
        }
    }

    println!("Count1: {}", count);
}

fn search_pos(map: &Map, x: isize, y: isize, left: &[Code], direction: &Direction) -> bool {
    if x as usize >= map.len() || y as usize >= map[x as usize].len() {
        return false;
    }

    if map[x as usize][y as usize] != left[0] {
        return false;
    }

    if left.len() == 1 {
        return true;
    }

    let (x, y) = (x + direction.0, y + direction.1);
    if x < 0 || y < 0 {
        return false;
    }

    return search_pos(map, x, y, &left[1..], direction);
}

fn load() -> Vec<Vec<Code>> {
    INPUT
        .lines()
        .map(|line| {
            line.chars()
                .map(|x| match x {
                    'X' => Code::X,
                    'M' => Code::M,
                    'A' => Code::A,
                    'S' => Code::S,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect()
}

fn p2() {
    let map = load();
    let mut count = 0;

    for x in 0..map.len() {
        for y in 0..map[x].len() {
            if search_mas(&map, x as isize, y as isize) {
                count += 1;
            }
        }
    }

    println!("Count2: {}", count);
}

fn search_mas(map: &Map, x: isize, y: isize) -> bool {
    // Find MAS going down
    if !search_pos(map, x, y, &[Code::M, Code::A, Code::S], &Direction(1, 1))
        && !search_pos(map, x, y, &[Code::S, Code::A, Code::M], &Direction(1, 1))
    {
        return false;
    }

    // Find MAS Down two going up
    if !search_pos(
        map,
        x + 2,
        y,
        &[Code::M, Code::A, Code::S],
        &Direction(-1, 1),
    ) && !search_pos(
        map,
        x + 2,
        y,
        &[Code::S, Code::A, Code::M],
        &Direction(-1, 1),
    ) {
        return false;
    }

    return true;
}
