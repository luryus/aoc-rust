use ndarray::{s, Array2};
use std::io;

fn unsmudge_reflection_score(cm: &Array2<i8>, transposed: bool) -> usize {
    for i in 0..cm.dim().1 - 1 {
        let view_after = cm.slice(s![.., i + 1..]);
        let view_before = cm.slice(s![.., ..i + 1]);
        let mut before_flipped = Array2::default(view_before.dim());
        for (j, c) in view_before.columns().into_iter().enumerate() {
            before_flipped
                .column_mut(before_flipped.dim().1 - 1 - j)
                .assign(&c);
        }

        let min_cols = before_flipped.dim().1.min(view_after.dim().1);
        before_flipped.slice_collapse(s![.., ..min_cols]);
        let diff = before_flipped - view_after.slice(s![.., ..min_cols]);

        if diff.iter().filter(|d| **d != 0).count() == 1 {
            return if transposed { 100 * (i + 1) } else { i + 1 };
        }
    }

    if !transposed {
        let cmt = cm.t().to_owned();
        return unsmudge_reflection_score(&cmt, true);
    }

    panic!("No smudge found")
}

fn score(cm: &Array2<i8>, transposed: bool) -> usize {
    'outer: for i in 0..cm.dim().1 - 1 {
        if cm.column(i) != cm.column(i + 1) {
            continue;
        }

        for (aa, bb) in ((i + 1)..cm.dim().1).zip((0..i + 1).rev()) {
            if cm.column(aa) != cm.column(bb) {
                continue 'outer;
            }
        }

        return i + 1;
    }

    if !transposed {
        let cmt = cm.t().to_owned();
        return 100 * score(&cmt, true);
    }

    panic!("No reflection found")
}

fn part1(input: &[Array2<i8>]) -> usize {
    input.iter().map(|cm| score(cm, false)).sum()
}

fn part2(input: &[Array2<i8>]) -> usize {
    input
        .iter()
        .map(|cm| unsmudge_reflection_score(cm, false))
        .sum()
}

fn parse_input(input: String) -> Vec<Array2<i8>> {
    input
        .split("\n\n")
        .map(|cc| aoclib::read_string_char_matrix(cc).unwrap())
        .map(|cm| {
            cm.mapv(|c| match c {
                '.' => 1,
                '#' => 2,
                _ => panic!(),
            })
        })
        .collect()
}

fn main() -> io::Result<()> {
    let input = aoclib::read_input_string()?;
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
        let input = std::fs::read_to_string(aoclib::get_test_input_file!(13)).unwrap();
        let input = parse_input(input);

        let p1 = part1(&input);
        assert_eq!(p1, 42974);

        let p2 = part2(&input);
        assert_eq!(p2, 27587);
    }
}
