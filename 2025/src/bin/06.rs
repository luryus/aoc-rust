use ndarray::s;
use std::io;

fn part1(input: &[String]) -> usize {
    let operand_lists = input[..(input.len() - 1)]
        .iter()
        .map(|l| aoclib::read_ints_from_string::<usize>(l, false))
        .collect::<Vec<_>>();
    let operators = input[input.len() - 1]
        .split_whitespace()
        .collect::<Vec<_>>();

    let mut sum = 0;
    for i in 0..operators.len() {
        let it = operand_lists.iter().map(|l| l[i]);
        sum += if operators[i] == "*" {
            it.product::<usize>()
        } else {
            it.sum()
        };
    }

    sum
}

fn part2(input: &[String]) -> usize {
    let input = aoclib::read_string_char_matrix(&input.join("\n")).unwrap();
    let mut sum = 0;

    let (h, w) = input.dim();
    let mut curr_x = 0;
    let oper_row = input.row(h - 1);

    while curr_x < w {
        let mut num_w = oper_row
            .iter()
            .skip(curr_x + 1)
            .take_while(|c| **c == ' ')
            .count();
        if curr_x + num_w + 1 == w {
            num_w += 1
        }
        let oper = oper_row[curr_x];

        let rect = input.slice(s![0..h - 1, curr_x..curr_x + num_w]);
        let operands = rect.columns().into_iter().map(|c| {
            let x = c.iter().collect::<String>();
            x.trim_ascii().parse::<usize>().unwrap()
        });

        sum += if oper == '*' {
            operands.product::<usize>()
        } else {
            operands.sum()
        };

        curr_x += num_w + 1;
    }

    sum
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
        let input = aoclib::read_file_lines(aoclib::get_test_input_file!(6)).unwrap();

        let p1 = part1(&input);
        assert_eq!(p1, 6100348226985);

        let p2 = part2(&input);
        assert_eq!(p2, 12377473011151);
    }
}
