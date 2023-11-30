use std::io;

fn parse_expr(input: &str, part2: bool) -> Vec<char> {
    let mut output = Vec::new();
    let mut operators = Vec::new();

    for c in input.chars() {
        match c {
            ' ' => continue,
            '+' => {
                while !operators.is_empty()
                    && operators.last() != Some(&'(')
                    && (!part2 || operators.last() == Some(&'+'))
                {
                    output.push(operators.pop().unwrap());
                }
                operators.push(c);
            }
            '*' => {
                while !operators.is_empty()
                    && operators.last() != Some(&'(')
                    && (!part2 || operators.last() == Some(&'+'))
                {
                    output.push(operators.pop().unwrap());
                }
                operators.push(c);
            }
            '(' => operators.push('('),
            ')' => {
                while let Some(op) = operators.pop() {
                    if op != '(' {
                        output.push(op);
                    } else {
                        break;
                    }
                }
            }
            _ => output.push(c),
        }
    }
    while let Some(op) = operators.pop() {
        output.push(op);
    }

    output
}

fn run(input: &Vec<String>, part2: bool) -> usize {
    input
        .iter()
        .map(|l| {
            let mut expr = parse_expr(l, part2);
            expr.reverse();

            let mut stack = Vec::new();
            while let Some(token) = expr.pop() {
                if token.is_ascii_digit() {
                    stack.push(token.to_digit(10).unwrap() as usize);
                } else {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    stack.push(match token {
                        '+' => a + b,
                        '*' => a * b,
                        _ => unreachable!(),
                    });
                }
            }
            assert!(stack.len() == 1);
            stack.pop().unwrap()
        })
        .sum()
}

fn main() -> io::Result<()> {
    let inp = aoc2020::read_stdin_lines()?;

    let p1 = run(&inp, false);
    println!("Part 1: {}", p1);

    let p2 = run(&inp, true);
    println!("Part 2: {}", p2);

    Ok(())
}

#[cfg(test)]
mod test {
    use itertools::Itertools;

    use crate::parse_expr;

    #[test]
    fn test_parse_expr() {
        let input = "1 + 2";
        let out = parse_expr(input, false).into_iter().join(" ");
        assert_eq!(out, "1 2 +");

        let input = "1 + 2 + 3";
        let out = parse_expr(input, false).into_iter().join(" ");
        assert_eq!(out, "1 2 + 3 +");

        let input = "(1 + 2) + 3";
        let out = parse_expr(input, false).into_iter().join(" ");
        assert_eq!(out, "1 2 + 3 +");

        let input = "1 + 2 * 3";
        let out = parse_expr(input, false).into_iter().join(" ");
        assert_eq!(out, "1 2 + 3 *");

        let input = "1 * 2 + 3";
        let out = parse_expr(input, false).into_iter().join(" ");
        assert_eq!(out, "1 2 * 3 +");

        let input = "1 + 2 * 3";
        let out = parse_expr(input, true).into_iter().join(" ");
        assert_eq!(out, "1 2 + 3 *");

        let input = "1 * 2 + 3";
        let out = parse_expr(input, true).into_iter().join(" ");
        assert_eq!(out, "1 2 3 + *");
    }
}
