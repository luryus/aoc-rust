use itertools::Itertools;
use ndarray::Array2;
use std::io;

fn check(arr: &Array2<char>, y: usize, x: usize) -> usize {
    let dirs = (-1..=1).cartesian_product(-1..=1);

    dirs.filter(|d| {
        if *d == (0, 0) {
            return false;
        }

        let Some(ny) = y.checked_add_signed(d.0) else {
            return false;
        };
        let Some(nx) = x.checked_add_signed(d.1) else {
            return false;
        };

        check_dir(arr, ny, nx, 'M', *d)
    })
    .count()
}

fn check_dir(arr: &Array2<char>, y: usize, x: usize, state: char, dir: (isize, isize)) -> bool {
    if y >= arr.dim().0 || x >= arr.dim().1 || arr[(y, x)] != state {
        return false;
    }

    if state == 'S' {
        return true;
    }

    let next = match state {
        'M' => 'A',
        'A' => 'S',
        _ => unreachable!(),
    };

    let Some(ny) = y.checked_add_signed(dir.0) else {
        return false;
    };
    let Some(nx) = x.checked_add_signed(dir.1) else {
        return false;
    };

    check_dir(arr, ny, nx, next, dir)
}

fn check_x_mas(arr: &Array2<char>, y: usize, x: usize) -> bool {
    if x == 0 || y == 0 {
        return false;
    }

    let Some(&top_left) = arr.get((y - 1, x - 1)) else {
        return false;
    };
    let Some(&top_right) = arr.get((y - 1, x + 1)) else {
        return false;
    };
    let Some(&bot_left) = arr.get((y + 1, x - 1)) else {
        return false;
    };
    let Some(&bot_right) = arr.get((y + 1, x + 1)) else {
        return false;
    };

    // Xor trick: check that the diagonal characters xorred
    // equals 'M' ^ 'S', then it's just enough to check that
    // one of them is either M or S

    let a_valid = top_left as u8 ^ bot_right as u8 == 0x1e && (top_left == 'M' || top_left == 'S');

    let b_valid = bot_left as u8 ^ top_right as u8 == 0x1e && (bot_left == 'M' || bot_left == 'S');

    a_valid && b_valid
}

fn part1(input: &Array2<char>) -> usize {
    input
        .indexed_iter()
        .map(|((y, x), c)| if c == &'X' { check(input, y, x) } else { 0 })
        .sum()
}

fn part2(input: &Array2<char>) -> usize {
    input
        .indexed_iter()
        .filter(|((y, x), &c)| c == 'A' && check_x_mas(input, *y, *x))
        .count()
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
        assert_eq!(p1, 2618);

        let p2 = part2(&input);
        assert_eq!(p2, 2011);
    }
}
