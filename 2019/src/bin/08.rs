use std::io;
use std::iter::repeat;
use itertools::Itertools;

const W: usize = 25;
const H: usize = 6;

fn main() -> io::Result<()> {
    let input = aoc2019::read_stdin_to_string()?;

    const DIGITS_PER_LAYER: usize = W * H;
    let layers = input
        .trim().chars()
        .chunks(DIGITS_PER_LAYER).into_iter()
        .map(|l| l.collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let min_layer = layers.iter()
        .min_by_key(|l| l.iter().filter(|c| **c == '0').count()).unwrap();
    let min_layer_ones = min_layer.iter().filter(|c| **c == '1').count();
    let min_layer_twos = min_layer.iter().filter(|c| **c == '2').count();
    
    println!("Part 1: {}", min_layer_ones * min_layer_twos);

    // Flatten the layers
    let rendered = layers.iter()
        .fold(repeat('2').take(DIGITS_PER_LAYER).collect::<Vec<_>>(), 
            |acc, l| {
                l.iter().zip(acc).map(|(a, b)| {
                    if b == '2' { *a } else { b }
                }).collect()
            });

    for row in rendered.chunks(W).into_iter() {
        println!("{}", row.iter().map(|c| match c {
            '0' => ' ',
            '1' => '#',
            _ => '.',
        }).collect::<String>());
    }

    Ok(())
}