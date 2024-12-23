use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    io,
};

struct Link<'a>(&'a str, &'a str);

struct Group<'a>(&'a str, &'a str, &'a str);
impl Eq for Group<'_> {}
impl PartialEq for Group<'_> {
    fn eq(&self, other: &Self) -> bool {
        let mut a = [self.0, self.1, self.2];
        let mut b = [other.0, other.1, other.2];
        a.sort();
        b.sort();
        a == b
    }
}
impl std::hash::Hash for Group<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let mut x = [self.0, self.1, self.2];
        x.sort();
        x.hash(state);
    }
}

fn part1(input: &Vec<Link>) -> usize {
    let input: HashMap<&str, HashSet<&str>> = input
        .iter()
        .flat_map(|l| [(l.0, l.1), (l.1, l.0)])
        .into_grouping_map()
        .collect();

    let mut groups: HashSet<Group> = Default::default();
    for (a, others) in &input {
        for (b, c) in others.iter().tuple_combinations() {
            if input[b].contains(c) {
                groups.insert(Group(a, b, c));
            }
        }
    }

    groups
        .iter()
        .filter(|Group(a, b, c)| a.starts_with('t') || b.starts_with('t') || c.starts_with('t'))
        .count()
}

fn part2(input: &Vec<Link>) -> String {
    let neighbours: HashMap<&str, HashSet<&str>> = input
        .iter()
        .flat_map(|l| [(l.0, l.1), (l.1, l.0)])
        .into_grouping_map()
        .collect();

    let mut max: HashSet<&str> = Default::default();
    bron_kerbosch(
        &mut max,
        &mut Default::default(),
        neighbours.keys().copied().collect(),
        Default::default(),
        &neighbours,
    );
    return max.iter().sorted().join(",");

    fn bron_kerbosch<'a>(
        max: &mut HashSet<&'a str>,
        r: &mut HashSet<&'a str>,
        mut p: HashSet<&'a str>,
        mut x: HashSet<&'a str>,
        neighbours: &HashMap<&'a str, HashSet<&'a str>>,
    ) {
        if p.is_empty() && x.is_empty() {
            if r.len() > max.len() {
                *max = r.clone();
            }
            return;
        }

        let u = *p.union(&x).max_by_key(|k| neighbours[*k].len()).unwrap();

        let pp: Vec<_> = p.difference(&neighbours[u]).copied().collect();
        for v in pp {
            r.insert(v);
            let n = &neighbours[v];
            bron_kerbosch(
                max,
                r,
                p.intersection(n).copied().collect(),
                x.intersection(n).copied().collect(),
                neighbours,
            );
            r.remove(v);
            p.remove(v);
            x.insert(v);
        }
    }
}

fn parse_input(input: &[String]) -> Vec<Link<'_>> {
    input
        .iter()
        .map(|l| {
            let (a, b) = aoclib::split_to_tuple2(l, "-").unwrap();
            Link(a, b)
        })
        .collect()
}

fn main() -> io::Result<()> {
    let input = aoclib::read_input_lines()?;
    let links = parse_input(&input);

    let p1 = part1(&links);
    println!("Part 1: {}", p1);

    let p2 = part2(&links);
    println!("Part 2: {}", p2);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_real_input() {
        let input = aoclib::read_file_lines(aoclib::get_test_input_file!(23)).unwrap();
        let links = parse_input(&input);

        let p1 = part1(&links);
        assert_eq!(p1, 1485);

        let p2 = part2(&links);
        assert_eq!(p2, "cc,dz,ea,hj,if,it,kf,qo,sk,ug,ut,uv,wh");
    }
}
