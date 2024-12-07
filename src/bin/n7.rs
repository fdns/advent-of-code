const INPUT: &str = include_str!("input/n7.input");

#[derive(Debug)]
struct Calibration {
    result: i64,
    inputs: Vec<i64>,
}

fn parse() -> Vec<Calibration> {
    let mut result = Vec::new();
    for line in INPUT.lines() {
        let sections = line.split(":").collect::<Vec<_>>();
        let inputs = sections[1]
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        result.push(Calibration {
            result: sections[0].parse().unwrap(),
            inputs: inputs,
        });
    }

    return result;
}

fn main() {
    p1();
    p2();
}

fn p1() {
    let data = parse();
    let mut result = 0;
    for line in data {
        if check_permutations(line.result, line.inputs[0], &line.inputs[1..], false) {
            result += line.result;
        }
    }
    println!("Result1: {}", result)
}

fn p2() {
    let data = parse();
    let mut result = 0;
    for line in data {
        if check_permutations(line.result, line.inputs[0], &line.inputs[1..], true) {
            result += line.result;
        }
    }
    println!("Result2: {}", result)
}

fn check_permutations(result: i64, cumulative: i64, inputs: &[i64], allow_concat: bool) -> bool {
    if result == cumulative && inputs.is_empty() {
        return true;
    }

    if inputs.is_empty() {
        return false;
    }

    return check_permutations(result, cumulative + inputs[0], &inputs[1..], allow_concat)
        || check_permutations(result, cumulative * inputs[0], &inputs[1..], allow_concat)
        || (allow_concat
            && check_permutations(
                result,
                concat(cumulative, inputs[0]),
                &inputs[1..],
                allow_concat,
            ));
}

fn concat(a: i64, b: i64) -> i64 {
    a * 10i64.pow(1 + (b as f64).log10().floor() as u32) + b
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(16442803, concat(1644280, 3));
        assert_eq!(164428039, concat(1644280, 39));
    }
}
