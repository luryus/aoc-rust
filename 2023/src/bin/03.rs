use itertools::Itertools;
use lazy_static::lazy_static;
use ndarray::{s, Array2};
use regex::Regex;
use std::{collections::HashMap, io};

lazy_static! {
    static ref NUM_REGEX: Regex = Regex::new(r"(\d+)").unwrap();
}

fn part1(input: &Array2<char>) -> usize {
    let (rows, cols) = input.dim();
    let mut resnums: Vec<usize> = vec![];
    for r in 0..rows {
        let row_string = input.row(r).iter().collect::<String>();
        let nums = NUM_REGEX.find_iter(&row_string);
        for n in nums {
            let start = n.start();
            let end = n.end() - 1;

            let xstart = start.saturating_sub(1);
            let xend = (end + 2).min(cols);
            let ystart = r.saturating_sub(1);
            let yend = (r + 2).min(rows);

            if input
                .slice(s![ystart..yend, xstart..xend])
                .iter()
                .any(|&c| c != '.' && !c.is_ascii_digit())
            {
                resnums.push(n.as_str().parse().unwrap())
            }
        }
    }

    resnums.into_iter().sum()
}

fn part2(input: &Array2<char>) -> usize {
    let (rows, cols) = input.dim();
    let mut geared: HashMap<(usize, usize), Vec<usize>> = HashMap::new();
    for r in 0..rows {
        let row_string = input.row(r).iter().collect::<String>();
        let nums = NUM_REGEX.find_iter(&row_string);
        for n in nums {
            let start = n.start();
            let end = n.end() - 1;
            let numparsed = n.as_str().parse().unwrap();

            let xstart = start.saturating_sub(1);
            let xend = (end + 2).min(cols);
            let ystart = r.saturating_sub(1);
            let yend = (r + 2).min(rows);

            let sl = input.slice(s![ystart..yend, xstart..xend]);
            let stars = sl.indexed_iter().filter(|(_, c)| c == &&'*');

            for ((sy, sx), _) in stars {
                geared
                    .entry((sy+ystart, sx+xstart))
                    .or_default()
                    .push(numparsed);
            }
        }
    }

    geared
        .values()
        .filter(|v| v.len() == 2)
        .map(|v| v.iter().product1::<usize>().unwrap())
        .sum()
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
        let input = aoclib::read_file_char_matrix(aoclib::get_test_input_file!(3)).unwrap();

        let p1 = part1(&input);
        assert_eq!(p1, 514969);

        let p2 = part2(&input);
        assert_eq!(p2, 78915902);
    }
}
