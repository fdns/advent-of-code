const INPUT: &str = include_str!("input/n2.input");

fn main() {
    p1();
    p2();
}

fn p1() {
    let mut total_safe = 0;
    for s in INPUT.split("\n") {
        let vars = s
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        if vars.len() < 2 {
            continue;
        }

        let increasing = vars[1] > vars[0];
        let mut safe = true;
        for i in 1..vars.len() {
            let before = vars[i - 1];
            let next = vars[i];

            // The levels are either all increasing or all decreasing.
            if before == next || (increasing != (next > before)) {
                safe = false;
                break;
            }

            // Any two adjacent levels differ by at least one and at most three.
            let diff = before.abs_diff(next);
            if diff > 3 {
                safe = false;
                break;
            }
        }

        if safe {
            total_safe += 1;
        }
    }

    println!("Safe 1: {}", total_safe);
}

fn p2() {
    let mut total_safe = 0;
    for s in INPUT.split("\n") {
        let vars = s
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        if is_safe(&vars) {
            total_safe += 1;
        } else {
            // Check by removing one
            for i in 0..vars.len() {
                let mut vars = vars.clone();
                vars.remove(i);
                if is_safe(&vars) {
                    total_safe += 1;
                    break;
                }
            }
        }
    }

    println!("Safe 2: {}", total_safe);
}

fn is_safe(vars: &Vec<u32>) -> bool {
    let increasing = vars[1] > vars[0];
    for i in 1..vars.len() {
        let before = vars[i - 1];
        let next = vars[i];

        // The levels are either all increasing or all decreasing.
        if before == next || (increasing != (next > before)) {
            return false;
        }

        // Any two adjacent levels differ by at least one and at most three.
        let diff = before.abs_diff(next);
        if diff > 3 {
            return false;
        }
    }

    return true;
}
