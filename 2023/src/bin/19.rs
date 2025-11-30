use aoclib::UnwrapOptionIterator;
use itertools::Itertools;
use std::{collections::HashMap, io};

struct Part(i32, i32, i32, i32);

impl Part {
    fn eval<'a>(&self, rule: &'a Rule) -> Option<&'a str> {
        match rule {
            Rule::Always(s) => Some(s),
            Rule::LessThan(f, cmp, s) if self.field_val(*f) < *cmp => Some(s),
            Rule::GreaterThan(f, cmp, s) if self.field_val(*f) > *cmp => Some(s),
            _ => None,
        }
    }

    fn field_val(&self, f: char) -> i32 {
        match f {
            'x' => self.0,
            'm' => self.1,
            'a' => self.2,
            's' => self.3,
            _ => panic!("Invalid field char"),
        }
    }

    fn field_mut(&mut self, f: char) -> &mut i32 {
        match f {
            'x' => &mut self.0,
            'm' => &mut self.1,
            'a' => &mut self.2,
            's' => &mut self.3,
            _ => panic!("Invalid field char"),
        }
    }

    fn sum(&self) -> i32 {
        self.0 + self.1 + self.2 + self.3
    }
}

struct Workflow<'a> {
    id: &'a str,
    rules: Vec<Rule<'a>>,
}

impl<'a> Workflow<'a> {
    fn run<'b>(&'b self, part: &'b Part) -> Option<&'b str> {
        self.rules.iter().filter_map(|r| part.eval(r)).next()
    }
}

enum Rule<'a> {
    Always(&'a str),
    LessThan(char, i32, &'a str),
    GreaterThan(char, i32, &'a str),
}

impl<'a> Rule<'a> {
    fn target(&self) -> &str {
        match self {
            Rule::Always(s) => s,
            Rule::LessThan(_, _, s) => s,
            Rule::GreaterThan(_, _, s) => s,
        }
    }
}

fn is_accepted(part: &Part, workflows: &HashMap<&str, Workflow>) -> bool {
    let mut wf_name = "in";

    while wf_name != "R" && wf_name != "A" {
        wf_name = workflows[wf_name]
            .run(part)
            .expect("Workflow did not terminate (invalid workflow)!");
    }

    wf_name == "A"
}

fn part1(workflows: &HashMap<&str, Workflow>, parts: &[Part]) -> i32 {
    parts
        .iter()
        .filter(|p| is_accepted(p, workflows))
        .map(|p| p.sum())
        .sum()
}

fn part2(workflows: &HashMap<&str, Workflow>) -> usize {
    let starts = workflows.values().flat_map(|wf| {
        wf.rules
            .iter()
            .enumerate()
            .filter(|(_, r)| r.target() == "A")
            .map(|(i, _)| (wf.id, i))
    });

    starts
        .map(|(id, pos)| {
            let mut lower = Part(1, 1, 1, 1);
            let mut upper = Part(4000, 4000, 4000, 4000);
            find_bounds(true, id, pos, workflows, &mut lower, &mut upper);

            let Part(la, lx, lm, ls) = lower;
            let Part(ua, ux, um, us) = upper;
            (ua - la + 1).max(0) as usize
                * (ux - lx + 1).max(0) as usize
                * (um - lm + 1).max(0) as usize
                * (us - ls + 1).max(0) as usize
        })
        .sum()
}

fn find_bounds(
    first: bool,
    id: &str,
    pos: usize,
    workflows: &HashMap<&str, Workflow>,
    lower: &mut Part,
    upper: &mut Part,
) {
    let wf = &workflows[id];
    let r = &wf.rules[pos];

    match r {
        Rule::LessThan(c, val, _) => {
            if first {
                let f = upper.field_mut(*c);
                *f = (*f).min(*val - 1);
            } else {
                let f = lower.field_mut(*c);
                *f = (*f).max(*val);
            }
        }
        Rule::GreaterThan(c, val, _) => {
            if first {
                let f = lower.field_mut(*c);
                *f = (*f).max(*val + 1);
            } else {
                let f = upper.field_mut(*c);
                *f = (*f).min(*val);
            }
        }
        _ => (),
    };

    if pos > 0 {
        find_bounds(false, id, pos - 1, workflows, lower, upper);
    } else if id != "in" {
        let (id, pos) = workflows
            .values()
            .flat_map(|wf| {
                wf.rules
                    .iter()
                    .enumerate()
                    .find(|(_, r)| r.target() == id)
                    .map(|(i, _)| (wf.id, i))
            })
            .next()
            .expect("No source rule found for this workflow");
        find_bounds(true, id, pos, workflows, lower, upper);
    }
}

fn parse_rule(s: &str) -> Option<Rule<'_>> {
    if let Some((l, r)) = aoclib::split_to_tuple2(s, ":") {
        let num = l[2..].parse().ok()?;
        let field = l.chars().next()?;
        if l.chars().nth(1)? == '>' {
            Some(Rule::GreaterThan(field, num, r))
        } else {
            Some(Rule::LessThan(field, num, r))
        }
    } else {
        Some(Rule::Always(s))
    }
}

fn parse_input(input: &[String]) -> (HashMap<&str, Workflow<'_>>, Vec<Part>) {
    let (rule_lines, part_lines) = input.split(|l| l.is_empty()).collect_tuple().unwrap();

    let workflows = rule_lines
        .iter()
        .map(|l| {
            let (id, rules) = aoclib::split_to_tuple2(l, "{").unwrap();
            let rules = rules[..rules.len() - 1]
                .split(',')
                .map(parse_rule)
                .unwrap_options()
                .collect();
            (id, Workflow { id, rules })
        })
        .collect();

    let parts = part_lines
        .iter()
        .map(|l| {
            let (x, m, a, s) = aoclib::read_ints_from_string(l, false)
                .into_iter()
                .collect_tuple()
                .unwrap();
            Part(x, m, a, s)
        })
        .collect();

    (workflows, parts)
}

fn main() -> io::Result<()> {
    let input = aoclib::read_input_lines()?;
    let (wfs, parts) = parse_input(&input);

    let p1 = part1(&wfs, &parts);
    println!("Part 1: {}", p1);

    let p2 = part2(&wfs);
    println!("Part 2: {}", p2);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_real_input() {
        let input = aoclib::read_file_lines(aoclib::get_test_input_file!(19)).unwrap();
        let (wfs, parts) = parse_input(&input);

        let p1 = part1(&wfs, &parts);
        assert_eq!(p1, 333263);

        let p2 = part2(&wfs);
        assert_eq!(p2, 130745440937650);
    }
}
