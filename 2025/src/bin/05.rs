use std::io;
use itertools::Itertools;

fn part1(input: &[String]) -> usize {
    let (ranges, ings) = parse_input(input);

    ings.into_iter().filter(|i| ranges.iter().any(|(a, b)| i >= a && i <= b)).count()
}

fn overlap(al: u64, ah: u64, bl: u64 ,bh: u64) -> bool {
    al <= bh && bl <= ah
}

fn part2(input: &[String]) -> usize {
    let (mut ranges, _) = parse_input(input);
    
    loop {
        let mut valid: Vec<(u64, u64)> = vec![];
        for &(rl, rh) in &ranges {
            let mut overlap_found = false;
            for (vl, vh) in valid.iter_mut() {
                if overlap(rl, rh, *vl, *vh) {
                    *vl = rl.min(*vl);
                    *vh = rh.max(*vh);
                    overlap_found = true;
                    break;
                }
            }
            if !overlap_found {
                valid.push((rl, rh));
            }
        }
        if ranges.len() == valid.len() {
            break;
        }
        ranges = valid;
    }

    ranges.into_iter().map(|(a, b)| b - a + 1).sum::<u64>() as usize
}

fn main() -> io::Result<()> {
    let input = aoclib::read_input_lines()?;

    let p1 = part1(&input);
    println!("Part 1: {}", p1);

    let p2 = part2(&input);
    println!("Part 2: {}", p2);

    Ok(())
}

fn parse_input(inp: &[String]) -> (Vec<(u64, u64)>, Vec<u64>) {
    let (ranges, ings) = inp.split(|l| l.is_empty()).collect_tuple().unwrap();

    let ranges = ranges.iter().map(|l| {
        let (a, b) = aoclib::split_to_tuple2(l, "-").unwrap();
        (a.parse().unwrap(), b.parse().unwrap())
    }).collect();

    let ings = ings.iter().map(|l| l.parse().unwrap()).collect();

    (ranges, ings)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_real_input() {
        let input = aoclib::read_file_lines(aoclib::get_test_input_file!(5)).unwrap();

        let p1 = part1(&input);
        assert_eq!(p1, 874);

        let p2 = part2(&input);
        assert_eq!(p2, 348548952146313);
    }
}
