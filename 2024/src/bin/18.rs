use std::{collections::{HashSet, VecDeque}, io};
use aoclib::coord2::Coord2;
use itertools::Itertools;
use ndarray::Array2;

fn part1(input: &[usize]) -> usize {
    let (dim, n) = if input.len() > 1000 {
        ((71, 71), 1024)
    } else {
        ((7, 7), 12)
    };

    let mut map = Array2::from_elem(dim, false);

    for (&x, &y) in input.iter().tuples().take(n) {
        map[(y, x)] = true;
    }
    
    find_path(&map).unwrap()
}

fn find_path(map: &Array2<bool>) -> Option<usize> {
    let mut q = VecDeque::new();
    let mut visited: HashSet<Coord2> = HashSet::new();
    q.push_back(((0, 0).into(), 0));

    let target = (map.dim().0 - 1, map.dim().1 - 1).into();

    while let Some((p, d)) = q.pop_front() {
        if p == target {
            return Some(d);
        }

        if !visited.insert(p) {
            continue
        }

        for dir in [Coord2::UP, Coord2::RIGHT, Coord2::LEFT, Coord2::DOWN] {
            if let Some(np) = p.checked_add_with_upper(dir, map.dim()) {
                if !visited.contains(&np) && !map[np] {
                    q.push_back((np, d + 1));
                }
            }
        }
    }

    None
}


fn part2(input: &[usize]) -> String {
    let (dim, n) = if input.len() > 1000 {
        ((71, 71), 1024)
    } else {
        ((7, 7), 12)
    };

    let mut lower = n;
    let mut upper = input.len() / 2 - 1;

    if try_n(lower, dim, input) || !try_n(upper, dim, input) {
        panic!("Lower/upper bounds invalid");
    }

    while lower + 1 < upper {
        let half = lower + (lower.abs_diff(upper) / 2);
        println!("{lower} {upper} {half}");
        if try_n(half, dim, input) {
            upper = half;
        } else {
            lower = half;
        }
    }

    let (x, y) = input.iter().tuples().nth(lower).unwrap();
    return format!("{x},{y}");

    fn try_n(n: usize, dim: (usize, usize), input: &[usize]) -> bool {
        let mut map = Array2::from_elem(dim, false);
        for (&x, &y) in input.iter().tuples().take(n) {
            map[(y, x)] = true;
        }
        find_path(&map).is_none()
    }
}

fn main() -> io::Result<()> {
    let input = aoclib::read_input_ints(false)?;

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
        let input = aoclib::read_ints_from_file(aoclib::get_test_input_file!(18), false).unwrap();

        let p1 = part1(&input);
        assert_eq!(p1, 404);

        let p2 = part2(&input);
        assert_eq!(p2, "27,60");
    }
}
