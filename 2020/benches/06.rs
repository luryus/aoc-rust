use std::collections::BTreeSet;
use itertools::Itertools;


pub fn part2(input: &str) -> usize {
    let groups = input.split("\n\n");

    groups
        .map(|g| {
            g.lines()
                .map(|l| l.chars().collect::<BTreeSet<_>>())
                .fold1(|acc, x| acc.intersection(&x).copied().collect::<BTreeSet<_>>())
                .map_or(0, |s| s.len())
        })
        .sum()
}


pub fn part2_vec(input: &str) -> usize {
    let groups = input.split("\n\n");

    groups
        .map(|g| {
            g.lines()
                .map(|l| l.chars().collect::<Vec<_>>())
                .fold1(|acc, x|
                    acc.into_iter().filter(|c| x.contains(c)).collect::<Vec<_>>())
                .map_or(0, |s| s.len())
        })
        .sum()
}

pub fn part2_vec_mut(input: &str) -> usize {
    let groups = input.split("\n\n");

    groups
        .map(|g| {
            g.lines()
                .map(|l| l.chars().collect_vec())
                .fold1(|mut acc, x| {
                    acc.retain(|c| x.contains(c)); acc
                })
                .map_or(0, |s| s.len())
        })
        .sum()
}

use criterion::{black_box, criterion_group, criterion_main, Criterion};


pub fn criterion_benchmark(c: &mut Criterion) {
    let inp = std::fs::read_to_string("src/bin/inputs/06.txt").unwrap();
    c.bench_function("part2 btreeset", |b| b.iter(|| part2(black_box(&inp))));
    c.bench_function("part2 vec", |b| b.iter(|| part2_vec(black_box(&inp))));
    c.bench_function("part2 vec mut", |b| b.iter(|| part2_vec_mut(black_box(&inp))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);