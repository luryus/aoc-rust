use num_integer::Integer;
use std::io;

fn part1(input: &[i64]) -> usize {
    input
        .iter()
        .scan(50, |acc, x| {
            *acc = (*acc + x + 100) % 100;
            Some(*acc)
        })
        .filter(|x| *x == 0)
        .count()
}

fn part2(input: &[i64]) -> usize {
    let mut c = 0;
    let mut acc = 50i64;
    for i in input {
        let (id, ir) = i.div_rem(&100);
        let new = acc + ir;
        let res = (acc > 0 && new <= 0) || new.abs() >= 100;
        c += id.unsigned_abs() as usize + (res as usize);
        acc = (new + 100) % 100;
    }
    c
}

fn parse_input(lines: Vec<String>) -> Vec<i64> {
    lines
        .into_iter()
        .map(|l| {
            let num: i64 = l[1..].parse().unwrap();
            if l.starts_with('L') { -num } else { num }
        })
        .collect()
}

fn main() -> io::Result<()> {
    let input = aoclib::read_input_lines()?;
    let input = parse_input(input);

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
        let input = aoclib::read_file_lines(aoclib::get_test_input_file!(1)).unwrap();
        let input = parse_input(input);

        let p1 = part1(&input);
        assert_eq!(p1, 1172);

        let p2 = part2(&input);
        assert_eq!(p2, 6932);
    }
}
