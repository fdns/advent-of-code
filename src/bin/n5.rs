use std::cmp::Ordering;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

const INPUT: &str = include_str!("input/n5.input");

fn main() {
    p1();
    p2();
}

fn p1() {
    let (rules, total_pages) = load();
    let mut result = 0;
    for pages in total_pages {
        let mut valid = true;
        'outer: for page in &pages {
            for rule in rules.get(&page).unwrap_or(&Vec::default()) {
                // Check that all rules match in this
                if !match_rules(&pages, rule) {
                    valid = false;
                    break 'outer;
                }
            }
        }

        if valid {
            let middle = pages[pages.len() / 2];
            result += middle;
        }
    }

    println!("Result1: {}", result);
}

fn match_rules(pages: &Vec<i32>, rule: &Rule) -> bool {
    for page in pages {
        if *page == rule.page {
            return true;
        }

        if *page == rule.before {
            return false;
        }
    }
    unreachable!()
}

struct Rule {
    page: i32,
    before: i32,
}

fn load() -> (HashMap<i32, Vec<Rule>>, Vec<Vec<i32>>) {
    // X|Y = (page number)|()
    let mut rules = HashMap::new();
    let mut pages = Vec::new();
    let mut on_rules = true;
    for line in INPUT.lines() {
        if on_rules {
            if line == "" {
                on_rules = false;
                continue;
            }
            let result = line
                .split("|")
                .map(|x| x.parse().unwrap())
                .collect::<Vec<i32>>();

            let rule = Rule {
                page: result[0],
                before: result[1],
            };
            match rules.entry(rule.page) {
                Entry::Vacant(e) => {
                    e.insert(vec![rule]);
                }
                Entry::Occupied(mut e) => {
                    e.get_mut().push(rule);
                }
            };
        } else {
            let result = line.split(",").map(|x| x.parse().unwrap()).collect();
            pages.push(result);
        }
    }

    return (rules, pages);
}

fn p2() {
    let (rules, mut total_pages) = load();
    let mut result = 0;

    for pages in total_pages.iter_mut() {
        'outer: for page in &pages.clone() {
            for rule in rules.get(&page).unwrap_or(&Vec::default()) {
                // Check that all rules match in this
                if !match_rules(&pages, rule) {
                    sort(pages, &rules);

                    let middle = pages[pages.len() / 2];
                    result += middle;
                    break 'outer;
                }
            }
        }
    }

    println!("Result2: {}", result);
}

fn sort(pages: &mut Vec<i32>, rules: &HashMap<i32, Vec<Rule>>) {
    pages.sort_by(|a, b| {
        for rule in rules.get(a).unwrap_or(&Vec::default()) {
            if *b == rule.before {
                return Ordering::Less;
            }
        }

        for rule in rules.get(b).unwrap_or(&Vec::default()) {
            if *a == rule.before {
                return Ordering::Greater;
            }
        }

        return Ordering::Equal;
    });
}
