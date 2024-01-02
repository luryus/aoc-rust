use std::{io, collections::{HashMap, HashSet, VecDeque}};

type EdgeMap<'a> = HashMap<&'a str, HashSet<&'a str>>;


fn shortest_path_edge_counts<'a>(edges: &EdgeMap<'a>) -> HashMap<(&'a str, &'a str), usize> {

    let mut res = HashMap::new();

    for v in edges.keys() {
        //println!("v: {v}");
        let mut prev: HashMap<&str, Option<&str>> = HashMap::new();

        let mut q = VecDeque::new();
        q.push_back((v, None));

        while let Some((u, from)) = q.pop_front() {
            if !prev.contains_key(u) {
                prev.insert(u, from);
            }

            for e in &edges[u] {
                if !prev.contains_key(e) {
                    q.push_back((e, Some(u)));
                }
            }
        }

        for u in prev.keys() {
            let mut u = *u;
            while let Some(p) = prev[u] {
                let tup = if u > p { (p, u) } else { (u, p) };
                *res.entry(tup).or_default() += 1;
                u = p;
            }
        }

    }

    res

}

fn part1(input: &Vec<String>) -> usize {
    let mut edges: EdgeMap = HashMap::default();
    
    for l in input {
        let (l, r) = aoclib::split_to_tuple2(l, ": ").unwrap();
        let rs = r.split(' ');

        for r in rs {
            edges.entry(l).or_default().insert(r);
            edges.entry(r).or_default().insert(l);
        }
    }

    for _ in 0..3 {
        let edge_counts = shortest_path_edge_counts(&edges);
        let most_common = edge_counts.into_iter().max_by_key(|(_, count)| *count).unwrap();
        println!("Most common edge: {:?}", most_common);

        let (a, b) = most_common.0;

        edges.get_mut(a).unwrap().remove(b);
        edges.get_mut(b).unwrap().remove(a);
    }

    // now the graph should be in two separate groups
    let mut vis: HashSet<&str> = HashSet::new();
    let mut q: VecDeque<&str> = VecDeque::new();
    q.push_back(edges.keys().next().unwrap());
    while let Some(v) = q.pop_front() {
        if !vis.insert(v) {
            continue;
        }
        for u in &edges[v] {
            if !vis.contains(u) {
                q.push_back(u);
            } 
        }
    }

    println!("Vis: {}, others: {}", vis.len(), edges.len() - vis.len());

    vis.len() * (edges.len() - vis.len())
}

fn main() -> io::Result<()> {
    let input = aoclib::read_input_lines()?;

    let p1 = part1(&input);
    println!("Part 1: {}", p1);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_real_input() {
        let input = aoclib::read_file_lines(aoclib::get_test_input_file!(25)).unwrap();

        let p1 = part1(&input);
        assert_eq!(p1, 507626);
    }
}
