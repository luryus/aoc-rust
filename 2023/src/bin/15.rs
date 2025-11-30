use std::io;

struct Step<'a>(&'a str, bool, u16);
#[derive(Clone)]
struct Lens(String, u16);

type LensBox = Vec<Lens>;

fn hash(s: &str) -> usize {
    s.chars().fold(0u8, |acc, c| acc.wrapping_add(c as u8).wrapping_mul(17)).into()
}

fn part1(input: &str) -> usize {
    input.split(',').map(hash).sum()
}

fn parse_step(s: &str) -> Step<'_> {
    if let Some((label, val)) = aoclib::split_to_tuple2(s, "=") {
        return Step(label, true, val.parse().unwrap());
    }
    if let Some((label, _)) = aoclib::split_to_tuple2(s, "-") {
        return Step(label, false, 0);
    }
    panic!("Invalid step: {s}");
}

fn part2(input: &str) -> usize {
    let mut boxes = vec![LensBox::default(); 256];
    for s in input.split(',') {
        let step = parse_step(s);
        let lbox = &mut boxes[hash(step.0)];
        if step.1 {
            if let Some(prev) = lbox.iter_mut().find(|x| x.0 == step.0) {
                prev.1 = step.2;
            } else {
                lbox.push(Lens(step.0.to_owned(), step.2))
            }
        } else {
            lbox.retain(|x| x.0 != step.0);
        }
    }

    boxes
        .into_iter()
        .enumerate()
        .flat_map(|(bi, b)| {
            b.into_iter()
                .enumerate()
                .map(move |(li, l)| (bi + 1) * (li + 1) * (l.1 as usize))
        })
        .sum()
}

fn main() -> io::Result<()> {
    let input = aoclib::read_input_string()?;

    let p1 = part1(input.trim());
    println!("Part 1: {}", p1);

    let p2 = part2(input.trim());
    println!("Part 2: {}", p2);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_real_input() {
        let input = std::fs::read_to_string(aoclib::get_test_input_file!(15)).unwrap();

        let p1 = part1(input.trim());
        assert_eq!(p1, 508498);

        let p2 = part2(input.trim());
        assert_eq!(p2, 279116);
    }
}
