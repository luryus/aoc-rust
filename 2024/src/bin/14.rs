use aoclib::coord2::Coord2;
use ndarray::Array2;
use std::io;

#[derive(Clone)]
struct Robot {
    pos: Coord2<usize>,
    vel: Coord2<isize>,
}

fn part1(input: &[Robot]) -> usize {
    let dim = (103usize, 101usize);

    let (q1, q2, q3, q4) = input
        .iter()
        .map(|r| r.pos.wrapping_add(r.vel * 100, dim))
        .fold((0, 0, 0, 0), |(a1, a2, a3, a4), p| match (p.y.cmp(&(dim.0 / 2)), p.x.cmp(&(dim.1 / 2))) {
            (std::cmp::Ordering::Less, std::cmp::Ordering::Less) => (a1 + 1, a2, a3, a4),
            (std::cmp::Ordering::Less, std::cmp::Ordering::Greater) => (a1, a2 +1, a3, a4),
            (std::cmp::Ordering::Greater, std::cmp::Ordering::Less) => (a1, a2, a3 + 1, a4),
            (std::cmp::Ordering::Greater, std::cmp::Ordering::Greater) => (a1, a2, a3, a4 + 1),
            _ => (a1, a2, a3, a4)
        });

    q1 * q2 * q3 * q4
}

fn part2(mut robots: Vec<Robot>) -> usize {
    let dim = (103usize, 101usize);

    let mut xx = 39;
    let mut yy = 99; 

    for i in 1..1_000_000 {
        for p in robots.iter_mut() {
            let c = p.pos.wrapping_add(p.vel, dim);
            p.pos = c;
        }

        if xx == i && yy == i {
            print_robots(&robots, dim);
            return i;
        }

        if xx == i {
            xx += dim.1;
        }
        if yy == i {
            yy += dim.0;
        }

    }

    unreachable!()
}

fn print_robots(robots: &[Robot], dim: (usize, usize)) {
    let mut arr: Array2<bool> = Array2::from_elem(dim, false);
    for r in robots {
        arr[r.pos] = true;
    }
    aoclib::print_bool_ndarray(arr.view());
}

fn main() -> io::Result<()> {
    let input = aoclib::read_input_lines()?;
    let input = parse_input(input);

    let p1 = part1(&input);
    println!("Part 1: {}", p1);

    let p2 = part2(input);
    println!("Part 2: {}", p2);

    Ok(())
}

fn parse_input(input: Vec<String>) -> Vec<Robot> {
    input
        .into_iter()
        .map(|l| {
            let nums: Vec<isize> = aoclib::read_ints_from_string(&l, true);
            Robot {
                pos: Coord2 {
                    x: nums[0].try_into().unwrap(),
                    y: nums[1].try_into().unwrap(),
                },
                vel: Coord2 {
                    x: nums[2],
                    y: nums[3],
                },
            }
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_real_input() {
        let input = aoclib::read_file_lines(aoclib::get_test_input_file!(14)).unwrap();
        let input = parse_input(input);

        let p1 = part1(&input);
        assert_eq!(p1, 217328832);

        let p2 = part2(input);
        assert_eq!(p2, 7412);
    }
}
