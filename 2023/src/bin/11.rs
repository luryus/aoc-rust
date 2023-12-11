use itertools::Itertools;
use ndarray::Array2;
use std::io;

fn run(input: &Array2<char>, age: usize) -> usize {
    let expand_rows: Vec<_> = input
        .rows()
        .into_iter()
        .enumerate()
        .filter(|(_, r)| !r.iter().contains(&'#'))
        .map(|(i, _)| i)
        .collect();
    let expand_cols: Vec<_> = input
        .columns()
        .into_iter()
        .enumerate()
        .filter(|(_, r)| !r.iter().contains(&'#'))
        .map(|(i, _)| i)
        .collect();

    input
        .indexed_iter()
        .filter(|(_, c)| **c == '#')
        .map(|((y, x), _)| {
            (
                y + expand_rows.iter().take_while(|rr| **rr < y).count() * (age - 1),
                x + expand_cols.iter().take_while(|cc| **cc < x).count() * (age - 1),
            )
        })
        .tuple_combinations()
        .map(|((ay, ax), (by, bx))| ay.abs_diff(by) + ax.abs_diff(bx))
        .sum()
}

fn part1(input: &Array2<char>) -> usize {
    run(input, 2)
}

fn part2(input: &Array2<char>) -> usize {
    run(input, 1_000_000)
}

fn main() -> io::Result<()> {
    let input = aoclib::read_input_char_matrix()?;

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
        let input = aoclib::read_file_char_matrix(aoclib::get_test_input_file!(11)).unwrap();

        let p1 = part1(&input);
        assert_eq!(p1, 9957702);

        let p2 = part2(&input);
        assert_eq!(p2, 512240933238);
    }
}
