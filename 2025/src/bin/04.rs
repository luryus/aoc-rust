use itertools::Itertools;
use ndarray::{Array2, s};
use std::io;

fn remove_rolls(input: &mut Array2<char>) -> usize {
    let mut removed = 0;
    let (h, w) = input.dim();
    for (y, x) in (0..h).cartesian_product(0..w) {
        if input[(y, x)] != '@' {
            continue;
        }
        let minx = x.saturating_sub(1);
        let maxx = (x + 1).min(w - 1);
        let miny = y.saturating_sub(1);
        let maxy = (y + 1).min(h - 1);

        let window = input.slice(s![miny..=maxy, minx..=maxx]);

        let rolls = window.iter().filter(|c| **c == '@').count();
        if rolls <= 4 {
            removed += 1;
            input[(y, x)] = '.';
        }
    }

    removed
}

fn part1(input: &Array2<char>) -> usize {
    remove_rolls(&mut input.clone())
}

fn part2(input: &Array2<char>) -> usize {
    let mut input = input.clone();
    let mut count = 0;

    loop {
        let removed_count = remove_rolls(&mut input);
        count += removed_count;
        if removed_count == 0 {
            break;
        }
    }

    count
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
        let input = aoclib::read_file_char_matrix(aoclib::get_test_input_file!(4)).unwrap();

        let p1 = part1(&input);
        assert_eq!(p1, 2988);

        let p2 = part2(&input);
        assert_eq!(p2, 9122);
    }
}
