use itertools::Itertools;
use regex::Regex;
use std::io;

fn part1((rules, _, others): &(Vec<Rule>, Vec<usize>, Vec<Vec<usize>>)) -> usize {
    others
        .into_iter()
        .flatten()
        .filter(|&&x| !rules.iter().any(|r| field_valid(r, x)))
        .sum()
}

fn field_valid(rule: &Rule, field: usize) -> bool {
    (rule.range1.0..=rule.range1.1).contains(&field)
        || (rule.range2.0..=rule.range2.1).contains(&field)
}

fn part2((rules, own, others): &(Vec<Rule>, Vec<usize>, Vec<Vec<usize>>)) -> usize {
    let others = others
        .into_iter()
        .filter(|&x| x.iter().all(|y| rules.iter().any(|r| field_valid(r, *y))))
        .collect_vec();

    let mut names = vec![None; own.len()];

    let mut pos = 0usize;
    loop {
        if names[pos] != None {
            if names.iter().all(|x| *x != None) {
                break;
            }
        }

        let mut candidates = rules
            .iter()
            .filter(|r| !names.contains(&Some(*r)))
            .collect_vec();

        for t in &others {
            candidates.retain(|&r| field_valid(r, t[pos]));
        }

        if candidates.len() == 1 {
            names[pos] = Some(candidates[0]);
        }

        pos = (pos + 1) % own.len();
    }

    names
        .into_iter()
        .enumerate()
        .filter(|(_, r)| r.unwrap().field.starts_with("departure"))
        .map(|(i, _)| own[i])
        .fold1(|a, b| a * b)
        .unwrap()
}

#[derive(Eq, PartialEq)]
struct Rule<'a> {
    field: &'a str,
    range1: (usize, usize),
    range2: (usize, usize),
}

fn parse(inp: &str) -> (Vec<Rule>, Vec<usize>, Vec<Vec<usize>>) {
    lazy_static::lazy_static! {
        static ref RULE_RE: Regex = Regex::new(r"([a-z ]+): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
    }
    let mut parts = inp.split("\n\n");

    let rules = parts
        .next()
        .unwrap()
        .lines()
        .map(|l| {
            let c = RULE_RE.captures(l).unwrap();
            Rule {
                field: c.get(1).unwrap().as_str(),
                range1: (
                    c.get(2).unwrap().as_str().parse().unwrap(),
                    c.get(3).unwrap().as_str().parse().unwrap(),
                ),
                range2: (
                    c.get(4).unwrap().as_str().parse().unwrap(),
                    c.get(5).unwrap().as_str().parse().unwrap(),
                ),
            }
        })
        .collect_vec();

    let own = aoc2020::read_ints_from_string(parts.next().unwrap().lines().nth(1).unwrap());

    let nearby = parts
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(|l| aoc2020::read_ints_from_string(l))
        .collect_vec();

    (rules, own, nearby)
}

fn main() -> io::Result<()> {
    let input = aoc2020::read_stdin_to_string()?;
    let input = parse(&input);

    let p1 = part1(&input);
    println!("Part 1: {}", p1);

    let p2 = part2(&input);
    println!("Part 2: {}", p2);

    Ok(())
}
