use itertools::Itertools;
use ndarray::Array2;
use std::{collections::HashMap, io};

const MASK_TOP_LEFT: u8 = 0x1;
const MASK_TOP_RIGHT: u8 = 0x2;
const MASK_BOT_LEFT: u8 = 0x4;
const MASK_BOT_RIGHT: u8 = 0x8;

#[repr(transparent)]
#[derive(Default, Copy, Clone, Eq, PartialEq)]
struct Fill(u8);
impl Fill {
    fn fill_top_left(&self) -> Self {
        Self(self.0 | MASK_TOP_LEFT)
    }

    fn fill_top_right(&self) -> Self {
        Self(self.0 | MASK_TOP_RIGHT)
    }

    fn fill_bot_left(&self) -> Self {
        Self(self.0 | MASK_BOT_LEFT)
    }

    fn fill_bot_right(&self) -> Self {
        Self(self.0 | MASK_BOT_RIGHT)
    }

    fn fill_full(&self) -> Self {
        Self(0xf)
    }

    fn top_left(&self) -> bool {
        (self.0 & MASK_TOP_LEFT) != 0
    }

    fn top_right(&self) -> bool {
        (self.0 & MASK_TOP_RIGHT) != 0
    }

    fn bot_left(&self) -> bool {
        (self.0 & MASK_BOT_LEFT) != 0
    }

    fn bot_right(&self) -> bool {
        (self.0 & MASK_BOT_RIGHT) != 0
    }

    fn any_filled(&self) -> bool {
        self.0 != 0
    }

    fn expand(&self, pipe: char) -> Self {
        match pipe {
            '.' if self.any_filled() => self.fill_full(),
            '|' => {
                let mut f = Fill::default();
                if self.top_left() || self.bot_left() {
                    f = f.fill_bot_left().fill_top_left();
                }
                if self.top_right() || self.bot_right() {
                    f = f.fill_bot_right().fill_top_right();
                }
                f
            }
            '-' => {
                let mut f = Fill::default();
                if self.top_left() || self.top_right() {
                    f = f.fill_top_left().fill_top_right();
                }
                if self.bot_left() || self.bot_right() {
                    f = f.fill_bot_left().fill_bot_right();
                }
                f
            }
            'F' if self.0 & 0b0111 > 0 => self.fill_bot_left().fill_top_right().fill_top_left(),
            'L' if self.0 & 0b1101 > 0 => self.fill_bot_left().fill_top_left().fill_bot_right(),
            'J' if self.0 & 0b1110 > 0 => self.fill_bot_left().fill_top_right().fill_bot_right(),
            '7' if self.0 & 0b1011 > 0 => self.fill_top_right().fill_top_left().fill_bot_right(),
            _ => *self,
        }
    }

    fn _as_char(&self) -> char {
        match self.0 {
            0x0 => ' ',
            0x1 => '▘',
            0x2 => '▝',
            0x3 => '▀',
            0x4 => '▖',
            0x5 => '▌',
            0x6 => '▞',
            0x7 => '▛',
            0x8 => '▗',
            0x9 => '▚',
            0xa => '▐',
            0xb => '▜',
            0xc => '▄',
            0xd => '▙',
            0xe => '▟',
            0xf => '█',
            _ => '?',
        }
    }
}

fn pipe_open_left(pipe: char) -> bool {
    matches!(pipe, '-' | 'J' | '7')
}
fn pipe_open_right(pipe: char) -> bool {
    matches!(pipe, '-' | 'L' | 'F')
}
fn pipe_open_up(pipe: char) -> bool {
    matches!(pipe, '|' | 'L' | 'J')
}
fn pipe_open_down(pipe: char) -> bool {
    matches!(pipe, '|' | 'F' | '7')
}

fn find_start(input: &Array2<char>) -> ((usize, usize), char) {
    let (rows, cols) = input.dim();
    let (sy, sx) = input.indexed_iter().find(|(_, c)| **c == 'S').unwrap().0;
    let l = sx > 0 && pipe_open_right(input[(sy, sx - 1)]);
    let r = sx < cols - 1 && pipe_open_left(input[(sy, sx + 1)]);
    let u = sy > 0 && pipe_open_down(input[(sy - 1, sx)]);
    let d = sy < rows - 1 && pipe_open_up(input[(sy + 1, sx)]);

    assert_eq!([l, r, u, d].into_iter().filter(|x| *x).count(), 2);

    let c = match (l, r, u, d) {
        (true, true, false, false) => '-',
        (true, false, true, false) => 'J',
        (true, false, false, true) => '7',
        (false, true, true, false) => 'L',
        (false, true, false, true) => 'F',
        (false, false, true, true) => '|',
        _ => unreachable!(),
    };

    ((sy, sx), c)
}

