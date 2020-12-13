use std::io;

fn part1(input: &Vec<usize>) -> usize {
    let mut input = input.clone();
    input.sort();
    input.insert(0, 0);

    let (l, r) = input.iter().zip(input.iter().skip(1))
        .fold((0, 0), |(accl, accr), (l, r)| {
            let lr = accl + (r - l == 1) as usize;
            let rr = accr + (r - l == 3) as usize;
            (lr, rr)
        });

    l * (r + 1)
}

fn valid(input: &Vec<usize>, start_index: usize, mem: &mut Vec<Option<usize>>) -> usize {
    if let Some(x) = mem[start_index] {
        return x;
    }

    // If this is the last item, just add 1
    if start_index == input.len() - 1 {
        return 1;
    }

    let val = input[start_index];

    let res = (start_index+1..input.len())
        .take_while(|x| input[*x] - val <= 3)
        .map(|x| valid(input, x, mem))
        .sum();

    mem[start_index] = Some(res);
    res
}

fn part2(input: &Vec<usize>) -> usize {
    let mut input = input.clone();
    input.sort();
    input.insert(0, 0);

    let mut mem = vec![None; input.len()];
    valid(&input, 0, &mut mem)
}

fn main() -> io::Result<()> {
    let input = aoc2020::read_ints_from_stdin()?;

    let p1 = part1(&input);
    println!("Part 1: {}", p1);

    let p2 = part2(&input);
    println!("Part 2: {}", p2);

    Ok(())
}