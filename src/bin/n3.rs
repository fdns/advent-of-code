use regex::Regex;

const INPUT: &str = include_str!("input/n3.input");

fn main() {
    p1();
    p2();
}

fn p1() {
    let mut total = 0;

    let re = Regex::new("(?<valid>mul\\([0-9]{1,3},[0-9]{1,3}\\))").unwrap();

    let result = re.captures_iter(INPUT);
    for i in result {
        let val = i.name("valid").unwrap().as_str();
        total += execute_mul(val);
    }
    println!("Total 1: {}", total);
}

fn execute_mul(mul: &str) -> i32 {
    let re = Regex::new("mul\\((?<a>[0-9]{1,3}),(?<b>[0-9]{1,3})\\)").unwrap();
    let result = re.captures(mul).unwrap();

    return result.name("a").unwrap().as_str().parse::<i32>().unwrap()
        * result.name("b").unwrap().as_str().parse::<i32>().unwrap();
}

fn p2() {
    let mut total = 0;

    let re =
        Regex::new("(?<valid>(mul\\([0-9]{1,3},[0-9]{1,3}\\))|(do\\(\\))|(don't\\(\\)))").unwrap();

    let result = re.captures_iter(INPUT);
    let mut active = true;
    for i in result {
        let val = i.name("valid").unwrap().as_str();
        let ins = parse(val);
        match ins {
            Instruction::DO => active = true,
            Instruction::DONT => active = false,
            Instruction::MUL(a, b) => {
                if active {
                    total += a * b;
                }
            }
        }
    }
    println!("Total 2: {}", total);
}

enum Instruction {
    DO,
    DONT,
    MUL(i32, i32),
}

fn parse(instruction: &str) -> Instruction {
    let re = Regex::new("(?<instruction>[a-z']+)\\((?<a>([0-9]{1,3}))?,?(?<b>([0-9]{1,3}))?\\)")
        .unwrap();
    let result = re.captures(instruction).unwrap();

    match result.name("instruction").unwrap().as_str() {
        "do" => return Instruction::DO,
        "don't" => return Instruction::DONT,
        "mul" => {
            return Instruction::MUL(
                result.name("a").unwrap().as_str().parse().unwrap(),
                result.name("b").unwrap().as_str().parse().unwrap(),
            )
        }
        _ => unreachable!(""),
    }
}