fn directions(c: char) -> &'static [(isize, isize)] {
    match c {
        '-' => &[(0, -1), (0, 1)],
        '|' => &[(-1, 0), (1, 0)],
        'F' => &[(1, 0), (0, 1)],
        '7' => &[(1, 0), (0, -1)],
        'L' => &[(-1, 0), (0, 1)],
        'J' => &[(-1, 0), (0, -1)],
        _ => unreachable!("No direction for '{}'", c),
    }
}

fn go(
    input: &Array2<char>,
    vis: &mut HashMap<(usize, usize), usize>,
    pos: (usize, usize),
    from: (usize, usize),
    d: usize,
) {
    if let Some(prev_d) = vis.get(&pos) {
        if *prev_d <= d {
            return;
        }
    }
    vis.insert(pos, d);

    let next = directions(input[pos])
        .iter()
        .map(|(dy, dx)| {
            (
                pos.0.saturating_add_signed(*dy),
                pos.1.saturating_add_signed(*dx),
            )
        })
        .find(|p| *p != from)
        .unwrap();

    go(input, vis, next, pos, d + 1)
}

fn part1(input: &mut Array2<char>) -> (usize, HashMap<(usize, usize), usize>) {
    let (start_pos, c) = find_start(input);
    let mut visited = HashMap::new();
    visited.insert(start_pos, 0);
    input[start_pos] = c;

    for (dy, dx) in directions(c) {
        let pn = (
            start_pos.0.saturating_add_signed(*dy),
            start_pos.1.saturating_add_signed(*dx),
        );
        go(input, &mut visited, pn, start_pos, 1);
    }

    (*visited.values().max().unwrap(), visited)
}

fn part2(mut input: Array2<char>) -> usize {
    // find all the loop pieces just like in part 1
    let (_, visited) = part1(&mut input);

    // Replace all non-visited with dots
    for (_, c) in input
        .indexed_iter_mut()
        .filter(|(c, _)| !visited.contains_key(c))
    {
        *c = '.';
    }

    let mut fills: Array2<Fill> = Array2::default(input.dim());
    let (h, w) = fills.dim();

    // Fill outer boundary
    fills
        .row_mut(0)
        .iter_mut()
        .for_each(|f| *f = f.fill_top_left().fill_top_right());
    fills
        .row_mut(h - 1)
        .iter_mut()
        .for_each(|f| *f = f.fill_bot_left().fill_bot_right());
    fills
        .column_mut(0)
        .iter_mut()
        .for_each(|f| *f = f.fill_top_left().fill_bot_left());
    fills
        .column_mut(w - 1)
        .iter_mut()
        .for_each(|f| *f = f.fill_top_right().fill_bot_right());

    let mut prev = Array2::default(input.dim());
    while prev != fills {
        prev = fills.clone();
        for (y, x) in (0..h).cartesian_product(0..w) {
            let c = input[(y, x)];
            let f = &mut fills[(y, x)];
            *f = f.expand(c);
            let f = *f;

            if x > 0 {
                let af = &mut fills[(y, x - 1)];
                if f.top_left() {
                    *af = af.fill_top_right();
                }
                if f.bot_left() {
                    *af = af.fill_bot_right();
                }
            }
            if x < (w - 1) {
                let af = &mut fills[(y, x + 1)];
                if f.top_right() {
                    *af = af.fill_top_left();
                }
                if f.bot_right() {
                    *af = af.fill_bot_left();
                }
            }

            if y > 0 {
                let af = &mut fills[(y - 1, x)];
                if f.top_left() {
                    *af = af.fill_bot_left();
                }
                if f.top_right() {
                    *af = af.fill_bot_right();
                }
            }
            if y < (h - 1) {
                let af = &mut fills[(y + 1, x)];
                if f.bot_left() {
                    *af = af.fill_top_left();
                }
                if f.bot_right() {
                    *af = af.fill_top_right();
                }
            }
        }
    }

    fills.iter().filter(|f| !f.any_filled()).count()
}

fn main() -> io::Result<()> {
    let input = aoclib::read_input_char_matrix()?;

    let (p1, _) = part1(&mut input.clone());
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
        let input = aoclib::read_file_char_matrix(aoclib::get_test_input_file!(10)).unwrap();

        let (p1, _) = part1(&mut input.clone());
        assert_eq!(p1, 6649);

        let p2 = part2(input);
        assert_eq!(p2, 601);
    }
}
