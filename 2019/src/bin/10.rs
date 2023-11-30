use std::io;
use std::f32;

fn ring_in_bounds(center: (usize, usize), r: usize, w: usize, h: usize) -> Option<Vec<(usize, usize)>> {
    let (c_x, c_y) = center;
    if c_x < r && c_x + r > w && c_y < r && c_y + r > h {
        return None
    }

    let mut res = Vec::new();

    // Left edge
    if c_x >= r {
        for y in (c_y.saturating_sub(r))..((c_y+r+1).min(h)) {
            res.push((c_x - r, y))
        }
    }
    // Right edge
    if c_x + r < w {
        for y in (c_y.saturating_sub(r))..((c_y+r+1).min(h)) {
            res.push((c_x + r, y))
        }
    }
    // Top
    if c_y >= r {
        for x in (c_x.saturating_sub(r-1))..((c_x+r).min(w)) {
            res.push((x, c_y - r))
        }
    }
    // Bottom
    if c_y + r < h {
        for x in (c_x.saturating_sub(r-1))..((c_x+r).min(w)) {
            res.push((x, c_y + r))
        }
    }

    Some(res)
}

fn angle(cx: usize, cy: usize, tx: usize, ty: usize) -> f32 {
    let dx = tx as i32 - cx as i32;
    let dy = cy as i32 - ty as i32;

    let angle = (-(dy as f32).atan2(dx as f32) + f32::consts::FRAC_PI_2 + (2.0 * f32::consts::PI)) % (2.0*f32::consts::PI);
    
    angle
}


fn seen_asteroids(x: usize, y: usize, map: &Vec<Vec<char>>) -> Vec<(usize, usize, f32)> {
    let mut r = 1usize;
    let mut seen_roids = Vec::new();

    while let Some(ring_coords) = ring_in_bounds((x, y), r, map[y].len(), map.len()) {
        for (rx, ry) in ring_coords {
            if map[ry][rx] == '#' {
                let angle = angle(x, y, rx, ry);
                if !seen_roids.iter().any(|(_, _, a)| *a == angle) {
                    seen_roids.push((rx, ry, angle));
                }
            }
        }

        r += 1;
    }

    seen_roids
}

fn run1(map: &Vec<Vec<char>>) -> (usize, (usize, usize)) {

    let mut top = 0usize;
    let mut top_coord = (0usize, 0usize);

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == '#' {
                let asts = seen_asteroids(x, y, &map).len();
                if asts > top {
                    top = asts;
                    top_coord = (x, y);
                }
            }
        }
    }

    (top, top_coord)
}

fn part2(x: usize, y: usize, map: &Vec<Vec<char>>) -> (usize, usize, f32) {
    let mut seen = seen_asteroids(x, y, map);

    seen.sort_by(|(_, _, a), (_, _, b)| a.partial_cmp(b).unwrap());
    
    seen[199]
}

fn main() -> io::Result<()> {
    let input = aoc2019::read_stdin_lines()?;

    let map = input.into_iter()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let (part1, (best_x, best_y)) = run1(&map);
    println!("{:?}", part1);

    let (part2_x, part2_y, _) = part2(best_x, best_y, &map);
    println!("{}", part2_x * 100 + part2_y);

    Ok(())
}