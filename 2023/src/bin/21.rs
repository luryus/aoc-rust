use itertools::Itertools;
use ndarray::Array2;
use num_integer::Integer;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    io,
};

type Coord = (usize, usize);

fn adjacents(y: usize, x: usize, h: usize, w: usize) -> impl Iterator<Item = Coord> {
    [
        if x > 0 { Some((y, x - 1)) } else { None },
        if x < w - 1 { Some((y, x + 1)) } else { None },
        if y > 0 { Some((y - 1, x)) } else { None },
        if y < h - 1 { Some((y + 1, x)) } else { None },
    ]
    .into_iter()
    .flatten()
}

fn wrapping_adjacents(
    y: usize,
    x: usize,
    h: usize,
    w: usize,
) -> impl Iterator<Item = (i32, i32, usize, usize)> {
    [
        if x > 0 {
            (0, 0, y, x - 1)
        } else {
            (0, -1, y, w - 1)
        },
        if x < w - 1 {
            (0, 0, y, x + 1)
        } else {
            (0, 1, y, 0)
        },
        if y > 0 {
            (0, 0, y - 1, x)
        } else {
            (-1, 0, h - 1, x)
        },
        if y < h - 1 {
            (0, 0, y + 1, x)
        } else {
            (1, 0, 0, x)
        },
    ]
    .into_iter()
}

fn find_targets(
    steps_remaining: u8,
    y: usize,
    x: usize,
    grid: &Array2<char>,
    target_bitmap: &mut Array2<bool>,
    visited: &mut HashSet<(u8, usize, usize)>,
) {
    if !visited.insert((steps_remaining, y, x)) {
        return;
    }

    if steps_remaining == 0 {
        target_bitmap[(y, x)] = true;
        return;
    }

    let (h, w) = grid.dim();
    adjacents(y, x, h, w)
        .filter(|(yy, xx)| grid[(*yy, *xx)] == '.')
        .for_each(|(yy, xx)| {
            find_targets(steps_remaining - 1, yy, xx, grid, target_bitmap, visited)
        });
}

fn find_min_pos(ds: &Array2<usize>) -> Vec<(usize, usize)> {
    ds.indexed_iter().min_set_by_key(|(_, d)| *d).into_iter().map(|(c, _)| c).collect()
}

fn find_targets_count(
    steps_remaining: u8,
    pos: Vec<(usize, usize)>,
    grid: &Array2<char>
) -> usize {
    let mut bitmap = Array2::default(grid.dim());
    let mut visited = HashSet::new();

    for (y, x) in pos.iter().copied() {
        find_targets(steps_remaining, y, x, grid, &mut bitmap, &mut visited);
    }

    bitmap.iter().filter(|x| **x).count()
}

fn part1(input: &Array2<char>, start: (usize, usize)) -> usize {
    let mut target_bitmap = Array2::<bool>::default(input.dim());
    let mut visited_state = HashSet::new();
    find_targets(
        64,
        start.0,
        start.1,
        input,
        &mut target_bitmap,
        &mut visited_state,
    );

    target_bitmap.iter().filter(|x| **x).count()
}

