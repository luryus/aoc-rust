use arrayvec::ArrayVec;
use std::{
    fmt::{Display, Write},
    io,
    str::FromStr,
};

#[derive(Debug)]
struct Snafu(ArrayVec<i8, 32>);

impl From<i64> for Snafu {
    fn from(v: i64) -> Self {
        fn core(v: i64) -> ArrayVec<i8, 32> {
            let dig = v % 5;
            let (snafu_dig, carry) = match dig {
                0 => (0, 0),
                1 => (1, 0),
                2 => (2, 0),
                3 => (-2, 1),
                4 => (-1, 1),
                _ => unreachable!(),
            };

            let mut av = if v < 5 && carry == 0 {
                ArrayVec::new()
            } else {
                core(v / 5 + carry)
            };
            av.push(snafu_dig);
            av
        }

        Self(core(v))
    }
}

impl FromStr for Snafu {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Snafu(
            s.chars()
                .map(|c| match c {
                    '=' => -2,
                    '-' => -1,
                    '0' => 0,
                    '1' => 1,
                    '2' => 2,
                    _ => panic!(),
                })
                .collect(),
        ))
    }
}

impl From<Snafu> for i64 {
    fn from(s: Snafu) -> Self {
        s.0.iter()
            .rev()
            .zip(0..)
            .map(|(d, e)| *d as i64 * 5i64.pow(e))
            .sum()
    }
}

impl Display for Snafu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for d in &self.0 {
            let c = match d {
                -2 => '=',
                -1 => '-',
                0 => '0',
                1 => '1',
                2 => '2',
                _ => unreachable!(),
            };
            f.write_char(c)?;
        }
        Ok(())
    }
}

fn part1(input: &[String]) -> String {
    let sum: i64 = input
        .iter()
        .map(|s| s.parse::<Snafu>().expect("Could not parse snafu"))
        .map(i64::from)
        .sum();
    let snafu_sum: Snafu = dbg!(sum.into());
    snafu_sum.to_string()
}

fn main() -> io::Result<()> {
    let input = aoclib::read_input_lines()?;

    let p1 = part1(&input);
    println!("Part 1: {}", p1);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_real_input() {
        let input = aoclib::read_file_lines(aoclib::get_test_input_file!(25)).unwrap();

        let p1 = part1(&input);
        assert_eq!(p1, "2-0==21--=0==2201==2");
    }
}
