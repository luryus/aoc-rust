use std::io;

fn run<const PART2: bool>(input: &[String]) -> usize {
    input
        .iter()
        .map(|l| aoclib::read_ints_from_string(l, false))
        .filter_map(|l| calc::<PART2>(&l))
        .sum()
}

fn calc<const CONCAT: bool>(l: &[usize]) -> Option<usize> {
    let res = l[0];
    let operands = &l[1..];

    return check::<CONCAT>(res, operands[0], &operands[1..]).then_some(res);

    fn check<const CONCAT: bool>(target: usize, curr: usize, operands_remaining: &[usize]) -> bool {
        if curr > target {
            return false;
        }

        if operands_remaining.is_empty() {
            return curr == target;
        }

        check::<CONCAT>(
            target,
            curr * operands_remaining[0],
            &operands_remaining[1..],
        ) || check::<CONCAT>(
            target,
            curr + operands_remaining[0],
            &operands_remaining[1..],
        ) || (CONCAT && {
            let right = operands_remaining[0];
            let scale = (right as f32).log10().floor() as u32 + 1;
            let new_curr = curr * 10usize.pow(scale) + right;
            check::<CONCAT>(target, new_curr, &operands_remaining[1..])
        })
    }
}

fn main() -> io::Result<()> {
    let input = aoclib::read_input_lines()?;

    let p1 = run::<false>(&input);
    println!("Part 1: {}", p1);

    let p2 = run::<true>(&input);
    println!("Part 2: {}", p2);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_real_input() {
        let input = aoclib::read_file_lines(aoclib::get_test_input_file!(7)).unwrap();

        let p1 = run::<false>(&input);
        assert_eq!(p1, 303876485655);

        let p2 = run::<true>(&input);
        assert_eq!(p2, 146111650210682);
    }
}
