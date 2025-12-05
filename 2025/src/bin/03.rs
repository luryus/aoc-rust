use std::io;

fn first_max_pos(d: &[u8]) -> Option<(usize, u8)> {
    d.iter()
        .copied()
        .enumerate()
        .reduce(|(ap, ax), (p, x)| if x > ax { (p, x) } else { (ap, ax) })
}

fn find_best(l: usize, digits: &[u8]) -> Option<usize> {
    return inner(l, digits).and_then(|ds| {
        ds.iter()
            .rev()
            .map(|x| *x as usize)
            .reduce(|acc, x| acc * 10 + x)
    });

    fn inner(l: usize, digits: &[u8]) -> Option<Vec<u8>> {
        if l == 1 {
            return digits.iter().max().map(|m| vec![*m]);
        }

        let head = &digits[..digits.len() - l + 1];
        let (p, x) = first_max_pos(head).unwrap();

        inner(l - 1, &digits[(p + 1)..]).map(|mut v| {
            v.push(x);
            v
        })
    }
}

fn run<const N: usize>(input: &[String]) -> usize {
    input
        .iter()
        .map(|l| {
            let digits: Vec<_> = l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect();
            find_best(N, &digits).unwrap()
        })
        .sum::<usize>()
}

fn main() -> io::Result<()> {
    let input = aoclib::read_input_lines()?;

    let p1 = run::<2>(&input);
    println!("Part 1: {}", p1);

    let p2 = run::<12>(&input);
    println!("Part 2: {}", p2);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_real_input() {
        let input = aoclib::read_file_lines(aoclib::get_test_input_file!(3)).unwrap();

        let p1 = run::<2>(&input);
        assert_eq!(p1, 17107);

        let p2 = run::<12>(&input);
        assert_eq!(p2, 169349762274117);
    }
}
