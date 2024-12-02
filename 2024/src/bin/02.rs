use itertools::Itertools;
use std::io;

fn is_safe(report: &[u8]) -> bool {
    fn check<'a>(it: impl Iterator<Item = &'a u8>) -> bool {
        it.tuple_windows()
            .all(|(a, b)| b > a && (b - a) >= 1 && (b - a) <= 3)
    }

    let rising = report.first().unwrap() < report.last().unwrap();
    if rising {
        check(report.iter())
    } else {
        check(report.iter().rev())
    }
}

fn part1(input: &[String]) -> usize {
    input
        .iter()
        .map(|l| aoclib::read_ints_from_string(l, false))
        .filter(|r| is_safe(r))
        .count()
}

fn part2(input: &[String]) -> usize {
    input
        .iter()
        .map(|l| aoclib::read_ints_from_string(l, false))
        .filter(|r| {
            if is_safe(r) {
                return true;
            }

            let mut rr = r.clone();
            for i in 0..r.len() {
                let x = rr.remove(i);
                if is_safe(&rr) {
                    return true;
                }
                rr.insert(i, x);
            }

            false
        })
        .count()
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
        let input = aoclib::read_file_lines(aoclib::get_test_input_file!(2)).unwrap();

        let p1 = part1(&input);
        assert_eq!(p1, 230);

        let p2 = part2(&input);
        assert_eq!(p2, 301);
    }
}
