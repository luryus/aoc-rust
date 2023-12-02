use std::io;

struct Game {
    id: usize,
    reveals: Vec<(usize, usize, usize)>,
}

fn tuple_max(a: (usize, usize, usize), b: (usize, usize, usize)) -> (usize, usize, usize) {
    (a.0.max(b.0), a.1.max(b.1), a.2.max(b.2))
}

fn part1(input: &[Game]) -> usize {
    input
        .iter()
        .map(|g| (g, g.reveals.iter().copied().reduce(tuple_max).unwrap()))
        .filter(|(_, gmax)| gmax.0 <= 12 && gmax.1 <= 13 && gmax.2 <= 14)
        .map(|(g, _)| g.id)
        .sum()
}

fn part2(input: &[Game]) -> usize {
    input
        .iter()
        .map(|g| g.reveals.iter().copied().reduce(tuple_max).unwrap())
        .map(|(r, g, b)| r * g * b)
        .sum()
}

fn parse_input(input: Vec<String>) -> Vec<Game> {
    input
        .into_iter()
        .map(|l| {
            let (left, right) = aoclib::split_to_tuple2(&l, ": ").unwrap();
            let id = *aoclib::read_ints_from_string(left, false).first().unwrap();
            let reveals = right
                .split("; ")
                .map(|r| {
                    r.split(", ").fold((0, 0, 0), |(ar, ag, ab), c| {
                        let num = *aoclib::read_ints_from_string(c, false).first().unwrap();
                        if c.ends_with("red") {
                            (num, ag, ab)
                        } else if c.ends_with("green") {
                            (ar, num, ab)
                        } else if c.ends_with("blue") {
                            (ar, ag, num)
                        } else {
                            panic!("Unknown color in '{c}'")
                        }
                    })
                })
                .collect();
            Game { id, reveals }
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
        let input = aoclib::read_file_lines(aoclib::get_test_input_file!(2)).unwrap();
        let input = parse_input(input);

        let p1 = part1(&input);
        assert_eq!(p1, 2716);

        let p2 = part2(&input);
        assert_eq!(p2, 72227);
    }
}
