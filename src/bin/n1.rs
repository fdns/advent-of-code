use std::collections::hash_map::Entry::*;
use std::collections::HashMap;

const INPUT: &str = include_str!("input/n1.input");

fn main() {
    p1();
    p2();
}

fn p2() {
    let mut left = Vec::<u32>::new();
    let mut right = HashMap::<u32, u32>::new();
    for s in INPUT.split("\n") {
        let vars = s.split_whitespace().collect::<Vec<&str>>();
        left.push(vars[0].parse().unwrap());

        match right.entry(vars[1].parse().unwrap()) {
            Occupied(mut x) => {
                x.insert(x.get() + 1);
            }
            Vacant(x) => {
                x.insert(1);
            }
        };
    }

    let mut simi = 0;
    for el in left {
        simi += el * right.get(&el).unwrap_or(&0);
    }

    println!("Similarity: {}", simi);
}

fn p1() {
    let mut left = Vec::<u32>::new();
    let mut right = Vec::<u32>::new();
    for s in INPUT.split("\n") {
        let vars = s.split_whitespace().collect::<Vec<&str>>();
        left.push(vars[0].parse().unwrap());
        right.push(vars[1].parse().unwrap());
    }

    left.sort();
    right.sort();

    let mut distance = 0;
    for i in 0..left.len() {
        distance += left[i].abs_diff(right[i]);
    }

    println!("Distance: {}", distance)
}
