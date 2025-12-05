use std::{io, ops::RangeInclusive};

fn part1(input: &[RangeInclusive<u64>]) -> u64 {
    input
        .iter()
        .cloned()
        .flat_map(|r| r.into_iter())
        .filter(|&x| {
            let x = x.to_string();
            let mid = x.len() / 2;
            let (a, b) = x.split_at(mid);
            a == b
        })
        .sum()
}

fn digits(mut num: u64, buf: &mut [u8; 10]) -> usize {
    let mut count = 0;
    while num > 0 {
        buf[count] = (num % 10) as u8;
        num /= 10;
        count += 1;
    }
    count
}

fn part2(input: &[RangeInclusive<u64>]) -> u64 {
    input
        .iter()
        .cloned()
        .flat_map(|r| r.into_iter())
        .filter(|&x| {
            let mut buf = [0u8; 10];
            let xlen = digits(x, &mut buf);
            let x = &buf[..xlen];

            (1..=(xlen / 2))
                .filter(|&l| xlen.is_multiple_of(l))
                .any(|l| {
                    let pat = &x[..l];
                    x[l..].chunks_exact(l).all(|c| c == pat)
                })
        })
        .sum()
}

fn main() -> io::Result<()> {
    let input: Vec<u64> = aoclib::read_input_ints(false)?;
    let input = parse_input(input);

    let p1 = part1(&input);
    println!("Part 1: {}", p1);

    let p2 = part2(&input);
    println!("Part 2: {}", p2);

    Ok(())
}

fn parse_input(inp: Vec<u64>) -> Vec<RangeInclusive<u64>> {
    inp.as_chunks::<2>().0.iter()
        .map(|x| x[0]..=x[1])
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_real_input() {
        let input = aoclib::read_ints_from_file(aoclib::get_test_input_file!(2), false).unwrap();
        let input = parse_input(input);

        let p1 = part1(&input);
        assert_eq!(p1, 19386344315);

        //let p2 = part2(&input);
        //assert_eq!(p2, 34421651192);
    }
}
