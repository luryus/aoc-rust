use ndarray::Array2;
use std::io;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    Up = 1,
    Right = 2,
    Down = 4,
    Left = 8,
}

impl Dir {
    fn turn(&self) -> Self {
        use Dir::*;
        match self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }

    fn d(&self) -> (isize, isize) {
        use Dir::*;
        match self {
            Up => (-1, 0),
            Right => (0, 1),
            Down => (1, 0),
            Left => (0, -1),
        }
    }

    fn mov(&self, pos: (usize, usize), dim: (usize, usize)) -> Option<(usize, usize)> {
        let d = self.d();
        let y = pos.0.checked_add_signed(d.0)?;
        let x = pos.1.checked_add_signed(d.1)?;

        (y < dim.0 && x < dim.1).then_some((y, x))
    }
}

fn part1(input: &Array2<char>) -> usize {
    let mut input = input.clone();
    let mut pos = input.indexed_iter().find(|(_, &c)| c == '^').unwrap().0;
    input[pos] = '.';

    let mut dir = Dir::Up;
    loop {
        input[pos] = 'x';
        let Some(new_pos) = dir.mov(pos, input.dim()) else {
            break;
        };

        if input[new_pos] == '#' {
            dir = dir.turn();
        } else {
            pos = new_pos;
        }
    }

    input.iter().filter(|&&c| c == 'x').count()
}

fn part2(input: &Array2<char>) -> usize {
    let mut pos = input.indexed_iter().find(|(_, &c)| c == '^').unwrap().0;
    let mut dir = Dir::Up;
    let mut mtx = input.map(|c| (*c, 0u8));

    let mut count: usize = 0;
    loop {
        mtx[pos].1 |= dir as u8;
        if check_loop(dir, pos, &mtx) {
            count += 1;
        }

        let Some(new_pos) = dir.mov(pos, mtx.dim()) else {
            break;
        };

        if mtx[new_pos].0 == '#' {
            dir = dir.turn();
        } else {
            pos = new_pos;
        }
    }

    fn check_loop(mut dir: Dir, mut pos: (usize, usize), mtx: &Array2<(char, u8)>) -> bool {
        let mut mtx = mtx.clone();
        let Some(block_pos) = dir.mov(pos, mtx.dim()) else {
            return false;
        };
        let np = &mtx[block_pos];
        if np.0 != '.' || np.1 != 0u8 {
            return false;
        }
        mtx[block_pos] = ('#', 0);

        loop {
            mtx[pos].1 |= dir as u8;
            let Some(new_pos) = dir.mov(pos, mtx.dim()) else {
                return false;
            };
            if mtx[new_pos].0 == '#' {
                dir = dir.turn();
            } else if mtx[new_pos].1 & (dir as u8) != 0 {
                // loop
                return true;
            } else {
                pos = new_pos;
            }
        }
    }

    count
}

fn main() -> io::Result<()> {
    let input = aoclib::read_input_char_matrix()?;

    let p1 = part1(&input);
    println!("Part 1: {}", p1);

    let p2 = part2(&input);
    println!("Part 2: {}", p2);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_real_input() {
        let input = aoclib::read_file_char_matrix(aoclib::get_test_input_file!(6)).unwrap();

        let p1 = part1(&input);
        assert_eq!(p1, 4711);

        let p2 = part2(&input);
        assert_eq!(p2, 1562);
    }
}
