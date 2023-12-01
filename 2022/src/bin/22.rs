use ndarray::{s, Array2, ArrayView2};
use nom::{
    branch::alt,
    character::complete::char,
    character::complete::u32,
    combinator::{all_consuming, map},
    multi::many1,
    IResult,
};
use std::io;

#[derive(Copy, Clone, Debug)]
enum Dir {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}
impl Dir {
    fn ccw(&self) -> Dir {
        match self {
            Dir::Up => Dir::Left,
            Dir::Left => Dir::Down,
            Dir::Down => Dir::Right,
            Dir::Right => Dir::Up,
        }
    }

    fn cw(&self) -> Dir {
        match self {
            Dir::Up => Dir::Right,
            Dir::Left => Dir::Up,
            Dir::Down => Dir::Left,
            Dir::Right => Dir::Down,
        }
    }

    fn ccw_on_cube(&self, side: CubeSide) -> Self {
        match side {
            CubeSide::Right | CubeSide::Back | CubeSide::Bottom => self.cw(),
            _ => self.ccw()
        }
    }

    fn cw_on_cube(&self, side: CubeSide) -> Self {
        match side {
            CubeSide::Right | CubeSide::Back | CubeSide::Bottom => self.ccw(),
            _ => self.cw()
        }
    }

    fn flip_horizontal(&self) -> Self {
        match self {
            Dir::Right => Dir::Left,
            Dir::Left => Dir::Right,
            d => *d,
        }
    }
    fn flip_vertical(&self) -> Self {
        match self {
            Dir::Down => Dir::Up,
            Dir::Up => Dir::Down,
            d => *d,
        }
    }
}

fn turn(input: &str) -> IResult<&str, Instruction> {
    map(alt((char('R'), char('L'))), |c| match c {
        'R' => Instruction::TurnCw,
        _ => Instruction::TurnCcw,
    })(input)
}

fn moven(input: &str) -> IResult<&str, Instruction> {
    map(u32, |n| Instruction::Move(n as usize))(input)
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    all_consuming(many1(alt((turn, moven))))(input).unwrap().1
}

fn wrapping_adjacent(dir: Dir, y: usize, x: usize, h: usize, w: usize) -> (usize, usize) {
    match dir {
        Dir::Up if y > 0 => (y - 1, x),
        Dir::Up => (h-1, x),
        Dir::Down if y < h - 1 => (y + 1, x),
        Dir::Down => (0, x),
        Dir::Left if x > 0 => (y, x - 1),
        Dir::Left => (y, w-1),
        Dir::Right if x < w - 1 => (y, x + 1),
        Dir::Right => (y, 0),
    }
}

fn find_first_grid_pos(dir: Dir, y: usize, x: usize, map: &Array2<char>) -> (usize, usize) {
    let (h, w) = map.dim();
    match dir {
        Dir::Up => (
            (0..h)
                .rev()
                .find(|yy| map[(*yy, x)] != ' ' && map[(*yy, x)] != char::default())
                .unwrap(),
            x,
        ),
        Dir::Down => (
            (0..h)
                .find(|yy| map[(*yy, x)] != ' ' && map[(*yy, x)] != char::default())
                .unwrap(),
            x,
        ),
        Dir::Left => (
            y,
            (0..w)
                .rev()
                .find(|xx| map[(y, *xx)] != ' ' && map[(y, *xx)] != char::default())
                .unwrap(),
        ),
        Dir::Right => (
            y,
            (0..w)
                .find(|xx| map[(y, *xx)] != ' ' && map[(y, *xx)] != char::default())
                .unwrap(),
        ),
    }
}

fn mov(dir: Dir, d: usize, pos: (usize, usize), map: &Array2<char>) -> Option<(usize, usize)> {
    let (y, x) = pos;
    let (h, w) = map.dim();

    let c = map[(y, x)];
    if c == ' ' || c == char::default() {
        return mov(dir, d, find_first_grid_pos(dir, y, x, map), map);
    } else if c == '#' {
        return None;
    } else if d == 0 {
        return Some((y, x));
    }

    Some(mov(dir, d - 1, wrapping_adjacent(dir, y, x, h, w), map).unwrap_or((y, x)))
}

fn part1(map: &Array2<char>, instructions: &Vec<Instruction>) -> usize {
    let mut dir = Dir::Right;
    let mut pos = (
        0,
        map.row(0)
            .indexed_iter()
            .find(|(_, c)| **c == '.')
            .unwrap()
            .0,
    );

    for ins in instructions {
        match ins {
            Instruction::TurnCw => dir = dir.cw(),
            Instruction::TurnCcw => dir = dir.ccw(),
            Instruction::Move(d) => pos = mov(dir, *d, pos, map).unwrap(),
        };
    }

    (pos.0 + 1) * 1000 + (pos.1 + 1) * 4 + dir as usize
}

