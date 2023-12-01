use std::{collections::HashMap, io};

fn part1(input: &HashMap<String, Monkey>) -> usize {
    get_monkey_val(&"root".to_string(), input)
}

fn part2(input: &HashMap<String, Monkey>) -> usize {
    fn walk<'a>(
        path: &[&'a str],
        current: &'a String,
        input: &'a HashMap<String, Monkey>,
    ) -> Option<Vec<&'a str>> {
        let m = input.get(current).unwrap();
        let mut path = path.to_owned();
        path.push(current);

        if current == "humn" {
            Some(path)
        } else {
            match m {
                Monkey::Plus(l, r)
                | Monkey::Div(l, r)
                | Monkey::Minus(l, r)
                | Monkey::Times(l, r) => walk(&path, l, input).or_else(|| walk(&path, r, input)),
                _ => None,
            }
        }
    }

    fn find_res(
        path: &[&str],
        current: &String,
        expected: usize,
        input: &HashMap<String, Monkey>,
    ) -> usize {
        let m = &input[current];
        if current == "humn" {
            return expected;
        }
        match m {
            Monkey::Literal(lit) => *lit,
            Monkey::Plus(l, r) => {
                if l == path[1] {
                    let exp = expected - get_monkey_val(r, input);
                    find_res(&path[1..], l, exp, input)
                } else {
                    let exp = expected - get_monkey_val(l, input);
                    find_res(&path[1..], r, exp, input)
                }
            }
            Monkey::Minus(l, r) => {
                if l == path[1] {
                    let exp = expected + get_monkey_val(r, input);
                    find_res(&path[1..], l, exp, input)
                } else {
                    let exp = get_monkey_val(l, input) - expected;
                    find_res(&path[1..], r, exp, input)
                }
            }
            Monkey::Times(l, r) => {
                if l == path[1] {
                    let exp = expected / get_monkey_val(r, input);
                    find_res(&path[1..], l, exp, input)
                } else {
                    let exp = expected / get_monkey_val(l, input);
                    find_res(&path[1..], r, exp, input)
                }
            }
            Monkey::Div(l, r) => {
                if l == path[1] {
                    let exp = expected * get_monkey_val(r, input);
                    find_res(&path[1..], l, exp, input)
                } else {
                    let exp = get_monkey_val(l, input) / expected;
                    find_res(&path[1..], r, exp, input)
                }
            }
        }
    }

    let rootname = "root".to_string();
    let human_path = walk(&[], &rootname, input).unwrap();
    let root = input.get(&"root".to_string()).unwrap();

    let known_side_res = match root {
        Monkey::Plus(l, r) | Monkey::Div(l, r) | Monkey::Minus(l, r) | Monkey::Times(l, r) => {
            if human_path[1] == l {
                get_monkey_val(r, input)
            } else {
                get_monkey_val(l, input)
            }
        }
        Monkey::Literal(_) => panic!(),
    };

    find_res(
        &human_path[1..],
        &human_path[1].to_string(),
        known_side_res,
        input,
    )
}

#[derive(Debug)]
enum Monkey {
    Literal(usize),
    Plus(String, String),
    Minus(String, String),
    Times(String, String),
    Div(String, String),
}

fn get_monkey_val(name: &String, monkeys: &HashMap<String, Monkey>) -> usize {
    let m = monkeys.get(name).unwrap();
    match m {
        Monkey::Literal(l) => *l,
        Monkey::Plus(l, r) => get_monkey_val(l, monkeys) + get_monkey_val(r, monkeys),
        Monkey::Minus(l, r) => get_monkey_val(l, monkeys) - get_monkey_val(r, monkeys),
        Monkey::Times(l, r) => get_monkey_val(l, monkeys) * get_monkey_val(r, monkeys),
        Monkey::Div(l, r) => get_monkey_val(l, monkeys) / get_monkey_val(r, monkeys),
    }
}

fn parse_input(lines: Vec<String>) -> HashMap<String, Monkey> {
    let re = regex::Regex::new(
        r"(?P<id>\w+): ((?P<lit>\d+)|(?P<op1>\w+) (?P<op>[\+\-\*/]) (?P<op2>\w+))",
    )
    .unwrap();
    lines
        .into_iter()
        .map(|l| {
            let caps = re.captures(&l).unwrap();
            let id = caps.name("id").unwrap().as_str().to_string();
            if let Some(lit) = caps.name("lit") {
                (id, Monkey::Literal(lit.as_str().parse().unwrap()))
            } else {
                let op = caps.name("op").unwrap().as_str();
                let op1 = caps.name("op1").unwrap().as_str().to_string();
                let op2 = caps.name("op2").unwrap().as_str().to_string();
                (
                    id,
                    match op.chars().next().unwrap() {
                        '+' => Monkey::Plus(op1, op2),
                        '*' => Monkey::Times(op1, op2),
                        '-' => Monkey::Minus(op1, op2),
                        '/' => Monkey::Div(op1, op2),
                        op => panic!("Unknown operator {op}"),
                    },
                )
            }
        })
        .collect()
}

fn main() -> io::Result<()> {
    let input = parse_input(aoclib::read_input_lines()?);

    let p1 = part1(&input);
    println!("Part 1: {}", p1);

    let p2 = part2(&input);
    println!("Part 2: {}", p2);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_real_input() {
        let input = aoclib::read_file_lines(aoclib::get_test_input_file!(21)).unwrap();
        let input = parse_input(input);

        let p1 = part1(&input);
        assert_eq!(p1, 155708040358220);

        let p2 = part2(&input);
        assert_eq!(p2, 3342154812537);
    }
}
