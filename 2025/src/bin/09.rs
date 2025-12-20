use aoclib::coord2::Coord2;
use itertools::Itertools;
use std::io;

struct Line(Coord2, Coord2);

impl Line {
    fn is_vertical(&self) -> bool {
        self.0.x == self.1.x
    }

    fn contains(&self, point: &Coord2) -> bool {
        if self.is_vertical() {
            if self.0.x != point.x {
                return false;
            }
            let miny = self.0.y.min(self.1.y);
            let maxy = self.0.y.max(self.1.y);
            point.y >= miny && point.y <= maxy
        } else {
            if self.0.y != point.y {
                return false;
            }
            let minx = self.0.x.min(self.1.x);
            let maxx = self.0.x.max(self.1.x);
            point.x >= minx && point.x <= maxx
        }
    }
}

fn area(a: &Coord2, b: &Coord2) -> usize {
    (a.x.abs_diff(b.x) + 1) * (a.y.abs_diff(b.y) + 1)
}

fn part1(input: &Vec<Coord2>) -> usize {
    input
        .iter()
        .tuple_combinations()
        .map(|(a, b)| area(a, b))
        .max()
        .unwrap()
}

fn is_inside(c: Coord2, vertical_edges: &[Line], hor_edges: &[Line]) -> bool {
    if hor_edges.iter().take_while(|e| e.0.y <= c.y).any(|l| l.contains(&c)) {
        return true;
    }
    if vertical_edges.iter().take_while(|e| e.0.x <= c.x).any(|l| l.contains(&c)) {
        return true;
    }

    let mut count = 0;
    for Line(ea, eb) in vertical_edges {
        if ea.x > c.x {
            break;
        }
        let y_min = ea.y.min(eb.y);
        let y_max = ea.y.max(eb.y);
        if y_min < c.y && c.y <= y_max {
            count += 1;
        }
    }

    count % 2 == 1
}

fn intersection_point(a: &Line, b: &Line) -> Option<Coord2> {
    if a.is_vertical() == b.is_vertical() {
        return None;
    }

    let (hor, ver) = if a.is_vertical() { (b, a) } else { (a, b) };

    let (miny, maxy) = (ver.0.y.min(ver.1.y), ver.0.y.max(ver.1.y));
    let (minx, maxx) = (hor.0.x.min(hor.1.x), hor.0.x.max(hor.1.x));

    if miny <= hor.0.y && hor.0.y <= maxy && minx <= ver.0.x && ver.0.x <= maxx {
        Some(Coord2 {
            y: hor.0.y,
            x: ver.0.x,
        })
    } else {
        None
    }
}

fn part2(input: &Vec<Coord2>) -> usize {
    let (mut vert_edges, mut hor_edges) = input
        .iter()
        .chain(std::iter::once(&input[0]))
        .copied()
        .tuple_windows()
        .map(|(a, b)| Line(a, b))
        .partition::<Vec<_>, _>(|l| l.is_vertical());

    vert_edges.sort_by_key(|e| e.0.x);
    hor_edges.sort_by_key(|e| e.0.y);

    let mut max_area = 0;

    'rect: for (a, b) in input.iter().tuple_combinations() {
        if a.x == b.x || a.y == b.y {
            continue;
        }
        let ar = area(a, b);
        if max_area > ar {
            continue;
        }

        // Quick check: verify the other corners are inside
        if !is_inside(Coord2 { y: a.y, x: b.x }, &vert_edges, &hor_edges)
            || !is_inside(Coord2 { y: b.y, x: a.x }, &vert_edges, &hor_edges)
        {
            continue;
        }

        let hor_line_1 = Line(Coord2 { y: a.y, x: a.x }, Coord2 { y: a.y, x: b.x });
        let hor_line_2 = Line(Coord2 { y: b.y, x: a.x }, Coord2 { y: b.y, x: b.x });
        for ve in &vert_edges {
            for hl in [&hor_line_1, &hor_line_2] {
                if let Some(ip) = intersection_point(ve, &hl) {
                    let left = Coord2 {
                        y: ip.y,
                        x: ip.x - 1,
                    };
                    let right = Coord2 {
                        y: ip.y,
                        x: ip.x + 1,
                    };
                    if (hl.contains(&left) && !is_inside(left, &vert_edges, &hor_edges))
                        || (hl.contains(&right) && !is_inside(right, &vert_edges, &hor_edges))
                    {
                        continue 'rect;
                    }
                }
            }
        }

        let ver_line_1 = Line(Coord2 { y: a.y, x: a.x }, Coord2 { y: b.y, x: a.x });
        let ver_line_2 = Line(Coord2 { y: a.y, x: b.x }, Coord2 { y: b.y, x: b.x });
        for he in &hor_edges {
            for vl in [&ver_line_1, &ver_line_2] {
                if let Some(ip) = intersection_point(he, &vl) {
                    let top = Coord2 {
                        y: ip.y - 1,
                        x: ip.x,
                    };
                    let bot = Coord2 {
                        y: ip.y + 1,
                        x: ip.x,
                    };
                    if (vl.contains(&top) && !is_inside(top, &vert_edges, &hor_edges))
                        || (vl.contains(&bot) && !is_inside(bot, &vert_edges, &hor_edges))
                    {
                        continue 'rect;
                    }
                }
            }
        }

        max_area = ar;
    }

    max_area
}

fn parse_input(lines: Vec<String>) -> Vec<Coord2> {
    lines
        .into_iter()
        .map(|l| {
            let &[y, x] = aoclib::read_ints_from_string(&l, false).as_slice() else {
                panic!("invalid input row");
            };
            Coord2 { x, y }
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
        let input = parse_input(aoclib::read_file_lines(aoclib::get_test_input_file!(9)).unwrap());

        let p1 = part1(&input);
        assert_eq!(p1, 4748769124);

        let p2 = part2(&input);
        assert_eq!(p2, 1525991432);
    }
}
