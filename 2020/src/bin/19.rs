use itertools::Itertools;
use regex::Regex;
use std::{collections::HashMap, io};

fn part1(input: &str) -> usize {
    let (rules, strings) = aoc2020::split_to_tuple2(input, "\n\n").unwrap();

    let raw: HashMap<usize, &str> = rules
        .lines()
        .map(|l| aoc2020::split_to_tuple2(l, ": ").unwrap())
        .map(|(idx, r)| (idx.parse().unwrap(), r))
        .collect();

    let rule0 = format!("^{}$", get_rule(&raw, 0));

    let reg = Regex::new(&rule0).unwrap();

    strings.lines().filter(|&l| reg.is_match(l)).count()
}

fn part2(input: &str) -> usize {
    let (rules, strings) = aoc2020::split_to_tuple2(input, "\n\n").unwrap();

    let mut raw: HashMap<usize, &str> = rules
        .lines()
        .map(|l| aoc2020::split_to_tuple2(l, ": ").unwrap())
        .map(|(idx, r)| (idx.parse().unwrap(), r))
        .collect();

    raw.insert(8, "42 | 42 8");
    raw.insert(11, "42 31 | 42 11 31");

    let rule0 = format!("^{}$", get_rule2(&raw, 0, 30).unwrap());

    let reg = pcre2::bytes::Regex::new(&rule0).unwrap();

    strings
        .lines()
        .filter(|&l| reg.is_match(&l.bytes().collect_vec()).unwrap())
        .count()
    // let rule42 = get_rule2(&raw, 42, 30).unwrap();
    // let rule31 = get_rule2(&raw, 31, 30).unwrap();

    // let comb_re = Regex::new(&format!("{}{}", rule42, rule31)).unwrap();
    // let repeat42_re = Regex::new(&format!("({})+", rule42)).unwrap();
    // let repeat31_re = Regex::new(&format!("({})+", rule31)).unwrap();

    // dbg!(&comb_re, &repeat42_re, &repeat31_re);

    // strings.lines().filter(|&l| {
    //     if reg.is_match(l) {
    //         if comb_re.is_match(l) {
    //             for m42 in repeat42_re.find_iter(l) {
    //                 return false
    //             }
    //         }
    //         return true
    //     }
    //     false
    //  }).count()
}

fn get_rule(raw_rules: &HashMap<usize, &str>, idx: usize) -> String {
    let raw = raw_rules[&idx];
    if raw.contains("\"") {
        return raw.replace("\"", "");
    }

    let rule = raw
        .split(" ")
        .map(|p| {
            if let Ok(x) = p.parse::<usize>() {
                get_rule(raw_rules, x)
            } else if p == "|" {
                "|".to_owned()
            } else {
                panic!()
            }
        })
        .join("");

    if rule.contains("|") {
        return format!("({})", rule);
    }

    rule
}

fn get_rule2(raw_rules: &HashMap<usize, &str>, idx: usize, stack_size: usize) -> Option<String> {
    if stack_size == 0 {
        return None;
    }

    // Special handling for part 2
    if idx == 8 {
        let rule42 = get_rule2(raw_rules, 42, stack_size - 1).unwrap();
        return Some(format!("({})+", rule42));
    }

    if idx == 11 {
        let rule42 = get_rule2(raw_rules, 42, stack_size - 1).unwrap();
        let rule31 = get_rule2(raw_rules, 31, stack_size - 1).unwrap();
        return Some(format!("(?<foo>({})(?&foo)?({}))", rule42, rule31));
    }

    let raw = raw_rules[&idx];
    if raw.contains("\"") {
        return Some(raw.replace("\"", ""));
    }

    let rule = raw
        .split("|")
        .filter_map(|p| {
            let rs = p
                .trim()
                .split(" ")
                .map(|r| get_rule2(raw_rules, r.parse().unwrap(), stack_size - 1))
                .collect_vec();
            if rs.iter().any(|x| x == &None) {
                None
            } else {
                Some(rs.into_iter().map(|x| x.unwrap()).join(""))
            }
        })
        .collect_vec();
    let orred = rule.len() > 1;

    let rule = rule.join("|");

    if orred {
        return Some(format!("({})", rule));
    }

    Some(rule)
}

fn main() -> io::Result<()> {
    let input = aoc2020::read_stdin_to_string()?;

    let p1 = part1(&input);
    println!("Part 1: {}", p1);

    let p2 = part2(&input);
    println!("Part 2: {}", p2);

    Ok(())
}
