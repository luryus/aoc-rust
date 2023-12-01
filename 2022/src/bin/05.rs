use itertools::Itertools;
use std::io;

#[repr(transparent)]
#[derive(Clone, Copy)]
struct Crate(char);
type Stack = Vec<Crate>;
struct Step(usize, usize, usize);

fn parse_input(input: &[String]) -> (Vec<Stack>, Vec<Step>) {
    let rows: Vec<_> = input
        .iter()
        .take_while(|l| !l.is_empty())
        .map(|l| {
            let chunks = l.chars().chunks(4);
            chunks
                .into_iter()
                .map(|c| {
                    let (s, cr) = c.take(2).collect_tuple().unwrap();
                    match s {
                        '[' => Some(Crate(cr)),
                        _ => None,
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect();

    let mut stacks: Vec<Stack> = vec![vec![]; rows[0].len()];

    for r in rows.into_iter().rev().skip(1) {
        for (i, cr) in r.into_iter().enumerate() {
            if let Some(cc) = cr {
                stacks[i].push(cc);
            }
        }
    }

    let steps = input
        .iter()
        .skip_while(|l| !l.is_empty())
        .skip(1)
        .map(|l| aoclib::read_ints_from_string(l, false))
        .map(|l| Step(l[0], l[1], l[2]))
        .collect();

    (stacks, steps)
}

fn part1(input: &[String]) -> String {
    let (mut stacks, steps) = parse_input(input);

    for step in steps {
        let Step(count, from, to) = step;
        for _ in 0..count {
            let cr = stacks[from - 1].pop().expect("Nothing to pop in a stack!");
            stacks[to - 1].push(cr);
        }
    }

    stacks
        .into_iter()
        .map(|s| s.last().expect("Stack was empty").0)
        .collect()
}

fn part2(input: &[String]) -> String {
    let (mut stacks, steps) = parse_input(input);

    for step in steps {
        let Step(count, from, to) = step;
        let from_stack = &mut stacks[from - 1];
        let mov = from_stack.split_off(from_stack.len() - count);
        stacks[to - 1].extend(mov);
    }

    stacks
        .into_iter()
        .map(|s| s.last().expect("Stack was empty").0)
        .collect()
}

fn main() -> io::Result<()> {
    let input = aoclib::read_input_lines()?;

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
        let input = aoclib::read_file_lines(aoclib::get_test_input_file!(5)).unwrap();

        let p1 = part1(&input);
        assert_eq!(p1, "TPGVQPFDH");

        let p2 = part2(&input);
        assert_eq!(p2, "DMRDFRHHH");
    }
}
