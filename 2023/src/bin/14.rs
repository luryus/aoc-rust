use std::io;
use ndarray::Array2;

fn part1(mut input: Array2<char>) -> usize {
    let rock_pos: Vec<_> = input.indexed_iter().filter(|(_, r)| **r == 'O').map(|(c, _)| c).collect();
    for (y, x) in rock_pos {
        input[(y, x)] = '.';
        let new_y = input.column(x).indexed_iter().take(y+1).filter(|(_, r)| **r != '.').last().map(|(yy, _)| yy+1).unwrap_or(0);
        input[(new_y, x)] = 'O';
    }

    score(&input)
}

fn score(arr: &Array2<char>) -> usize {
    arr.indexed_iter().filter(|(_, r)| **r == 'O').map(|((y, _), _)| arr.dim().0-y).sum()
}

fn cycle(mut arr: Array2<char>) -> Array2<char> {
    // north
    let rock_pos: Vec<_> = arr.indexed_iter().filter(|(_, r)| **r == 'O').map(|(c, _)| c).collect();
    for (y, x) in rock_pos {
        arr[(y, x)] = '.';
        let new_y = arr.column(x).indexed_iter().take(y+1).filter(|(_, r)| **r != '.').last().map(|(yy, _)| yy+1).unwrap_or(0);
        arr[(new_y, x)] = 'O';
    }

    // west
    let rock_pos: Vec<_> = arr.indexed_iter().filter(|(_, r)| **r == 'O').map(|(c, _)| c).collect();
    for (y, x) in rock_pos {
        arr[(y, x)] = '.';
        let new_x = arr.row(y).indexed_iter().take(x+1).filter(|(_, r)| **r != '.').last().map(|(xx, _)| xx+1).unwrap_or(0);
        arr[(y, new_x)] = 'O';
    }

    // south
    let rock_pos: Vec<_> = arr.indexed_iter().filter(|(_, r)| **r == 'O').map(|(c, _)| c).collect();
    for (y, x) in rock_pos.into_iter().rev() {
        arr[(y, x)] = '.';
        let new_y = arr.column(x).indexed_iter().skip(y).take_while(|(_, r)| **r == '.').last().unwrap().0;
        arr[(new_y, x)] = 'O';
    }

    // east
    let rock_pos: Vec<_> = arr.indexed_iter().filter(|(_, r)| **r == 'O').map(|(c, _)| c).collect();
    for (y, x) in rock_pos.into_iter().rev() {
        arr[(y, x)] = '.';
        let new_x = arr.row(y).indexed_iter().skip(x).take_while(|(_, r)| **r == '.').last().unwrap().0;
        arr[(y, new_x)] = 'O';
    }

    arr
}


fn part2(input: Array2<char>) -> usize {
    let mut tortoise = input.clone();
    let mut hare = input;

    let mut loop_size = 0;

    for i in 1usize.. {
        tortoise = cycle(tortoise);
        hare = cycle(cycle(hare));

        if tortoise == hare {
            loop_size = i;
            break;
        }
    }

    let add_iters = (1_000_000_000 - loop_size).div_euclid(loop_size) * loop_size;

    for _ in loop_size+add_iters..1_000_000_000 {
        tortoise = cycle(tortoise);
    }
    
    score(&tortoise)
}

fn main() -> io::Result<()> {
    let input = aoclib::read_input_char_matrix()?;

    let p1 = part1(input.clone());
    println!("Part 1: {}", p1);

    let p2 = part2(input);
    println!("Part 2: {}", p2);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_real_input() {
        let input = aoclib::read_file_char_matrix(aoclib::get_test_input_file!(14)).unwrap();

        let p1 = part1(input.clone());
        assert_eq!(p1, 111979);

        let p2 = part2(input);
        assert_eq!(p2, 102055);
    }
}
