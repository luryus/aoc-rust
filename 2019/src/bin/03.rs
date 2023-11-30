use std::io;
use std::collections::HashMap;
use std::iter::repeat;

fn get_wire_path(wire: &str) -> HashMap<(i32, i32), u32> {
    let mut wire_path: HashMap<(i32, i32), u32> = HashMap::new();
    wire.trim()
        .split(",")
        .map(|p| (p.chars().next().unwrap(), p[1..].parse::<usize>().unwrap()))
        .flat_map(|(wd, wl)| repeat(wd).take(wl))
        .scan(((0i32, 0i32), 0u32), |((x, y), c), m| {
            *c += 1;
            match m {
                'U' => *y -= 1,
                'D' => *y += 1,
                'L' => *x -= 1,
                'R' => *x += 1,
                _ => panic!("Invalid direction")
            };
            Some(((*x, *y), *c))
        })
        .for_each(|(k, c)| {
            if !wire_path.contains_key(&k) {
                wire_path.insert(k, c);
            }
        });
    wire_path
}

fn main() -> io::Result<()> {
    let input = aoc2019::read_stdin_lines()?;

    let wire1_path = get_wire_path(&input[0]);
    let wire2_path = get_wire_path(&input[1]);

    let part1 = wire1_path.keys()
        .filter(|k| wire2_path.contains_key(k))
        .map(|(x,y)| x.abs()+y.abs())
        .min().map(|s| s as u32);
    let part2 = wire1_path.iter()
        .filter_map(|(k, v)| wire2_path.get(k).map(|v2| *v + *v2))
        .min();
    println!("Part 1: {:?}", part1);
    println!("Part 2: {:?}", part2);

    Ok(())
}