use std::ops::Mul;

const INPUT: &str = include_str!("input/n9.input");

fn main() {
    p1();
    p2();
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Disk {
    None,
    File(u32),
}

fn input() -> Vec<Disk> {
    let mut is_file = true;
    let mut file_id = 0;
    let mut result = Vec::new();
    for el in INPUT.trim().chars() {
        let num = el.to_digit(10).unwrap();
        for _ in 0..num {
            match is_file {
                true => result.push(Disk::File(file_id)),
                false => result.push(Disk::None),
            }
        }
        is_file = !is_file;
        if is_file {
            file_id += 1;
        }
    }

    return result;
}

fn p1() {
    let mut input = input();
    compress_fragmented(&mut input);
    println!("Result1: {:?}", checksum(&input));
}

fn p2() {
    let mut input = input();
    compress_continuous(&mut input);
    println!("Result2: {:?}", checksum(&input));
}

fn compress_fragmented(input: &mut Vec<Disk>) {
    let mut current_free = 0;
    let mut current_used = input.len();
    while let Some(next_free) = next_free(input, current_free) {
        let next_used = next_used(input, current_used);
        if next_used.is_none() {
            break;
        }
        let next_used = next_used.unwrap();
        if next_free >= next_used {
            break;
        }

        input[next_free] = input[next_used];
        input[next_used] = Disk::None;

        // next iter
        current_free = next_free;
        current_used = next_used;
    }
}

fn compress_continuous(input: &mut Vec<Disk>) {
    let mut current_block = input.len() - 1;
    while let Some(next_block) = next_block(input, current_block) {
        if next_block.position == 0 {
            break;
        }

        // Next iter
        current_block = next_block.position - 1;

        // select free spot
        let free_spot = match lookup_free(input, next_block.size) {
            None => continue,
            Some(pos) => pos,
        };

        if free_spot >= next_block.position {
            continue;
        }

        // Move block
        let file_id = match input[next_block.position] {
            Disk::File(id) => id,
            _ => unreachable!(),
        };

        for i in 0..next_block.size {
            input[free_spot + i] = Disk::File(file_id);
            input[next_block.position + i] = Disk::None;
        }
    }
}

fn lookup_free(input: &Vec<Disk>, size: usize) -> Option<usize> {
    let mut free_size: usize = 0;
    for i in 0..input.len() {
        match input[i] {
            Disk::None => free_size += 1,
            _ => free_size = 0,
        }

        if free_size == size {
            return Some(i - free_size + 1);
        }
    }

    return None;
}

struct NextBlock {
    position: usize,
    size: usize,
}

fn next_block(input: &Vec<Disk>, from: usize) -> Option<NextBlock> {
    let mut has_block = None;
    let mut size = 0;
    for i in (0..from + 1).rev() {
        if input[i] == Disk::None && has_block.is_none() {
            continue;
        }

        match input[i] {
            Disk::None => {
                return Some(NextBlock {
                    position: i + 1,
                    size: size,
                })
            }
            Disk::File(id) => {
                if has_block == None {
                    size += 1;
                    has_block = Some(id);
                } else if has_block.unwrap() == id {
                    size += 1;
                } else {
                    return Some(NextBlock {
                        position: i + 1,
                        size: size,
                    });
                }
            }
        }
    }

    if has_block.is_some() {
        return Some(NextBlock {
            position: 0,
            size: size,
        });
    }

    return None;
}

fn checksum(input: &Vec<Disk>) -> usize {
    return input
        .iter()
        .enumerate()
        .filter(|(_, x)| **x != Disk::None)
        .map(|(pos, x)| {
            if let Disk::File(id) = x {
                (pos, id)
            } else {
                unreachable!()
            }
        })
        .map(|(pos, id)| pos.mul(*id as usize))
        .reduce(|acc, e| acc + e)
        .unwrap();
}

fn next_free(input: &Vec<Disk>, current: usize) -> Option<usize> {
    for i in current..input.len() {
        if input[i] == Disk::None {
            return Some(i);
        }
    }

    return None;
}

fn next_used(input: &Vec<Disk>, current: usize) -> Option<usize> {
    for i in (0..current).rev() {
        if input[i] != Disk::None {
            return Some(i);
        }
    }

    return None;
}
