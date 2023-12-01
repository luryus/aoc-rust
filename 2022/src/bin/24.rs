use std::{io, collections::{VecDeque, HashSet, HashMap}, mem::swap};
use arrayvec::ArrayVec;
use ndarray::{Array2, s};

type Coord = (usize, usize);
type Movement = (i32, i32);

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
struct Blizzard { pos: Coord, dir: Movement }

impl Blizzard {
    fn mov(self, h: usize, w: usize) -> Self {
        let y = (self.pos.0 as i32 + self.dir.0) as usize;
        let x = (self.pos.1 as i32 + self.dir.1) as usize;

        let y = if y == 0 { h - 2 } else if y == h-1 { 1 } else { y };
        let x = if x == 0 { w - 2 } else if x == w-1 { 1 } else { x };

        Blizzard { pos: (y, x), dir: self.dir }
    }
}

fn run(input: &[Blizzard], h: usize, w: usize, trips: usize) -> usize {
    let mut q = VecDeque::new();
    let mut visited = HashSet::new();
    q.push_back(((0, 1), 0, 0));

    let mut blizzards: HashMap<_, ArrayVec<_, 10>> = HashMap::new();
    for b in input.iter().cloned() {
        blizzards.entry(b.pos).or_default().push(b);
    }
    let mut tmp_blizzards: HashMap<_, ArrayVec<_, 10>> = HashMap::with_capacity(blizzards.len());
    let mut current_round = 0;

    while let Some(state) = q.pop_front() {
        if !visited.insert(state) {
            continue;
        }
        let ((y, x), r, mut trip) = state;

        if y == h-1 && x == w-2 {
            if trip == trips - 1 {
                return r;
            } else if trip % 2 == 0 {
                trip += 1;
            }
        } else if trip % 2 == 1 && y == 0 && x == 1 {
            trip += 1;
        }

        if r + 1 > current_round {
            for (_, bs) in blizzards.drain() {
                for b in bs {
                    let nb = b.mov(h, w);
                    tmp_blizzards.entry(nb.pos).or_default().push(nb);
                }
            }
            swap(&mut blizzards, &mut tmp_blizzards);
            
            current_round += 1;
        }

        if !blizzards.contains_key(&(y, x)) {
            q.push_back(((y, x), r+1, trip));
        }
        if x > 1 && y != h-1 && !blizzards.contains_key(&(y, x-1)) {
            q.push_back(((y, x-1), r+1, trip));
        }
        if (y > 1 || (x == 1 && y > 0)) && !blizzards.contains_key(&(y-1, x)) {
            q.push_back(((y-1, x), r+1, trip));
        }
        if x < w - 2 && y != 0 && !blizzards.contains_key(&(y, x+1)) {
            q.push_back(((y, x+1), r+1, trip));
        }
        if (y < h - 2 || (x == w-2 && y < h-1)) && !blizzards.contains_key(&(y+1, x)) {
            q.push_back(((y+1, x), r+1, trip));
        }
    }

    unreachable!()
}

fn parse_input(arr: Array2<char>) -> (Vec<Blizzard>, usize, usize) {
    let (h, w) = arr.dim();
    let arr = arr.slice(s![1..h-1, 1..w-1]);

    (arr.indexed_iter().filter_map(|((y, x), c)| {
        let dir = match c {
            '<' => (0, -1),
            '>' => (0, 1),
            '^' => (-1, 0),
            'v' => (1, 0),
            _ => return None
        };
        Some(Blizzard { 
            pos: (y+1, x+1),
            dir
        })
    })
    .collect(), h, w)

}

fn main() -> io::Result<()> {
    let (input, h, w) = parse_input(aoc2022::read_input_char_matrix()?);

    let p1 = run(&input, h, w, 1);
    println!("Part 1: {}", p1);

    let p2 = run(&input, h, w, 3);
    println!("Part 2: {}", p2);

    Ok(())
}
