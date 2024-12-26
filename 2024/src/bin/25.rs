use itertools::Itertools;
use std::io;

fn part1(input: String) -> usize {
    let parts = input.split("\n\n");
    let mut keys = vec![];
    let mut locks = vec![];
    for p in parts {
        let p = aoclib::read_string_char_matrix(p).unwrap();
        let is_key = p[(0, 0)] == '.';
        let heights = p
            .columns()
            .into_iter()
            .map(|c| c.iter().filter(|cc| **cc == '#').count() - 1)
            .collect::<Vec<_>>();
        if is_key {
            keys.push(heights);
        } else {
            locks.push(heights);
        }
    }

    keys.iter()
        .cartesian_product(locks.iter())
        .filter(|(k, l)| k.iter().zip(l.iter()).all(|(kk, ll)| kk + ll <= 5))
        .count()
}

fn main() -> io::Result<()> {
    let input = aoclib::read_input_string()?;

    let p1 = part1(input);
    println!("Part 1: {}", p1);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_real_input() {
        let input = std::fs::read_to_string(aoclib::get_test_input_file!(25)).unwrap();

        let p1 = part1(input);
        assert_eq!(p1, 3136);
    }
}
