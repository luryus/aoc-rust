use itertools::Itertools;
use nalgebra::{Matrix2, Matrix2x1};
use num_integer::Integer;
use std::io;

struct Machine {
    a11: isize,
    a12: isize,
    a21: isize,
    a22: isize,
    b1: isize,
    b2: isize,
}

fn det(m: &Matrix2<isize>) -> isize {
    m.m11 * m.m22 - m.m21 * m.m12
}

fn solve<const UPPER_LIMIT: isize>(a: &Matrix2<isize>, b: &Matrix2x1<isize>) -> Option<Matrix2x1<isize>> {
    let a_det = det(a);
        
    let mut ax = *a;
    ax.set_column(0, b);
    let ax_det = det(&ax);
    let mut ay = *a;
    ay.set_column(1, b);
    let ay_det = det(&ay);

    let (x, remx) = ax_det.div_rem(&a_det);
    let (y, remy) = ay_det.div_rem(&a_det);

    if x < 0 || y < 0 || x > UPPER_LIMIT || y > UPPER_LIMIT || remx != 0 || remy != 0 {
        None
    } else {
        Some(Matrix2x1::new(x, y))
    }
}

fn part1(input: &Vec<Machine>) -> usize {
    let mut price = 0usize;
    for m in input {
        let a = Matrix2::new(m.a11, m.a12, m.a21, m.a22);
        let b = Matrix2x1::new(m.b1, m.b2);
        
        if let Some(r) = solve::<100>(&a, &b) {
            price += (r.x * 3 + r.y) as usize;
        }
    }

    price
}

fn part2(input: &Vec<Machine>) -> usize {
    let mut price = 0usize;
    for m in input {
        let a = Matrix2::new(m.a11, m.a12, m.a21, m.a22);
        let b = Matrix2x1::new(m.b1 + 10000000000000, m.b2 + 10000000000000);
        
        if let Some(r) = solve::<{isize::MAX}>(&a, &b) {
            price += (r.x * 3 + r.y) as usize;
        }
    }

    price
}

fn parse_input(input: Vec<String>) -> Vec<Machine> {
    input
        .split(|l| l.is_empty())
        .map(|ls| {
            assert_eq!(3, ls.len());
            let (a11, a21) = aoclib::read_ints_from_string(&ls[0], false)
                .into_iter()
                .collect_tuple()
                .unwrap();
            let (a12, a22) = aoclib::read_ints_from_string(&ls[1], false)
                .into_iter()
                .collect_tuple()
                .unwrap();
            let (b1, b2) = aoclib::read_ints_from_string(&ls[2], false)
                .into_iter()
                .collect_tuple()
                .unwrap();
            Machine {
                a11,
                a12,
                a21,
                a22,
                b1,
                b2,
            }
        })
        .collect()
}

fn main() -> io::Result<()> {
    let input = aoclib::read_input_lines()?;
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
        let input = aoclib::read_file_lines(aoclib::get_test_input_file!(13)).unwrap();
        let input = parse_input(input);

        let p1 = part1(&input);
        assert_eq!(p1, 29438);

        let p2 = part2(&input);
        assert_eq!(p2, 104958599303720);
    }
}