fn part2(grid: &Array2<char>, start: (usize, usize)) -> usize {
    let mut distance_maps = Array2::from_elem((5, 5), Array2::from_elem(grid.dim(), usize::MAX));
    let mut q = VecDeque::new();

    let (h, w) = grid.dim();

    // Calculate distances for 5x5 full squares using BFS
    for (dy, dx, y, x) in wrapping_adjacents(start.0, start.1, h, w) {
        q.push_back((dy, dx, y, x, 1));
    }

    while let Some((dy, dx, y, x, dist)) = q.pop_front() {
        let dm = &mut distance_maps[((dy + 2) as usize, (dx + 2) as usize)];
        let d = &mut dm[(y, x)];
        if *d <= dist {
            continue;
        }
        *d = dist;

        let next_coords = wrapping_adjacents(y, x, h, w)
            .map(|(dyy, dxx, yy, xx)| (dy + dyy, dx + dxx, yy, xx))
            .filter(|(dyy, dxx, yy, xx)| {
                grid[(*yy, *xx)] == '.' && *dyy >= -2 && *dyy <= 2 && *dxx >= -2 && *dxx <= 2
            });
        for (ddy, ddx, yy, xx) in next_coords {
            q.push_back((ddy, ddx, yy, xx, dist + 1));
        }
    }

    for dm in distance_maps.iter_mut() {
        for (_, d) in dm.indexed_iter_mut() {
            if d.is_even() {
                *d = usize::MAX;
            }
        }
    }
    
    let filled_even_val = distance_maps[(0, 0)].iter().filter(|x| **x < usize::MAX).count();
    let filled_odd_val = distance_maps[(0, 1)].iter().filter(|x| **x < usize::MAX).count();

    let min_dist_1_1 = *distance_maps[(1 + 2, 1 + 2)].iter().min().unwrap();
    let max_dist_1_1 = *distance_maps[(1+2, 1+2)].iter().filter(|x| **x < usize::MAX).max().unwrap();
    let min_dist_1_2 = *distance_maps[(1+2, 2+2)].iter().min().unwrap();
    let corner_dist_delta = min_dist_1_2 - min_dist_1_1;

    let calc_corner_min_dist = |d: usize| (d-2) * corner_dist_delta + min_dist_1_1 + (d-2) / 2 * 2;
    let calc_corner_max_dist = |d: usize| (d-2) * corner_dist_delta + max_dist_1_1 + (d-1) / 2 * 2;

    let min_dist_0_1 = *distance_maps[(2, 1 + 2)].iter().min().unwrap();
    let max_dist_0_1 = *distance_maps[(2, 1+2)].iter().filter(|x| **x < usize::MAX).max().unwrap();
    let min_dist_0_2 = *distance_maps[(2, 2+2)].iter().min().unwrap();
    let straight_dist_delta = min_dist_0_2 - min_dist_0_1;

    let calc_straight_min_dist = |d: usize| (d-1) * straight_dist_delta + min_dist_0_1 + (d-1) / 2 * 2;
    let calc_straight_max_dist = |d: usize| (d-1) * straight_dist_delta + max_dist_0_1 + (d-1) / 2 * 2;

    // now we know how to calculate min/max distances for each square
    // determine how far we get
    const STEPS: usize = 26501365;
    let corner_entered_sq_dist = (10..).find(|&d| calc_corner_min_dist(d) > STEPS).unwrap() - 1;
    let corner_filled_sq_dist = (10..).find(|&d| calc_corner_max_dist(d) > STEPS).unwrap() - 1;

    let straight_entered_sq_dist = (10..).find(|&d| calc_straight_min_dist(d) > STEPS).unwrap() - 1;
    let straight_filled_sq_dist = (10..).find(|&d| calc_straight_max_dist(d) > STEPS).unwrap() - 1;

    assert_eq!(corner_filled_sq_dist, straight_filled_sq_dist);
    let filled_sq_dist = corner_filled_sq_dist;

    // Filled squares are easy to calculate, but we must manually check the partial ones.
    // And I think we must do all 8 directions separately
    let mut partial_counts = HashMap::new();
    for d in filled_sq_dist+1..=straight_entered_sq_dist {
        let steps: u8 = (STEPS - calc_straight_min_dist(d)).try_into().unwrap();
        // top
        let start_pos = if d.is_even() {
            find_min_pos(&distance_maps[(0, 2)])
        }else {
            find_min_pos(&distance_maps[(1, 2)])
        };
        partial_counts.insert((d, -1, 0), find_targets_count(steps, start_pos, grid));

        // bottom
        let start_pos = if d.is_even() {
            find_min_pos(&distance_maps[(4, 2)])
        }else {
            find_min_pos(&distance_maps[(3, 2)])
        };
        partial_counts.insert((d, 1, 0), find_targets_count(steps, start_pos, grid));

        // left
        let start_pos = if d.is_even() {
            find_min_pos(&distance_maps[(2, 0)])
        }else {
            find_min_pos(&distance_maps[(2, 1)])
        };
        partial_counts.insert((d, 0, -1), find_targets_count(steps, start_pos, grid));

        // right
        let start_pos = if d.is_even() {
            find_min_pos(&distance_maps[(2, 4)])
        }else {
            find_min_pos(&distance_maps[(2, 3)])
        };
        partial_counts.insert((d, 0, 1), find_targets_count(steps, start_pos, grid));
    }

    for d in filled_sq_dist+1..=corner_entered_sq_dist {
        let steps: u8 = (STEPS - calc_corner_min_dist(d)).try_into().unwrap();
        // Top left
        let start_pos = if d.is_even() {
            find_min_pos(&distance_maps[(1, 1)])
        } else {
            find_min_pos(&distance_maps[(1, 0)])
        };
        partial_counts.insert((d, -1, -1), find_targets_count(steps, start_pos, grid));
        
        // Bottom left
        let start_pos = if d.is_even() {
            find_min_pos(&distance_maps[(3, 1)])
        } else {
            find_min_pos(&distance_maps[(3, 0)])
        };
        partial_counts.insert((d, 1, -1), find_targets_count(steps, start_pos, grid));

        // Bottom right
        let start_pos = if d.is_even() {
            find_min_pos(&distance_maps[(3, 3)])
        } else {
            find_min_pos(&distance_maps[(3, 4)])
        };
        partial_counts.insert((d, 1, 1), find_targets_count(steps, start_pos, grid));

        // Top right
        let start_pos = if d.is_even() {
            find_min_pos(&distance_maps[(1, 3)])
        } else {
            find_min_pos(&distance_maps[(1, 4)])
        };
        partial_counts.insert((d, -1, 1), find_targets_count(steps, start_pos, grid));
    }

    assert_eq!(partial_counts.len(), 12);

    // 1 + 4 * (Sum of 2, 4, 6, ..., n)
    // Sum of first n even numbers == n* (n+1)
    let filled_even_sq_count = 1 + 4 * (filled_sq_dist/2)*(filled_sq_dist/2 + 1); //1 + ((2..=filled_sq_dist).step_by(2).sum::<usize>()) * 4;

    // 4 * (Sum of 1, 3, 5, ..., n)
    // Sum of n first odd numbers == n^2
    let filled_odd_sq_count = (filled_sq_dist.div_ceil(2)).pow(2) * 4;

    let filled_sum = filled_even_sq_count * filled_even_val + filled_odd_sq_count * filled_odd_val;

    let partial_sum: usize = partial_counts.into_iter()
        .map(|((d, y, x), v)| {
            if y == 0 || x == 0 {
                v
            } else {
                (d-1) * v
            }
        })
        .sum();

    filled_sum + partial_sum
}

fn find_start(mut arr: Array2<char>) -> ((usize, usize), Array2<char>) {
    let start = arr.indexed_iter().find(|(_, c)| **c == 'S').unwrap().0;
    arr[start] = '.';
    (start, arr)
}

fn main() -> io::Result<()> {
    let input = aoclib::read_input_char_matrix()?;
    let (start, input) = find_start(input);

    let p1 = part1(&input, start);
    println!("Part 1: {}", p1);

    let p2 = part2(&input, start);
    println!("Part 2: {}", p2);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_real_input() {
        let input = aoclib::read_file_char_matrix(aoclib::get_test_input_file!(21)).unwrap();
        let (start, input) = find_start(input);

        let p1 = part1(&input, start);
        assert_eq!(p1, 3809);

        let p2 = part2(&input, start);
        assert_eq!(p2, 629720570456311);
    }
}
