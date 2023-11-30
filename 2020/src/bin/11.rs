use std::io;

fn adjacent_coords(x: usize, y: usize, w: usize, h: usize) -> Vec<(usize, usize)> {
    let mut res = Vec::new();
    if y > 0 {
        res.push((x, y - 1));
        if x > 0 {
            res.push((x - 1, y - 1));
        }
        if x < w -1 {
            res.push((x + 1, y - 1))
        }
    }

    if x > 0 {
        res.push((x - 1, y));
    }
    if x < w - 1 {
        res.push((x + 1, y))
    }

    if y < h - 1 {
        res.push((x, y + 1));
        if x > 0 {
            res.push((x - 1, y + 1));
        }
        if x < w -1 {
            res.push((x + 1, y + 1))
        }
    }

    res
}

fn valid_coord(x: i64, y: i64, w: i64, h: i64) -> bool {
    x >= 0 && x < w && y >= 0 && y < h
}

fn visible_coords(x: usize, y: usize, inp: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut res = Vec::new();
    let dirs = &[
        (-1i64, 0i64), (-1, -1), (0, -1), (1, -1),
        (1, 0), (1, 1), (0, 1), (-1, 1)
    ];
    let w = inp[0].len() as i64;
    let h = inp.len() as i64; 

    for (dx, dy) in dirs {
        let mut xx  = x as i64 + *dx;
        let mut yy = y as i64 + *dy;
        while valid_coord(xx, yy, w, h) {
            if inp[yy as usize][xx as usize] == '.' {
                xx += *dx;
                yy += *dy;
            } else {
                res.push((xx as usize, yy as usize));
                break;
            }
        }
    }


    res
}

fn part1(input: &Vec<Vec<char>>) -> usize {
    let mut prev_state = input.clone();
    let w = input[0].len();
    let h = input.len();
    loop {
        let mut new_state = prev_state.clone();

        for x in 0..w {
            for y in 0..h {
                let prev = prev_state[y][x];
                if prev == '.' {
                    continue
                }
                let adj: usize = adjacent_coords(x, y, w, h).into_iter()
                    .map(|(acx, acy)| (prev_state[acy][acx] == '#') as usize)
                    .sum();
                let new = match prev {
                    'L' if adj == 0 => '#',
                    '#' if adj >= 4 => 'L',
                    _ => prev
                };

                new_state[y][x] = new;
            }
        }

        let eq = new_state.iter().zip(&prev_state)
            .all(|(a, b)| a.iter().zip(b.iter()).all(|(ac, bc)| ac == bc));
        if eq {
            break;
        }

        prev_state = new_state;
    }

    prev_state
        .into_iter()
        .map(|l| l.into_iter().map(|c| (c == '#') as usize).sum::<usize>())
        .sum::<usize>()
}

fn part2(input: &Vec<Vec<char>>) -> usize {
    let mut prev_state = input.clone();
    let w = input[0].len();
    let h = input.len();
    loop {
        let mut new_state = prev_state.clone();

        for x in 0..w {
            for y in 0..h {
                let prev = prev_state[y][x];
                if prev == '.' {
                    continue
                }
                let adj: usize = visible_coords(x, y, input).into_iter()
                    .map(|(acx, acy)| (prev_state[acy][acx] == '#') as usize)
                    .sum();
                let new = match prev {
                    'L' if adj == 0 => '#',
                    '#' if adj >= 5 => 'L',
                    _ => prev
                };

                new_state[y][x] = new;
            }
        }

        let eq = new_state.iter().zip(&prev_state)
            .all(|(a, b)| a.iter().zip(b.iter()).all(|(ac, bc)| ac == bc));
        if eq {
            break;
        }

        prev_state = new_state;
    }

    prev_state
        .into_iter()
        .map(|l| l.into_iter().map(|c| (c == '#') as usize).sum::<usize>())
        .sum::<usize>()
}

fn main() -> io::Result<()> {
    let input = aoc2020::read_stdin_lines()?;
    let input = input.into_iter()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect();

    let p1 = part1(&input);
    println!("Part 1: {}", p1);

    let p2 = part2(&input);
    println!("Part 2: {}", p2);

    Ok(())
}