use std::io;
use num_complex::Complex;

fn mv(x: i32, y: i32, dir: Complex<i32>, amt: i32) -> (i32, i32) {
    (x + amt * dir.re, y + amt * -dir.im)
}

fn part1(input: &Vec<(char, i32)>) -> i32 {
    let (x, y, _) = input
        .into_iter()
        .fold((0, 0, Complex::new(1, 0)), |(accx, accy, accdir), (c, amt)| {
            let (x, y) = match c {
                'N' => mv(accx, accy, Complex::new(0, 1), *amt),
                'S' => mv(accx, accy, Complex::new(0, -1), *amt),
                'W' => mv(accx, accy, Complex::new(-1, 0), *amt),
                'E' => mv(accx, accy, Complex::new(1, 0), *amt),
                'F' => mv(accx, accy, accdir, *amt),
                _ => (accx, accy),
            };
            let dir = match c {
                'L' => accdir * Complex::new(0, 1).powi(*amt / 90),
                'R' => accdir * Complex::new(0, -1).powi(*amt / 90),
                _ => accdir,
            };

            (x, y, dir)
        });
    x.abs() + y.abs()
}

fn part2(input: &Vec<(char, i32)>) -> i32 {
    let (x, y, _) = input.into_iter().fold(
        (0, 0, Complex::new(10,1)),
        |(accx, accy, accwp), (c, amt)| {
            let wp = match c {
                'N' => Complex::new(accwp.re, accwp.im + *amt),
                'S' => Complex::new(accwp.re, accwp.im - *amt),
                'W' => Complex::new(accwp.re - *amt, accwp.im),
                'E' => Complex::new(accwp.re + *amt, accwp.im),
                'L' => accwp * Complex::new(0, 1).powi(*amt / 90),
                'R' => accwp * Complex::new(0, -1).powi(*amt / 90),
                _ => accwp,
            };
            let (x, y) = if *c == 'F' {
                (accx + *amt * wp.re, accy + *amt * wp.im)
            } else {
                (accx, accy)
            };

            (x, y, wp)
        },
    );
    x.abs() + y.abs()
}

fn parse_input() -> std::io::Result<Vec<(char, i32)>> {
    let lines = aoc2020::read_stdin_lines()?;

    Ok(lines
        .iter()
        .map(|l| l.split_at(1))
        .map(|(l, r)| (l.chars().nth(0).unwrap(), r.parse::<i32>().unwrap()))
        .collect::<Vec<_>>())
}

fn main() -> io::Result<()> {
    let input = parse_input()?;

    let p1 = part1(&input);
    println!("Part 1: {}", p1);

    let p2 = part2(&input);
    println!("Part 2: {}", p2);

    Ok(())
}
