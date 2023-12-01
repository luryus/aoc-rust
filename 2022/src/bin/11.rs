use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, multispace0, space1, u32},
    combinator::{all_consuming, map, map_opt, value},
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    IResult,
};

type Item = usize;

#[derive(Clone, Debug)]
enum Operation {
    AddLiteral(usize),
    TimesLiteral(usize),
    TimesSelf,
}

#[derive(Clone, Debug)]
struct Monkey {
    items: Vec<Item>,
    operation: Operation,
    test_divisor: usize,
    true_target_monkey: usize,
    false_target_monkey: usize,
}

impl Monkey {
    fn run_op(&self, item: Item) -> Item {
        match self.operation {
            Operation::AddLiteral(val) => item + val,
            Operation::TimesLiteral(val) => item * val,
            Operation::TimesSelf => item * item,
        }
    }

    fn get_target(&self, item: Item) -> Item {
        if item % self.test_divisor == 0 {
            self.true_target_monkey
        } else {
            self.false_target_monkey
        }
    }
}

fn monkey(input: &str) -> IResult<&str, Monkey> {
    map(
        tuple((
            delimited(tag("Monkey "), u32, tuple((char(':'), multispace0))),
            delimited(
                tag("Starting items: "),
                separated_list1(tag(", "), u32),
                multispace0,
            ),
            delimited(tag("Operation: new = "), operation, multispace0),
            delimited(tag("Test: divisible by "), u32, multispace0),
            delimited(tag("If true: throw to monkey "), u32, multispace0),
            preceded(tag("If false: throw to monkey "), u32),
        )),
        |(_mi, items, operation, div, true_target, false_target)| Monkey {
            items: items.into_iter().map(|n| n as usize).collect(),
            operation,
            test_divisor: div as usize,
            true_target_monkey: true_target as usize,
            false_target_monkey: false_target as usize,
        },
    )(input)
}

fn operand(input: &str) -> IResult<&str, Option<u32>> {
    alt((value(None, tag("old")), map(u32, Some)))(input)
}

fn operation(input: &str) -> IResult<&str, Operation> {
    map_opt(
        separated_pair(
            preceded(tuple((tag("old"), space1)), alt((char('+'), char('*')))),
            space1,
            operand,
        ),
        |(operator, operand)| match (operator, operand) {
            ('+', Some(val)) => Some(Operation::AddLiteral(val as usize)),
            ('*', Some(val)) => Some(Operation::TimesLiteral(val as usize)),
            ('*', None) => Some(Operation::TimesSelf),
            _ => None,
        },
    )(input)
}

fn parse_input(input: &str) -> Vec<Monkey> {
    all_consuming(terminated(
        separated_list1(multispace0, monkey),
        multispace0,
    ))(input)
    .unwrap()
    .1
}

fn run<F: Fn(Item) -> Item>(mut monkeys: Vec<Monkey>, rounds: usize, relax: F) -> usize {
    let mut inspections = vec![0; monkeys.len()];

    for _ in 0..rounds {
        for mi in 0..monkeys.len() {
            let m = &mut monkeys[mi];
            let mut items = std::mem::take(&mut m.items);
            let moves = items
                .drain(..)
                .map(|i| {
                    let i = relax(m.run_op(i));
                    (m.get_target(i), i)
                })
                .collect_vec();

            inspections[mi] += moves.len();

            for (target, i) in moves {
                monkeys[target].items.push(i);
            }
        }
    }

    inspections.sort();
    inspections[inspections.len() - 2..].iter().product()
}

fn main() -> anyhow::Result<()> {
    let input = aoc2022::read_input_string()?;
    let input = parse_input(&input);

    let p1 = run(input.clone(), 20, |i| i / 3);
    println!("Part 1: {}", p1);

    let modulo: usize = input.iter().map(|m| m.test_divisor).product();
    let p2 = run(input, 10_000, move |i| i % modulo);
    println!("Part 2: {}", p2);

    Ok(())
}
