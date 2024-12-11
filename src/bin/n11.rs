use std::collections::HashMap;

const INPUT: &str = include_str!("input/n11.input");

type Line = Vec<u64>;

fn main() {
    p1();
    p2();
}

fn input() -> Line {
    return INPUT
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
}

fn p1() {
    let mut result = input();
    for _ in 0..25 {
        result = blink(&result);
    }

    println!("Result1: {}", result.len())
}

fn p2() {
    let input = input();
    let mut memo = HashMap::new();
    let mut result = 0;
    for x in input.iter() {
        result += blink_and_count(*x, 75, &mut memo);
    }

    println!("Result2: {}", result)
}

fn blink(line: &Line) -> Line {
    let mut result = Vec::with_capacity(line.len());
    for rock in line.iter() {
        if *rock == 0 {
            result.push(1);
        } else if *rock >= 10 && rock.ilog10() % 2 == 1 {
            let (a, b) = split_digits(*rock);
            result.push(a);
            result.push(b);
        } else {
            result.push(rock * 2024);
        }
    }

    return result;
}

fn blink_and_count(rock: u64, depth: u32, memo: &mut HashMap<(u64, u32), u64>) -> u64 {
    if let Some(r) = memo.get(&(rock, depth)) {
        // println!("Match {}", r);
        return *r;
    }

    if depth == 0 {
        return 1;
    }

    if rock == 0 {
        let r = blink_and_count(1, depth - 1, memo);
        memo.insert((rock, depth), r);
        return r;
    }
    if rock >= 10 && rock.ilog10() % 2 == 1 {
        let (a, b) = split_digits(rock);
        let r = blink_and_count(a, depth - 1, memo) + blink_and_count(b, depth - 1, memo);

        memo.insert((rock, depth), r);
        return r;
    }

    let r = blink_and_count(rock * 2024, depth - 1, memo);
    memo.insert((rock, depth), r);
    return r;
}

fn split_digits(x: u64) -> (u64, u64) {
    let digit_count = x.ilog10() + 1;
    let multiplier = 10u64.pow(digit_count / 2);
    return (x / multiplier, x % multiplier);
}
