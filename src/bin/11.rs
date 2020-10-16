use std::io;
use std::collections::HashMap;
use num_complex::Complex;

extern crate num_complex;

fn run(mem: &Vec<i64>, init: i64, visualize: bool) {
    let mut c = aoc2019::IntCodeComputer::new(&mem);
    let mut x = 0i32;
    let mut y = 0i32;
    let mut dir = Complex::new(0, 1);
    let mut panels: HashMap<(i32, i32), i64> = HashMap::new();
    panels.insert((x, y), init);
    while c.run() {
        while c.outputs.len() > 0 {
            let new_c = c.outputs.pop_front().unwrap();
            let turn = c.outputs.pop_front().unwrap();

            panels.insert((x, y), new_c);
            dir *= if turn == 0 { Complex::new(0, 1) } else { Complex::new(0, -1) };
            x += dir.re;
            y -= dir.im;
        }

        let color = panels.get(&(x, y)).unwrap_or(&0);
        c.inputs.push_back(*color);
    }

    println!("{:?}", panels.len());

    if visualize {
        let min_x = *panels.keys().map(|(x, _)| x).min().unwrap();
        let max_x = *panels.keys().map(|(x, _)| x).max().unwrap();
        let min_y = *panels.keys().map(|(_, y)| y).min().unwrap();
        let max_y = *panels.keys().map(|(_, y)| y).max().unwrap();

        for yy in min_y..=max_y {
            for xx in min_x..=max_x {
                if panels.get(&(xx, yy)).unwrap_or(&0) == &1 {
                    print!("#");
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    }
}

fn main() -> io::Result<()> {
    let input = aoc2019::read_stdin_to_string()?;

    let mem = input.trim().split(",").map(|i| i.parse().unwrap()).collect();

    run(&mem, 0, false);
    run(&mem, 1, true);

    Ok(())
}