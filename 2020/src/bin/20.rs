use itertools::Itertools;
use std::collections::HashMap;
use std::io;
use aoc2020::UnwrapOptionIterator;

type Grid = Vec<Vec<bool>>;

fn grid_rotations(g: &Grid) -> Vec<Grid> {
    let r1 = rotate(&g);
    let r2 = rotate(&r1);
    let flipped = flip_vertical(&g);
    let fr1 = rotate(&flipped);
    let fr2 = rotate(&fr1);
    vec![
        g.clone(),
        rotate(&r2),
        r2,
        r1,
        flipped,
        rotate(&fr2),
        fr2,
        fr1,
    ]
}

fn tile_rotations(t: &Tile) -> Vec<Tile> {
    grid_rotations(&t.grid)
        .into_iter()
        .map(|g| Tile { id: t.id, grid: g })
        .collect_vec()
}

fn stitch(tiles: &Vec<Vec<Tile>>) -> Grid {
    let tile_side = &tiles[0][0].grid.len();
    let cropped_side = tile_side - 2;
    let full_side = tiles.len() * cropped_side;
    let mut grid = vec![vec![false; full_side]; full_side];
    for ty in 0..tiles.len() {
        for tx in 0..tiles.len() {
            for y in 0..cropped_side {
                for x in 0..cropped_side {
                    grid[(ty * cropped_side) + y][(tx * cropped_side) + x] =
                        tiles[ty][tx].grid[1 + y][1 + x];
                }
            }
        }
    }

    grid
}

fn place_tile<'a>(
    tiles: &'a HashMap<usize, Tile>,
    mut ids_remaining: Vec<usize>,
    mut current_grid: Vec<Vec<Option<&'a Tile>>>,
    place: &'a Tile,
    x: usize,
    y: usize,
) -> Option<Vec<Vec<Tile>>> {
    assert!(&current_grid[y][x].is_none());
    let going_right =
        (x > 0 && current_grid[y][x - 1].is_some()) || (x == 0 && current_grid[y][x + 1].is_none());
    let side_len = current_grid.len();

    if y > 0 && current_grid[y - 1][x].unwrap().bottom_edge() != place.top_edge() {
        return None;
    }
    if x > 0 && going_right && current_grid[y][x - 1].unwrap().right_edge() != place.left_edge() {
        return None;
    }
    if x < side_len - 1
        && !going_right
        && current_grid[y][x + 1].unwrap().left_edge() != place.right_edge()
    {
        return None;
    }

    // This placement is valid. Now recursively place next tiles.
    current_grid[y][x] = Some(place);
    ids_remaining.swap_remove(ids_remaining.iter().position(|id| *id == place.id)?);

    if ids_remaining.is_empty() {
        return Some(
            current_grid
                .into_iter()
                .map(|l| l.into_iter().unwrap_options().cloned().collect_vec())
                .collect_vec(),
        );
    }

    let (next_x, next_y) = match going_right {
        true if x < side_len - 1 => (x + 1, y),
        false if x > 0 => (x - 1, y),
        true => (x, y + 1),
        false => (x, y + 1),
    };
    for id in &ids_remaining {
        let t = &tiles[id];
        for rot in tile_rotations(t) {
            let res = place_tile(
                tiles,
                ids_remaining.clone(),
                current_grid.clone(),
                &rot,
                next_x,
                next_y,
            );
            if res.is_some() {
                return res;
            }
        }
    }

    None
}

