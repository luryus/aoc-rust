use aoclib::UnwrapOptionIterator;
use std::{collections::HashMap, io};

struct Dev<'a>(Vec<&'a str>);

fn part1(input: &HashMap<&str, Dev>) -> usize {
    return step(input, "you");

    fn step(input: &HashMap<&str, Dev>, curr: &str) -> usize {
        if curr == "out" {
            1
        } else {
            let dev = input.get(curr).unwrap();
            dev.0.iter().map(|n| step(input, n)).sum()
        }
    }
}

fn part2(input: &HashMap<&str, Dev>) -> usize {
    let mut cache = HashMap::new();
    return step(input, &mut cache, "svr", false, false);

    fn step<'a>(
        input: &'a HashMap<&'a str, Dev>,
        cache: &mut HashMap<(&'a str, bool, bool), usize>,
        curr: &'a str,
        fft_seen: bool,
        dac_seen: bool,
    ) -> usize {
        if curr == "out" {
            (fft_seen && dac_seen) as usize
        } else {
            let fft = fft_seen || (curr == "fft");
            let dac = dac_seen || (curr == "dac");
            let cache_key = (curr, fft, dac);
            if let Some(c) = cache.get(&cache_key) {
                *c
            } else {
                let dev = input.get(curr).unwrap();
                let res = dev.0.iter().map(|n| step(input, cache, n, fft, dac)).sum();
                cache.insert(cache_key, res);
                res
            }
        }
    }
}

fn main() -> io::Result<()> {
    let input = aoclib::read_input_lines()?;
    let input = parse_devs(&input);

    let p1 = part1(&input);
    println!("Part 1: {}", p1);

    let p2 = part2(&input);
    println!("Part 2: {}", p2);

    Ok(())
}

fn parse_devs(input: &[String]) -> HashMap<&str, Dev<'_>> {
    input
        .iter()
        .map(|l| {
            let (name, conns) = aoclib::split_to_tuple2(l, ": ")?;
            let conns = conns.split_ascii_whitespace().collect();
            Some((name, Dev(conns)))
        })
        .unwrap_options()
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_real_input() {
        let input = aoclib::read_file_lines(aoclib::get_test_input_file!(11)).unwrap();
        let input = parse_devs(&input);

        let p1 = part1(&input);
        assert_eq!(p1, 571);

        let p2 = part2(&input);
        assert_eq!(p2, 511378159390560);
    }
}
