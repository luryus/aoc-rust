use arrayvec::ArrayVec;
use fraction::GenericFraction;
use fraction::{ConstOne, ConstZero};
use ndarray::{Array1, Array2};
use rayon::prelude::*;
use std::{
    collections::{HashSet, VecDeque},
    io,
};

type F = GenericFraction<i32>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Button(Vec<usize>);

#[derive(Debug, Clone)]
struct Machine {
    indicators: Vec<bool>,
    buttons: Vec<Button>,
    joltage: Vec<usize>,
}

struct BfsState1(Vec<bool>, usize, ArrayVec<usize, 12>);

fn part1(input: &[Machine]) -> usize {
    let counts = input.iter().map(search).sum();
    return counts;

    fn search(m: &Machine) -> usize {
        // BFS

        let indicator_count = m.indicators.len();

        let mut q = VecDeque::new();
        let mut visited = HashSet::new();

        q.push_back(BfsState1(
            vec![false; indicator_count],
            0,
            Default::default(),
        ));

        while let Some(BfsState1(state_ind, depth, pressed)) = q.pop_front() {
            if state_ind == m.indicators {
                return depth;
            }

            if visited.contains(&state_ind) {
                continue;
            }

            visited.insert(state_ind.clone());

            for i in 0..m.buttons.len() {
                if pressed.contains(&i) {
                    continue;
                }

                let new_ind = m.buttons[i].0.iter().fold(state_ind.clone(), |mut acc, j| {
                    acc[*j] = !acc[*j];
                    acc
                });
                let mut pressed = pressed.clone();
                pressed.push(i);
                q.push_back(BfsState1(new_ind, depth + 1, pressed));
            }
        }

        unreachable!()
    }
}

fn part2(input: &Vec<Machine>) -> usize {
    let counts = input.par_iter().map(solve).sum();
    return counts;

    fn solve(m: &Machine) -> usize {
        // Create matrix
        // Buttons are columns, final column the joltages
        let h = m.joltage.len();
        let w = m.buttons.len() + 1;
        let mut mtx = ndarray::Array2::zeros((h, w));

        for (i, j) in m.joltage.iter().enumerate() {
            mtx[(i, w - 1)] = F::new(*j as i32, 1);
        }

        for (j, but) in m.buttons.iter().enumerate() {
            for &i in &but.0 {
                mtx[(i, j)] = F::ONE;
            }
        }

        // Transform to row echelon
        let mut row = 0;
        let mut col = 0;
        while row < h && col < w - 1 {
            // Find row with max value in column col
            let column = mtx.column(col);
            let Some(pivot_row) = column
                .indexed_iter()
                .skip(row)
                .max_by_key(|(_, x)| x.abs())
                .filter(|(_, x)| **x != F::ZERO)
            else {
                col += 1;
                continue;
            };

            let pivot_row_idx = pivot_row.0;
            let pivot_row = mtx.row(pivot_row_idx).to_owned();
            let r_row = mtx.row(row).to_owned();
            mtx.row_mut(row).assign(&pivot_row);
            mtx.row_mut(pivot_row_idx).assign(&r_row);

            for y in (row + 1)..h {
                let f = mtx[(y, col)] / mtx[(row, col)];
                assert!(!f.is_nan());
                mtx[(y, col)] = F::new_raw(0, 1);
                for x in (col + 1)..w {
                    let r = mtx[(row, x)] * f;
                    mtx[(y, x)] -= r;
                }
            }
            row += 1;
            col += 1;
        }

        let pivot_button_indices: Vec<_> = mtx
            .rows()
            .into_iter()
            .filter_map(|r| {
                r.indexed_iter()
                    .filter_map(|(pos, v)| (v != &F::ZERO).then_some(pos))
                    .next()
            })
            .collect();
        let mut free_vars: Vec<_> = Default::default();
        for i in 0..m.buttons.len() {
            if pivot_button_indices.contains(&i) {
                continue;
            }
            let but = &m.buttons[i];
            let max_value = but.0.iter().map(|j| m.joltage[*j]).min().unwrap();
            free_vars.push((i, max_value as i32));
        }

        let mut free_var_vals = free_vars.clone();
        search(&mtx, &free_vars, &mut free_var_vals, 0).unwrap()
    }

    // DFS search for minimum valid free variables
    fn search(
        mtx: &Array2<F>,
        free_var_defs: &Vec<(usize, i32)>,
        free_var_values: &mut Vec<(usize, i32)>,
        depth: usize,
    ) -> Option<usize> {
        if depth == free_var_defs.len() {
            return solve_mtx(mtx, free_var_values);
        }

        let (pos, max) = free_var_defs[depth];
        let mut best = usize::MAX;
        for i in 0..=max {
            free_var_values[depth] = (pos, i);
            match search(mtx, free_var_defs, free_var_values, depth + 1) {
                None => continue,
                Some(r) => best = best.min(r),
            };
        }

        (best != usize::MAX).then_some(best)
    }

    fn solve_mtx(mtx: &Array2<F>, free_var_values: &Vec<(usize, i32)>) -> Option<usize> {
        let (h, w) = mtx.dim();
        let mut btns = Array1::zeros(w - 1);
        for &(pos, val) in free_var_values {
            btns[pos] = val;
        }

        for y in (0..h).rev() {
            let row = mtx.row(y);
            let constant = row[row.len() - 1];
            let Some((pivot_pos, pivot_coeff)) = row
                .indexed_iter()
                .find(|(_, val)| **val != F::ZERO)
            else {
                continue;
            };

            let sum_known: F = row.iter().zip(&btns).map(|(a, b)| a * *b).sum();
            let new_btn = (constant - sum_known) / *pivot_coeff;
            //println!("btn[{}] = ({} - {}) / {} = {}", pivot_pos, constant, sum_known, *pivot_coeff, new_btn);
            if new_btn.is_sign_negative() || new_btn.denom() != Some(&1) {
                return None;
            }
            let btn_count = *new_btn.numer().unwrap();
            btns[pivot_pos] = btn_count;
        }

        Some(btns.sum() as usize)
    }
}

fn parse_input(input: Vec<String>) -> Vec<Machine> {
    input
        .into_iter()
        .map(|l| {
            let parts: Vec<_> = l.split_ascii_whitespace().collect();
            assert!(parts.len() >= 3);

            let indicators = parts[0][1..parts[0].len() - 1]
                .chars()
                .map(|c| c == '#')
                .collect();

            let joltage = aoclib::read_ints_from_string(parts[parts.len() - 1], false);

            let buttons = parts[1..parts.len() - 1]
                .iter()
                .map(|b| Button(aoclib::read_ints_from_string(b, false)))
                .collect();

            Machine {
                indicators,
                buttons,
                joltage,
            }
        })
        .collect()
}

fn main() -> io::Result<()> {
    let input = parse_input(aoclib::read_input_lines()?);

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
        let input = parse_input(aoclib::read_file_lines(aoclib::get_test_input_file!(10)).unwrap());

        let p1 = part1(&input);
        assert_eq!(p1, 530);

        let p2 = part2(&input);
        assert_eq!(p2, 20172)
    }
}
