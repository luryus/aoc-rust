use std::io;

fn part1(input: &[String]) -> usize {
    input
        .iter()
        .map(|l| aoclib::read_ints_from_string::<usize>(l, false))
        .filter(|nums| nums.len() == 2)
        .map(|nums| nums[0] * nums[1])
        .sum()
}

fn part2(input: &[String]) -> usize {
    input
        .iter()
        .scan(true, |acc, l| {
            if l.starts_with("don't") {
                *acc = false;
                Some(None)
            } else if l.starts_with("do") {
                *acc = true;
                Some(None)
            } else if *acc {
                Some(Some(l))
            } else {
                Some(None)
            }
        })
        .flatten()
        .map(|l| aoclib::read_ints_from_string::<usize>(l, false))
        .map(|nums| nums[0] * nums[1])
        .sum()
}

fn main() -> io::Result<()> {
    let input = aoclib::read_regex_matches_from_stdin(r"(mul\(\d{1,3},\d{1,3}\)|do(n't)?\(\))")?;

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
        let input = aoclib::read_regex_matches_from_file(
            aoclib::get_test_input_file!(3),
            r"(mul\(\d{1,3},\d{1,3}\)|do(n't)?\(\))",
        )
        .unwrap();

        let p1 = part1(&input);
        assert_eq!(p1, 167650499);

        let p2 = part2(&input);
        assert_eq!(p2, 95846796);
    }
}