fn part1(input: &HashMap<usize, Tile>) -> Option<(usize, Grid)> {
    let side_len = (input.len() as f64).sqrt() as usize;
    let grid = vec![vec![None; side_len]; side_len];

    // Find corners
    let mut side_counts = HashMap::new();
    for t in input.values() {
        for x in [t.bottom_edge(), t.top_edge(), t.left_edge(), t.right_edge()].iter_mut() {
            *side_counts.entry(x.clone()).or_insert(0) += 1;
            x.reverse();
            *side_counts.entry(x.clone()).or_insert(0) += 1;
        }
    }

    let corner = input.values()
        .filter(|t| t.edges().iter().map(|e| side_counts[e]).sum::<u8>() <= 6)
        .next()?;

    let ids_remaining = input.keys().copied().collect_vec();
    for rot in tile_rotations(corner) {
        let res = place_tile(input, ids_remaining.clone(), grid.clone(), &rot, 0, 0);
        if let Some(res) = res {
            let part1res = res[0][0].id
                * res[0][side_len - 1].id
                * res[side_len - 1][0].id
                * res[side_len - 1][side_len - 1].id;
            return Some((part1res, stitch(&res)));
        }
    }

    None
}

fn part2(input: Grid) -> usize {
    //                   # 
    // #    ##    ##    ###
    //  #  #  #  #  #  #  
    let monster_pixels = [
        (18, 0),
        (0, 1),
        (5, 1),
        (6, 1),
        (11, 1),
        (12, 1),
        (17, 1),
        (18, 1),
        (19, 1),
        (1, 2),
        (4, 2),
        (7, 2),
        (10, 2),
        (13, 2),
        (16, 2),
    ];
    let mw = 20;
    let mh = 3;

    for mut rot in grid_rotations(&input) {
        let zero_coords = (0..=rot.len() - mw + 1)
            .cartesian_product(0..=rot.len() - mh + 1)
            .filter(|(xx, yy)| monster_pixels.iter().all(|(mx, my)| rot[my + yy][mx + xx]))
            .flat_map(|(xx, yy)| {
                monster_pixels
                    .iter()
                    .map(move |(mx, my)| (mx + xx, my + yy))
            })
            .collect_vec();
        if zero_coords.len() == 0 {
            continue;
        }

        for (xx, yy) in zero_coords {
            rot[yy][xx] = false;
        }

        return rot
            .into_iter()
            .flat_map(|l| l.into_iter())
            .filter(|x| *x)
            .count();
    }

    unreachable!("Sea monsters not found")
}

fn rotate(grid: &Grid) -> Grid {
    let mut new_grid: Grid = Vec::new();
    for old_x in 0..grid.len() {
        let mut l = Vec::new();
        for old_y in (0..grid.len()).rev() {
            l.push(grid[old_y][old_x]);
        }
        new_grid.push(l);
    }

    new_grid
}

fn flip_vertical(grid: &Grid) -> Grid {
    let mut new = grid.clone();
    new.reverse();
    new
}

#[derive(Clone)]
struct Tile {
    id: usize,
    grid: Grid,
}

impl Tile {
    fn left_edge(&self) -> Vec<bool> {
        (0..10).map(|i| self.grid[i][0]).collect()
    }

    fn right_edge(&self) -> Vec<bool> {
        (0..10).map(|i| self.grid[i][9]).collect()
    }

    fn top_edge(&self) -> Vec<bool> {
        self.grid[0].clone()
    }

    fn bottom_edge(&self) -> Vec<bool> {
        self.grid[9].clone()
    }

    fn edges(&self) -> [Vec<bool>; 4] {
        [self.left_edge(), self.right_edge(), self.top_edge(), self.bottom_edge()]
    }
}

fn read_tile(inp: &str) -> Option<Tile> {
    let mut line_iter = inp.lines();

    let id: usize = *aoc2020::read_ints_from_string(line_iter.next()?).first()?;

    let grid = line_iter
        .map(|l| l.chars().map(|c| c == '#').collect_vec())
        .collect_vec();

    Some(Tile { id, grid })
}

fn read_input() -> io::Result<HashMap<usize, Tile>> {
    Ok(aoc2020::read_stdin_to_string()?
        .trim()
        .split("\n\n")
        .map(|s| read_tile(s).unwrap())
        .map(|t| (t.id, t))
        .collect())
}

fn main() -> io::Result<()> {
    let input = read_input()?;

    let p1 = part1(&input).unwrap();
    println!("Part 1: {}", p1.0);

    let p2 = part2(p1.1);
    println!("Part 2: {}", p2);

    Ok(())
}