struct Cube {
    top: Array2<char>,
    front: Array2<char>,
    back: Array2<char>,
    bottom: Array2<char>,
    left: Array2<char>,
    right: Array2<char>,
}

#[derive(Clone, Copy, Debug)]
enum CubeSide {
    Top, Bottom, Left, Right, Front, Back
}

fn flip_horizontal(arr: ArrayView2<char>) -> Array2<char> {
    let mut new = Array2::from_elem(arr.raw_dim(), '.');
    let w = arr.dim().1;

    for c in 0..w {
        new.column_mut(w - c - 1).assign(&arr.column(c).to_owned());
    }

    new
}
fn flip_vertical(arr: ArrayView2<char>) -> Array2<char> {
    let mut new = Array2::from_elem(arr.raw_dim(), '.');
    let h = arr.dim().0;

    for r in 0..h {
        new.row_mut(h - r - 1).assign(&arr.row(r).to_owned());
    }

    new
}
fn rotate_ccw(arr: ArrayView2<char>) -> Array2<char> {
    let mut new = Array2::from_elem(arr.raw_dim(), '.');
    let side = arr.dim().0;

    for ((y, x), c) in arr.indexed_iter() {
        new[(side-1-x, y)] = *c;
    }

    new
}
fn rotate_cw(arr: ArrayView2<char>) -> Array2<char> {
    let mut new = Array2::from_elem(arr.raw_dim(), '.');
    let side = arr.dim().0;

    for ((y, x), c) in arr.indexed_iter() {
        new[(x, side - 1 - y)] = *c;
    }

    new
}

fn cube_adjacent(dir: Dir, cube_side: CubeSide, y: usize, x: usize, cube: &Cube) -> (Dir, CubeSide, usize, usize)
{
    let side_map = match cube_side {
        CubeSide::Top => &cube.top,
        CubeSide::Bottom => &cube.bottom,
        CubeSide::Left => &cube.left,
        CubeSide::Right => &cube.right,
        CubeSide::Front => &cube.front,
        CubeSide::Back => &cube.back,
    };
    let w = side_map.dim().0;

    match dir {
        Dir::Right if x < w - 1  => (dir, cube_side, y, x+1),
        Dir::Left if x > 0  => (dir, cube_side, y, x-1),
        Dir::Down if y < w - 1  => (dir, cube_side, y+1, x),
        Dir::Up if y > 0  => (dir, cube_side, y-1, x),

        Dir::Right => match cube_side {
            CubeSide::Top => (Dir::Down, CubeSide::Right, 0, y),
            CubeSide::Bottom => (Dir::Up, CubeSide::Right, w-1, y),
            CubeSide::Left => (Dir::Right, CubeSide::Front, y, 0),
            CubeSide::Right => (Dir::Left, CubeSide::Front, y, w-1),
            CubeSide::Front => (Dir::Left, CubeSide::Right, y, w-1),
            CubeSide::Back => (Dir::Right, CubeSide::Right, y, 0),
        },

        Dir::Left => match cube_side {
            CubeSide::Top => (Dir::Down, CubeSide::Left, 0, y),
            CubeSide::Bottom => (Dir::Up, CubeSide::Left, w-1, y),
            CubeSide::Left => (Dir::Right, CubeSide::Back, y, 0),
            CubeSide::Right => (Dir::Left, CubeSide::Back, y, w-1),
            CubeSide::Front => (Dir::Left, CubeSide::Left, y, w-1),
            CubeSide::Back => (Dir::Right, CubeSide::Left, y, 0),
        },

        Dir::Up => match cube_side {
            CubeSide::Top => (Dir::Down, CubeSide::Back, 0, x),
            CubeSide::Bottom => (Dir::Up, CubeSide::Back, w-1, x),
            CubeSide::Left => (Dir::Right, CubeSide::Top, x, 0),
            CubeSide::Right => (Dir::Left, CubeSide::Top, x, w-1),
            CubeSide::Front => (Dir::Up, CubeSide::Top, w-1, x),
            CubeSide::Back => (Dir::Down, CubeSide::Top, 0, x),
        },

        Dir::Down => match cube_side {
            CubeSide::Top => (Dir::Down, CubeSide::Front, 0, x),
            CubeSide::Bottom => (Dir::Up, CubeSide::Front, w-1, x),
            CubeSide::Left => (Dir::Right, CubeSide::Bottom, x, 0),
            CubeSide::Right => (Dir::Left, CubeSide::Bottom, x, w-1),
            CubeSide::Front => (Dir::Up, CubeSide::Bottom, w-1, x),
            CubeSide::Back => (Dir::Down, CubeSide::Bottom, 0, x),
        },
    }
}

