use arrayvec::ArrayVec;
use ndarray::{s, Array2};
use std::io;

const WIDTH: usize = 7;
const MAX_HEIGHT: usize = 10_000;

#[derive(Clone, Copy, Debug, PartialEq, Hash)]
enum BlockShape {
    Minus,
    Plus,
    ReverseL,
    I,
    Square,
}
impl BlockShape {
    fn next(&self) -> BlockShape {
        match self {
            BlockShape::Minus => BlockShape::Plus,
            BlockShape::Plus => BlockShape::ReverseL,
            BlockShape::ReverseL => BlockShape::I,
            BlockShape::I => BlockShape::Square,
            BlockShape::Square => BlockShape::Minus,
        }
    }
    fn height(&self) -> usize {
        match self {
            BlockShape::Minus => 1,
            BlockShape::Plus => 3,
            BlockShape::ReverseL => 3,
            BlockShape::I => 4,
            BlockShape::Square => 2,
        }
    }
    fn width(&self) -> usize {
        match self {
            BlockShape::Minus => 4,
            BlockShape::Plus => 3,
            BlockShape::ReverseL => 3,
            BlockShape::I => 1,
            BlockShape::Square => 2,
        }
    }
    fn collides(&self, grid: &Array2<u8>, x: usize, y: usize) -> bool {
        match self {
            BlockShape::Minus => grid.slice(s![y, x..x + 4]).sum() > 0,
            BlockShape::Plus => {
                grid.slice(s![y + 1, x..x + 3]).sum() > 0
                    || grid[(y, x + 1)] > 0
                    || grid[(y + 2, x + 1)] > 0
            }
            BlockShape::ReverseL => {
                grid.slice(s![y + 2, x..x + 3]).sum() > 0
                    || grid.slice(s![y..y + 2, x + 2]).sum() > 0
            }
            BlockShape::I => grid.slice(s![y..y + 4, x]).sum() > 0,
            BlockShape::Square => grid.slice(s![y..y + 2, x..x + 2]).sum() > 0,
        }
    }
    fn mark(&self, grid: &mut Array2<u8>, x: usize, y: usize) {
        assert!(y < MAX_HEIGHT);
        assert!(x < WIDTH);
        match self {
            BlockShape::Minus => grid.slice_mut(s![y, x..x + 4]).fill(1),
            BlockShape::Plus => {
                grid.slice_mut(s![y + 1, x..x + 3]).fill(1);
                grid[(y, x + 1)] = 1;
                grid[(y + 2, x + 1)] = 1;
            }
            BlockShape::ReverseL => {
                grid.slice_mut(s![y + 2, x..x + 3]).fill(1);
                grid.slice_mut(s![y..y + 2, x + 2]).fill(1);
            }
            BlockShape::I => grid.slice_mut(s![y..y + 4, x]).fill(1),
            BlockShape::Square => grid.slice_mut(s![y..y + 2, x..x + 2]).fill(1),
        }
    }
}

#[derive(Debug)]
struct Block {
    shape: BlockShape,
    x: usize,
    y: usize,
}

impl Block {
    fn new(shape: BlockShape, max_y: usize) -> Block {
        Block {
            y: max_y - shape.height(),
            x: 2,
            shape,
        }
    }

    fn movex(mut self, dx: isize, grid: &Array2<u8>) -> Block {
        let new_x = self.x as isize + dx;
        if new_x < 0
            || new_x as usize + self.shape.width() > WIDTH
            || self.shape.collides(grid, new_x as usize, self.y)
        {
            self
        } else {
            self.x = new_x as usize;
            self
        }
    }

    fn move_down(mut self, grid: &Array2<u8>) -> (Block, bool) {
        let new_y = self.y + 1;
        if new_y + self.shape.height() > MAX_HEIGHT || self.shape.collides(grid, self.x, new_y) {
            (self, false)
        } else {
            self.y = new_y;
            (self, true)
        }
    }

    fn mark(&self, grid: &mut Array2<u8>) {
        self.shape.mark(grid, self.x, self.y);
    }
}

#[derive(Clone)]
struct State {
    grid: Array2<u8>,
    next_shape: BlockShape,
    next_wind: usize,
    max_y: usize,
}
impl State {
    fn new() -> State {
        State {
            grid: Array2::zeros((MAX_HEIGHT, WIDTH)),
            next_shape: BlockShape::Minus,
            next_wind: 0,
            max_y: MAX_HEIGHT,
        }
    }
    fn tick(&mut self, winds: &Vec<char>) {
        let current_shape = self.next_shape;
        let mut block = Block::new(current_shape, self.max_y - 3);

        self.next_shape = current_shape.next();

        loop {
            let current_wind = self.next_wind;
            self.next_wind = (current_wind + 1) % winds.len();
            let dir = winds[current_wind];
            let dx = match dir {
                '<' => -1,
                '>' => 1,
                _ => panic!("Invalid jet stream dir"),
            };

            let moved;
            (block, moved) = block.movex(dx, &self.grid).move_down(&self.grid);

            if !moved {
                block.mark(&mut self.grid);
                self.max_y = self.max_y.min(block.y);
                break;
            }
        }
    }

    fn distances_from_above(&self) -> ArrayVec<usize, WIDTH> {
        let view = self.grid.slice(s![self.max_y.., ..]);
        view.columns()
            .into_iter()
            .map(|c| c.iter().take_while(|x| **x == 0).count())
            .collect()
    }
}

fn part1(input: &Vec<char>) -> usize {
    let mut state = State::new();

    for _ in 0..2022 {
        state.tick(input);
    }

    MAX_HEIGHT - state.max_y
}


fn part2(input: &Vec<char>) -> usize {
    let mut tortoise = State::new();
    let mut hare = State::new();

    tortoise.tick(input);
    hare.tick(input);
    hare.tick(input);

    let mut i = 1;
    while tortoise.next_wind != hare.next_wind
        || tortoise.next_shape != hare.next_shape
        || tortoise.distances_from_above() != hare.distances_from_above()
    {
        i += 1;
        tortoise.tick(input);
        hare.tick(input);
        hare.tick(input);
    }

    let loop_start_y = tortoise.max_y;

    let mut loop_size = 1;
    tortoise.tick(input);
    while tortoise.next_wind != hare.next_wind
        || tortoise.next_shape != hare.next_shape
        || tortoise.distances_from_above() != hare.distances_from_above()
    {
        loop_size += 1;
        tortoise.tick(input);
    }

    let dy = tortoise.max_y.abs_diff(loop_start_y);

    let iters = 1_000_000_000_000usize - i;
    let skips = iters / loop_size;
    let rem_iters = iters - (skips * loop_size);

    let dy_skip = dy * skips;

    let y_before_rem =  tortoise.max_y;
    for _ in 0..rem_iters {
        tortoise.tick(input);
    }

    let dy_rem = tortoise.max_y.abs_diff(y_before_rem);
    
    (MAX_HEIGHT - loop_start_y) + dy_skip + dy_rem
}

fn main() -> io::Result<()> {
    let input = aoclib::read_input_string()?;
    let input = input.trim().chars().collect();

    let p1 = part1(&input);
    println!("Part 1: {}", p1);

    let p2 = part2(&input);
    println!("Part 2: {}", p2);

    Ok(())
}
