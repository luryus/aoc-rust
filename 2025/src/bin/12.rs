use aoclib::{UnwrapOptionIterator, coord2::Coord2};
use arrayvec::ArrayVec;
use itertools::Itertools;
use ndarray::{Array2, s};
use rayon::prelude::*;
use std::{collections::HashMap, io, sync::OnceLock};

static SHAPE_VARIANT_CACHE: OnceLock<HashMap<Shape, Vec<Shape>>> = OnceLock::new();

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct Shape(Array2<bool>);

impl Shape {
    fn count(&self) -> usize {
        self.0.iter().filter(|b| **b).count()
    }
    fn variants(&self) -> Vec<Shape> {
        let (h, w) = self.0.dim();
        assert_eq!(h, w);

        let mut flipped = self.clone();
        flipped.flip();
        [
            self.clone(),
            self.rotate_cw(),
            self.rotate_cw().rotate_cw(),
            self.rotate_ccw(),
            flipped.rotate_cw(),
            flipped.rotate_ccw(),
            flipped.rotate_cw().rotate_cw(),
            flipped,
        ]
        .into_iter()
        .unique()
        .collect()
    }

    fn flip(&mut self) {
        let rlen = self.0.dim().1;
        for mut r in self.0.rows_mut() {
            for i in 0..(rlen / 2) {
                let tmp = r[i];
                r[i] = r[rlen - 1];
                r[rlen - 1] = tmp;
            }
        }
    }

    fn rotate_cw(&self) -> Self {
        let mut res = self.clone();
        res.0.reverse_axes();
        res.flip();
        res
    }

    fn rotate_ccw(&self) -> Self {
        let mut res = self.clone();
        res.flip();
        res.0.reverse_axes();
        res
    }
}
struct Space((usize, usize), Vec<usize>);

fn part1(shapes: &Vec<Shape>, spaces: &Vec<Space>) -> usize {
    SHAPE_VARIANT_CACHE
        .set(shapes.iter().map(|s| (s.clone(), s.variants())).collect())
        .unwrap();
    return spaces.par_iter().filter(|s| is_possible(s, shapes)).count();

    fn is_possible(space: &Space, shapes: &Vec<Shape>) -> bool {
        let mut map = Array2::default(space.0);
        let mut counts_remaining = space.1.clone();
        let small_holes = Array2::default(space.0);

        step(0, 0, &mut map, &mut counts_remaining, shapes, &small_holes)
    }

    fn step(
        prev_y: usize,
        prev_x: usize,
        map: &mut Array2<bool>,
        counts_remaining: &mut Vec<usize>,
        shapes: &Vec<Shape>,
        small_holes: &Array2<bool>,
    ) -> bool {
        let (h, w) = map.dim();
        if counts_remaining.iter().all(|c| *c == 0) {
            return true;
        }

        let new_small_holes = find_holes(small_holes, map);
        let effective_space_remaining =
            h * w - new_small_holes.map(|b| *b as usize).sum() - map.map(|b| *b as usize).sum();
        let space_still_needed = counts_remaining
            .iter()
            .enumerate()
            .map(|(pos, rem)| shapes[pos].count() * rem)
            .sum();
        if effective_space_remaining < space_still_needed {
            return false;
        }

        for sh_idx in 0..counts_remaining.len() {
            if counts_remaining[sh_idx] == 0 {
                continue;
            }
            let sh_variants = SHAPE_VARIANT_CACHE
                .get()
                .and_then(|cache| cache.get(&shapes[sh_idx]))
                .unwrap();

            let ymin = prev_y.saturating_sub(3);
            let ymax = prev_y + 3;
            let xmin = prev_x.saturating_sub(3);
            let xmax = prev_x + 3;
            for (y, x) in (ymin..=ymax).cartesian_product(xmin..=xmax) {
                if y + 2 >= h || x + 2 >= w {
                    continue;
                }
                for sh in sh_variants {
                    let mut win = map.slice_mut(s![y..y + 3, x..x + 3]);
                    if win.iter().zip(sh.0.iter()).any(|(a, b)| *a && *b) {
                        continue;
                    }
                    win |= &sh.0;
                    counts_remaining[sh_idx] -= 1;

                    if step(y, x, map, counts_remaining, shapes, &new_small_holes) {
                        return true;
                    }

                    counts_remaining[sh_idx] += 1;
                    let inv = !(&sh.0);
                    let mut win = map.slice_mut(s![y..y + 3, x..x + 3]);
                    win &= &inv;
                }
            }
        }

        false
    }
    fn find_holes(small_holes: &Array2<bool>, map: &Array2<bool>) -> Array2<bool> {
        let mut new = small_holes.clone();

        let (h, w) = small_holes.dim();
        for (y, x) in (0..h).cartesian_product(0..w) {
            if map[(y, x)] || new[(y, x)] {
                continue;
            }

            let mut coords: ArrayVec<Coord2, 12> = Default::default();
            coords.push((y, x).into());
            for i in 0.. {
                if coords.len() > 3 || i >= coords.len() {
                    break;
                }

                let curr = coords[i];
                let adjacent: ArrayVec<Coord2, 4> = [
                    curr.checked_add_with_upper((-1isize, 0isize).into(), (h, w)),
                    curr.checked_add_with_upper((1isize, 0isize).into(), (h, w)),
                    curr.checked_add_with_upper((0isize, -1isize).into(), (h, w)),
                    curr.checked_add_with_upper((0isize, 1isize).into(), (h, w)),
                ]
                .into_iter()
                .flatten()
                .filter(|c| !coords.contains(c) && !map[c.usizes()])
                .collect();
                coords.extend(adjacent);
            }

            if coords.len() <= 3 {
                for c in coords {
                    new[c.usizes()] = true;
                }
            }
        }

        new
    }
}

fn main() -> io::Result<()> {
    let (shapes, spaces) = parse_input(aoclib::read_input_string()?).unwrap();

    let p1 = part1(&shapes, &spaces);
    println!("Part 1: {}", p1);

    Ok(())
}

fn parse_input(input: String) -> Option<(Vec<Shape>, Vec<Space>)> {
    let parts: Vec<_> = input.split("\n\n").collect();

    let shapes = parts[..parts.len() - 1]
        .iter()
        .map(|p| {
            let (_, grid) = p.split_once("\n")?;
            Some(Shape(
                aoclib::read_string_char_matrix(grid)
                    .ok()?
                    .map(|c| *c == '#'),
            ))
        })
        .unwrap_options()
        .collect();

    let spaces = parts
        .last()?
        .lines()
        .map(|l| {
            let mut nums = aoclib::read_ints_from_string(l, false);
            let w = nums[0];
            let h = nums[1];
            let counts = nums.split_off(2);
            Some(Space((h, w), counts))
        })
        .unwrap_options()
        .collect();

    Some((shapes, spaces))
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_real_input() {
        let (shapes, spaces) =
            parse_input(std::fs::read_to_string(aoclib::get_test_input_file!(12)).unwrap())
                .unwrap();

        let p1 = part1(&shapes, &spaces);
        assert_eq!(p1, 565);
    }
}
