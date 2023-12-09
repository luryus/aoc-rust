use itertools::Itertools;
use std::io;

fn extrapolate_seq(seq: &[i64]) -> i64 {
    if seq.iter().all(|i| *i == 0) {
        return 0;
    }

    let diffs: Vec<i64> = seq.iter().tuple_windows().map(|(a, b)| b - a).collect();
    let d = extrapolate_seq(&diffs);
    seq.last().unwrap() + d
}

fn extrapolate_seq_back(seq: &[i64]) -> i64 {
    if seq.iter().all(|i| *i == 0) {
        return 0;
    }

    let diffs: Vec<i64> = seq.iter().tuple_windows().map(|(a, b)| b - a).collect();
    let d = extrapolate_seq_back(&diffs);
    seq.first().unwrap() - d
}

fn part1(input: &[Vec<i64>]) -> i64 {
    input.iter().map(|seq| extrapolate_seq(seq)).sum()
}

fn part2(input: &[Vec<i64>]) -> i64 {
    input.iter().map(|seq| extrapolate_seq_back(seq)).sum()
}

fn main() -> io::Result<()> {
    let input: Vec<Vec<i64>> = aoclib::read_input_lines()?
        .into_iter()
        .map(|l| aoclib::read_ints_from_string(&l, true))
        .collect();

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
        let input: Vec<Vec<i64>> = aoclib::read_file_lines(aoclib::get_test_input_file!(9))
            .unwrap()
            .into_iter()
            .map(|l| aoclib::read_ints_from_string(&l, true))
            .collect();

        let p1 = part1(&input);
        assert_eq!(p1, 1725987467);

        let p2 = part2(&input);
        assert_eq!(p2, 971);
    }
}
