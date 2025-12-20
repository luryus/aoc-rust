use std::{collections::{HashMap}, io, str::FromStr, u32};
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Point(u32, u32, u32);
impl FromStr for Point {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums = aoclib::read_ints_from_string(s, false);
        if nums.len() < 3 {
            Err(())
        } else {
            Ok(Point(nums[0], nums[1], nums[2]))
        }
    }
}

impl Point {
    fn sq_dist(&self, other: &Self) -> u64 {
        let dx = self.0.abs_diff(other.0) as u64;
        let dy = self.1.abs_diff(other.1) as u64;
        let dz = self.2.abs_diff(other.2) as u64;
        dx*dx + dy*dy + dz*dz
    }
}


fn run(input: &Vec<Point>) -> (usize, usize) {
    let mut all_pairs = vec![];

    for p in input {
        for x in input {
            if p < x {
                let d = p.sq_dist(x);
                all_pairs.push((d, p, x));
            }
        }
    }

    all_pairs.sort_by_key(|(d, _, _)| -(*d as i64));

    let mut max_group = 0;
    let mut group_count = 0;
    let mut groups: HashMap<Point, u32> = HashMap::new();

    let mut part1 = 0;
    let mut part2 = 0;

    for i in 1.. {
        let (_, a, b) = all_pairs.pop().unwrap();

        let ag = groups.get(a).copied();
        let bg = groups.get(b).copied();

        match (ag, bg) {
            (Some(g), None) => { groups.insert(*b, g); },
            (None, Some(g)) => { groups.insert(*a, g); },
            (None, None) => {
                groups.insert(*a, max_group);
                groups.insert(*b, max_group);
                max_group += 1;
                group_count += 1;
            },
            (Some(aa), Some(bb)) if aa != bb => {
                for (_, v) in groups.iter_mut() {
                    if *v == bb {
                        *v = aa;
                    }
                }
                group_count -= 1;
            }
            _ => {}
        }

        if i == 1000 {
            part1 = groups.values().counts().values().k_largest(3).copied().product()
        }
        else if group_count == 1 && groups.len() == input.len() {
            part2 = a.0 as usize * b.0 as usize;
            break;
        }
    }

    (part1, part2)

}

fn parse_input(lines: &[String]) -> Vec<Point> {
    lines.into_iter().map(|l| l.parse().unwrap()).collect()
} 

fn main() -> io::Result<()> {
    let input = parse_input(&aoclib::read_input_lines()?);

    let (p1, p2) = run(&input);
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_real_input() {
        let input = parse_input(&aoclib::read_file_lines(aoclib::get_test_input_file!(8)).unwrap());

        let (p1, p2) = run(&input);
        assert_eq!(p1, 135169);
        assert_eq!(p2, 302133440);
    }
}
