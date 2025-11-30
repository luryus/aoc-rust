use std::io;
use std::cmp::min;

fn parse_line(line: &str) -> (u32, u32, u32) {
    let parts = line.split('x');
    let mut number_iter = parts.map(|p| p.parse::<u32>().unwrap());
    (
        number_iter.next().unwrap(),
        number_iter.next().unwrap(),
        number_iter.next().unwrap()
    )
}

fn main()-> io::Result<()>  {
    let input = aoc2015::read_stdin_lines()?;

    let areas = input.iter().map(|l| {
        let (h, l, d) = parse_line(l);
        let (a, b, c) = (h*l, l*d, h*d);

        2*a + 2*b + 2*c + min(min(a, b), c)
    });

    let ribbons = input.iter().map(|l| {
        let (h, l, d) = parse_line(l);
        let (min, mid) = if h < l {
            (h, min(d, l))
        } else {
            (l, min(h, d))
        };

        2 * min + 2 * mid + h*l*d
    });

    println!("1: {}", areas.sum::<u32>());
    println!("2: {}", ribbons.sum::<u32>());
    Ok(())
}