fn mov_cube(dir: Dir, d: usize, cube_side: CubeSide, pos: (usize, usize), cube: &Cube) -> Option<(Dir, CubeSide, (usize, usize))> {
    let side_map = match cube_side {
        CubeSide::Top => &cube.top,
        CubeSide::Bottom => &cube.bottom,
        CubeSide::Left => &cube.left,
        CubeSide::Right => &cube.right,
        CubeSide::Front => &cube.front,
        CubeSide::Back => &cube.back,
    };

    let (y, x) = pos;
    let c = side_map[(y, x)];
    if c == '#' {
        return None;
    } else if d == 0 {
        return Some((dir, cube_side, (y, x)));
    }

    let (adj_dir, adj_side, adj_y, adj_x) = cube_adjacent(dir, cube_side, y, x, cube);

    Some(mov_cube(adj_dir, d - 1, adj_side, (adj_y, adj_x), cube).unwrap_or((dir, cube_side, (y, x))))
}

fn part2(map: &Array2<char>, instructions: &Vec<Instruction>) -> usize {
    let side = map.row(0).iter().filter(|c| **c == '.' || **c == '#').count();
    let (side, cube) = if side == 4 {
        let top = map.slice(s![0..side, 2*side..3*side]).to_owned();
        let front = map.slice(s![side..2*side, 2*side..3*side]).to_owned();
        let left = map.slice(s![side..2*side, side..2*side]).to_owned();
        let back = flip_horizontal(map.slice(s![side..2*side, 0..side]));
        let bottom = flip_vertical(map.slice(s![2*side..3*side, 2*side..3*side]));
        let right = flip_horizontal(rotate_ccw(map.slice(s![2*side..3*side, 3*side..4*side])).view());
        (side, Cube {
            top, front, left, back, bottom, right
        })
    } else if side == 100 {
        let side = 50;
        let top = map.slice(s![0..side, side..2*side]).to_owned();
        let front = map.slice(s![side..2*side, side..2*side]).to_owned();
        let bottom = flip_vertical(map.slice(s![2*side..3*side, side..2*side]));
        let left = rotate_cw(map.slice(s![2*side..3*side, 0..side]));
        let back = flip_horizontal(rotate_cw(map.slice(s![3*side..4*side, 0..side])).view());
        let right = flip_horizontal(rotate_cw(map.slice(s![0..side, 2*side..3*side])).view());

        (side, Cube {
            top, front, left, back, bottom, right
        })
        
    } else {
        panic!("Unsupported map");
    };

    let mut dir = Dir::Right;
    let mut cube_side = CubeSide::Top;
    let mut pos = (0, 0);

    for ins in instructions {
        match ins {
            Instruction::TurnCw => dir = dir.cw_on_cube(cube_side),
            Instruction::TurnCcw => dir = dir.ccw_on_cube(cube_side),
            Instruction::Move(d) => (dir, cube_side, pos) = mov_cube(dir, *d, cube_side, pos, &cube).unwrap(),
        };
    }

    let (y, x) = pos;

    let (y, x, dir) = if side == 4 {
        match cube_side {
            CubeSide::Top => (y, x + 2*side, dir),
            CubeSide::Bottom => (side - 1 - y + 2*side, x + 2*side, dir.flip_vertical()),
            CubeSide::Left => (y + side, x + side, dir),
            CubeSide::Right => (x + 2*side, side - 1 - y + 3*side, dir.flip_horizontal()),
            CubeSide::Front => (y + side, x + 2*side, dir),
            CubeSide::Back => (y+side, side - x - 1, dir.flip_horizontal()),
        }
    } else if side == 50 {
        match cube_side {
            CubeSide::Top => (y, x + 2*side, dir),
            CubeSide::Front => (y + side, x + 2*side, dir),
            CubeSide::Bottom => (side - 1 - y + 2*side, x + 2*side, dir.flip_vertical()),
            CubeSide::Left => (side - 1 - x + 2*side, y, dir.ccw()),
            CubeSide::Right => (x, y + 2*side, dir.flip_horizontal().ccw()),
            CubeSide::Back => (x + 3*side, y, dir.flip_horizontal().ccw()),
        }
    } else {
        unreachable!()
    };

    (y + 1) * 1000 + (x + 1) * 4 + dir as usize
}

#[derive(Debug)]
enum Instruction {
    TurnCw,
    TurnCcw,
    Move(usize),
}

fn main() -> io::Result<()> {
    let input = aoclib::read_input_string()?;
    let (map, instr_str) = input.split_once("\n\n").unwrap();
    let map = aoclib::read_string_char_matrix(map.trim_end())?;
    let instructions = parse_instructions(instr_str.trim());

    let p1 = part1(&map, &instructions);
    println!("Part 1: {}", p1);

    let p2 = part2(&map, &instructions);
    println!("Part 2: {}", p2);

    Ok(())
}
