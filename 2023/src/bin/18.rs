use itertools::Itertools;
use std::{
    cmp::Ordering,
    io,
    str::FromStr,
};

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
enum Dir {
    Left,
    Right,
    Up,
    Down,
}

impl Dir {
    fn delta(&self) -> (isize, isize) {
        match self {
            Dir::Left => (0, -1),
            Dir::Right => (0, 1),
            Dir::Up => (-1, 0),
            Dir::Down => (1, 0),
        }
    }
}

impl FromStr for Dir {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" | "3" => Ok(Self::Up),
            "D" | "1" => Ok(Self::Down),
            "L" | "2" => Ok(Self::Left),
            "R" | "0" => Ok(Self::Right),
            _ => Err(anyhow::anyhow!("Invalid dir")),
        }
    }
}

fn run(input: &PartInput) -> usize {
    let mut hor_lines: Vec<((isize, isize), (isize, isize))> = Vec::with_capacity(input.len());
    let mut ver_lines: Vec<((isize, isize), (isize, isize))> = Vec::with_capacity(input.len());
    let mut pos = (0, 0);
    for (dir, len) in input {
        let d = dir.delta();
        let d = (d.0 * len, d.1 * len);
        let end = (pos.0 + d.0, pos.1 + d.1);
        let line = match d.0.cmp(&0) {
            Ordering::Equal => match d.1 >= 0 {
                true => (pos, end),
                false => (end, pos),
            },
            Ordering::Less => (end, pos),
            Ordering::Greater => (pos, end),
        };
        if d.0 == 0 {
            hor_lines.push(line);
        } else {
            ver_lines.push(line)
        }
        pos = end;
    }

    let hor_line_ys = hor_lines.iter().map(|(a, _)| a.0).unique().sorted().collect::<Vec<_>>();
    let mut other_ys = vec![];
    for (ya, yb) in hor_line_ys.iter().copied().tuple_windows() {
        if yb - ya <= 1 {
            continue
        }
        other_ys.push((ya + 1, yb - ya - 1));
    }

    let mut count = 0;
    for y in hor_line_ys {
        let hor_y_lines: Vec<_> = hor_lines
            .iter()
            .filter(|(a, _)| a.0 == y)
            .map(|(a, b)| (a.1, b.1))
            .collect();
        let ver_intersect_lines: Vec<_> = ver_lines
            .iter()
            .filter(|(a, b)| (a.0..=b.0).contains(&y))
            .collect();
        let intersect_points: Vec<_> = ver_intersect_lines
            .iter()
            .map(|(a, _)| a.1)
            .sorted()
            .collect();

        let mut inside = false;
        let mut prev = isize::MIN;

        for x in intersect_points {
            if x <= prev {
                continue;
            }
            if let Some(&horyline) = hor_y_lines.iter().find(|l| l.0 == x) {
                let start_ver_line = ver_intersect_lines
                    .iter()
                    .find(|(a, _)| a.1 == horyline.0)
                    .unwrap();
                let end_ver_line = ver_intersect_lines
                    .iter()
                    .find(|(a, _)| a.1 == horyline.1)
                    .unwrap();

                let lines_same_updown = (start_ver_line.0 .0 + start_ver_line.1 .0 - y < y)
                    == (end_ver_line.0 .0 + end_ver_line.1 .0 - y < y);

                if inside {
                    // Fill until this + the line size
                    count += horyline.1 - prev;
                } else {
                    // fill line
                    count += horyline.1 - horyline.0 + 1;
                }
                prev = horyline.1;

                if !lines_same_updown {
                    inside = !inside;
                }
            } else {
                if inside {
                    // Fill until this + the line size
                    count += x - prev;
                } else {
                    // fill line
                    count += 1;
                }
                prev = x;
                inside = !inside;
            }
        }
    }

    for (y, mult) in other_ys {
        let ver_intersect_lines: Vec<_> = ver_lines
            .iter()
            .filter(|(a, b)| (a.0..=b.0).contains(&y))
            .collect();
        let intersect_points: Vec<_> = ver_intersect_lines
            .iter()
            .map(|(a, _)| a.1)
            .sorted()
            .collect();

        let mut inside = false;
        let mut prev = isize::MIN;

        for x in intersect_points {
            if inside {
                // Fill until this + the line size
                count += mult * (x - prev);
            } else {
                // fill line
                count += mult;
            }
            prev = x;
            inside = !inside;
        }
    }

    count.try_into().unwrap()
}

type PartInput = Vec<(Dir, isize)>;

fn parse_input(input: Vec<String>) -> (PartInput, PartInput) {
    let mut part1 = Vec::with_capacity(input.len());
    let mut part2 = Vec::with_capacity(input.len());

    for s in input {
        let (dir, dist, color) = aoclib::split_to_tuple3(&s, " ").unwrap();
        let dir1 = dir.parse().unwrap();
        let dist1 = dist.parse().unwrap();
        let dir2 = color[7..8].parse().unwrap();
        let dist2 = isize::from_str_radix(&color[2..7], 16).unwrap();
        part1.push((dir1, dist1));
        part2.push((dir2, dist2));
    }

    (part1, part2)
}

fn main() -> io::Result<()> {
    let input = aoclib::read_input_lines()?;
    let (p1_input, p2_input) = parse_input(input);

    let p1 = run(&p1_input);
    println!("Part 1: {}", p1);

    let p2 = run(&p2_input);
    println!("Part 2: {}", p2);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_real_input() {
        let input = aoclib::read_file_lines(aoclib::get_test_input_file!(18)).unwrap();
        let (p1_input, p2_input) = parse_input(input);

        let p1 = run(&p1_input);
        assert_eq!(p1, 48400);

        let p2 = run(&p2_input);
        assert_eq!(p2, 72811019847283);
    }
}
