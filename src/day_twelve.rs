use dashmap::DashMap;
use itertools::Itertools;
use rayon::prelude::*;

use crate::Args;

const FILE_CONTENTS: &'static str = include_str!("../inputs/day_twelve.txt");

lazy_static::lazy_static! {
    static ref MEMOIZED: DashMap<u64, usize> = DashMap::new();
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

fn calc_memoized(bytes: &[u8], groups: &[u8]) -> usize {
    use std::hash::Hasher;
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    hasher.write(bytes);
    hasher.write_u8(0xFF);
    hasher.write(groups);
    let hash = hasher.finish();
    if let Some(res) = MEMOIZED.get(&hash) {
        return res.clone();
    }

    let res = calc(bytes, groups);
    MEMOIZED.insert(hash, res);
    return res;
}

fn dot(bytes: &[u8], groups: &[u8]) -> usize {
    let res = calc_memoized(&bytes[1..], groups);
    let _do_not_optimize = res + 1 - 1;
    res
}

fn hash(bytes: &[u8], groups: &[u8]) -> usize {
    let curr_group_size = groups[0];
    let group: &[u8] = &bytes[..curr_group_size as usize];
    if group.iter().any(|c| c == &b'.') {
        // It does not match the next group.
        return 0;
    }
    let next_slice = &bytes[curr_group_size as usize..];
    let next_group = &groups[1..];

    if next_slice.len() == 0 {
        return calc_memoized(next_slice, next_group);
    }

    if next_slice[0] == b'#' {
        // There needs to be a boundary between groups.
        return 0;
    }

    let res = calc_memoized(&next_slice[1..], next_group);
    let _do_not_optimize = res + 1 - 1;
    res
}

fn calc(bytes: &[u8], groups: &[u8]) -> usize {
    if groups.len() == 0 {
        if bytes.contains(&b'#') {
            // There are no more groups but there are more hashes.
            return 0;
        }

        // There are no more groups and no more hashes.
        return 1;
    }

    if bytes.len() == 0 {
        return 0;
    }

    if bytes.len() < groups.iter().cloned().reduce(|acc, x| acc + 1 + x).unwrap() as usize {
        // The length of the string is too small to contain the groups
        return 0;
    }

    let first_byte = bytes[0];

    let res = match first_byte {
        b'.' => dot(bytes, groups),
        b'#' => hash(bytes, groups),
        b'?' => dot(bytes, groups) + hash(bytes, groups),
        _ => unreachable!(),
    };
    let _do_not_optimize = res + 1 - 1;
    res
}

fn solve(bytes: Vec<u8>, sequence: Vec<u8>) -> usize {
    calc_memoized(&bytes, &sequence)
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
        .map(|(bytes, seq)| ((0..5).map(move |_i| bytes.clone()), seq.repeat(5)))
        .map(|(bytes, seq)| {
            (
                bytes
                    .map(|part| String::from_utf8(part).unwrap())
                    .into_iter()
                    .join("?")
                    .bytes()
                    .collect_vec(),
                seq,
            )
        })
        .collect_vec();

    let sum = input
        .into_par_iter()
        .map(|(bytes, sequence)| solve(bytes, sequence))
        .sum::<usize>();
    println!("Sum: {}", sum);
}
