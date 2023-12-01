use std::io;

fn part1(input: &Vec<i32>) -> i32 {
    // let (x, y) = input.iter().tuple_combinations()
    //     .filter(|(&x, &y)| x + y == 2020)
    //     .next().unwrap();
    // x * y

    for i in 0..input.len() {
        for j in 0..input.len() {
            if input[i] + input[j] == 2020 {
                return input[i] * input[j];
            }
        }
    }
    unreachable!()
}

fn part2(input: &Vec<i32>) -> i32 {
    for i in 0..input.len() {
        for j in 0..input.len() {
            if input[i] + input[j] > 2020 {
                continue;
            }
            for k in 0..input.len() {
                if input[i] + input[j] + input[k] == 2020 {
                    return input[i] * input[j] * input[k];
                }
            }
        }
    }
    unreachable!()
}

fn main() -> io::Result<()> {
    let input = aoc2020::read_ints_from_stdin()?;

    let p1 = part1(&input);
    println!("Part 1: {}", p1);

    let p2 = part2(&input);
    println!("Part 2: {}", p2);

    Ok(())
}
