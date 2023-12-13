use std::{sync::Arc, time::Instant};

use dashmap::DashMap;
use itertools::{repeat_n, Itertools};
use rayon::prelude::*;

use crate::Args;

const FILE_CONTENTS: &'static str = include_str!("../inputs/day_twelve.txt");

lazy_static::lazy_static! {
    static ref PERMUTATIONS: DashMap<usize, Arc<Vec<String>>> = DashMap::new();
}

fn read_input() -> Vec<(Vec<u8>, Vec<u8>)> {
    FILE_CONTENTS
        .lines()
        .map(|l| {
            let (left, right) = l.split(' ').collect_tuple().unwrap();
            (
                left.bytes().collect_vec(),
                right
                    .split(',')
                    .map(|c| c.parse::<u8>().unwrap())
                    .collect_vec(),
            )
        })
        .collect_vec()
}

fn find_holes(bytes: &[u8]) -> impl Iterator<Item = usize> + '_ {
    bytes.iter().enumerate().filter_map(|(i, byte)| {
        if byte == &b'?' {
            return Some(i);
        } else {
            return None;
        }
    })
}

fn hole_groups(holes: &[usize]) -> Vec<(usize, usize)> {
    let mut holes = holes.iter().cloned();
    let mut res = Vec::new();
    let mut prev = holes.next().unwrap();
    let mut buf = vec![prev];

    while let Some(hole_idx) = holes.next() {
        if hole_idx - prev != 1 {
            let start_idx = buf[0];
            let len = buf.len();
            res.push((start_idx, len));
            buf.clear();
        }

        buf.push(hole_idx);
        prev = hole_idx;
    }

    if buf.len() != 0 {
        let start_idx = buf[0];
        let len = buf.len();
        res.push((start_idx, len));
        buf.clear();
    }

    res
}

fn is_combination(buf: &str, seq: &Vec<u8>) -> bool {
    let groups = buf
        .split('.')
        .filter_map(|grp| {
            if grp.len() == 0 {
                return None;
            }
            return Some(grp.len() as u8);
        })
        .collect_vec();

    if groups.len() != seq.len() {
        return false;
    }

    groups
        .into_iter()
        .zip(seq.iter())
        .all(|(left, right)| left == *right)
}

fn get_permutations(len: usize) -> Arc<Vec<String>> {
    if let Some(perm) = PERMUTATIONS.get(&len) {
        return perm.clone();
    }

    let perms = repeat_n([".", "#"], len)
        .multi_cartesian_product()
        .map(|perms| perms.join(""))
        .collect_vec();

    PERMUTATIONS.insert(len, Arc::new(perms));
    let perms = PERMUTATIONS.get(&len).unwrap();

    perms.clone()
}

fn solve(bytes: Vec<u8>, sequence: Vec<u8>) -> usize {
    let holes = find_holes(&bytes).collect_vec();
    let groups = hole_groups(&holes);

    let permutation_ptrs = groups
        .iter()
        .map(|(_start, len)| {
            let permutations = get_permutations(*len);
            permutations
        })
        .collect_vec();

    let unique_combos_per_group = permutation_ptrs
        .iter()
        .map(|p| p.as_slice())
        .multi_cartesian_product();

    let mut final_string = String::from_utf8(bytes.clone()).unwrap();
    let mut sum = 0;
    for group in unique_combos_per_group {
        for (replacement, (start, len)) in group.into_iter().zip(groups.iter().cloned()) {
            final_string.replace_range(start..start + len, &replacement);
        }

        if is_combination(&final_string, &sequence) {
            sum += 1;
        }
    }

    sum
}

pub fn part_one(_args: Args) {
    let input = read_input();
    let sum = input
        .into_par_iter()
        .map(|(bytes, sequence)| solve(bytes, sequence))
        .sum::<usize>();
    println!("Sum: {}", sum);
}

pub fn part_two(_args: Args) {
    let input = read_input()
        .into_iter()
        .map(|(bytes, seq)| (bytes.repeat(5), seq.repeat(5)))
        .collect_vec();
    let sum = input
        .into_par_iter()
        .enumerate()
        .map(|(i, (bytes, sequence))| {
            println!("Solving {}", i);
            let start = Instant::now();
            let solved = solve(bytes, sequence);
            println!("{} solved in {}", i, (start - Instant::now()).as_millis());
            solved
        })
        .sum::<usize>();
    println!("Sum: {}", sum);
}
