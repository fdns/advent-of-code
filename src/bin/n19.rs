use std::collections::HashMap;

const INPUT: &str = include_str!("input/n19.input");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Color {
    White,
    Blue,
    Black,
    Red,
    Green,
}

type Pattern = Vec<Color>;

#[derive(Debug)]
struct Input {
    available: Vec<Pattern>,
    wanted: Vec<Pattern>,
}

fn str_to_pattern(input: &str) -> Pattern {
    let mut result = Vec::new();
    for c in input.trim().chars() {
        result.push(match c {
            'w' => Color::White,
            'u' => Color::Blue,
            'b' => Color::Black,
            'r' => Color::Red,
            'g' => Color::Green,
            _ => unimplemented!("Color {} not found", c),
        })
    }
    return result;
}

fn input() -> Input {
    let mut inp = INPUT.trim().lines();

    let mut available = Vec::new();
    for entry in inp.next().unwrap().split(",") {
        available.push(str_to_pattern(entry));
    }

    let _ = inp.next().unwrap();

    let mut wanted = Vec::new();
    for entry in inp {
        wanted.push(str_to_pattern(entry));
    }

    return Input {
        available: available,
        wanted: wanted,
    };
}

fn count_has_combinations(input: &Input) -> usize {
    let mut result = 0;
    for entry in &input.wanted {
        let mut res = HashMap::new();
        combinations(&input.available, &entry, 0, &mut res);
        if !res.is_empty() {
            result += 1;
        }
    }

    return result;
}

fn count_combinations_unique(input: &Input) -> usize {
    let mut result = 0;

    for entry in &input.wanted {
        let mut patterns = HashMap::new();
        combinations(&input.available, &entry, 0, &mut patterns);
        if let Some(x) = patterns.get(&0) {
            result += x;
        }
    }

    return result;
}

fn combinations(
    towels: &Vec<Pattern>,
    expected: &[Color],
    index: usize,
    patterns: &mut HashMap<usize, usize>,
) {
    if expected.len() == index {
        patterns.insert(index, 1);
        return;
    }

    // Level already searched
    if patterns.contains_key(&index) {
        return;
    }

    for i in 0..towels.len() {
        if is_subpattern(&towels[i], &expected[index..]) {
            let next = index + towels[i].len();

            combinations(towels, expected, next, patterns);

            // Collect patterns
            if let Some(count) = patterns.get(&next) {
                *patterns.entry(index).or_default() += *count;
            }
        }
    }
}

fn is_subpattern(towel: &Pattern, expected: &[Color]) -> bool {
    if towel.len() > expected.len() {
        return false;
    }

    for i in 0..towel.len() {
        if towel[i] != expected[i] {
            return false;
        }
    }

    return true;
}

fn p1() {
    let input = input();
    println!("Result 1: {:?}", count_has_combinations(&input));
}
fn p2() {
    let input = input();
    println!("Result 2: {:?}", count_combinations_unique(&input));
}

fn main() {
    p1();
    p2();
}
