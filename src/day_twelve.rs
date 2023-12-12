use std::collections::HashSet;

use itertools::{iproduct, Itertools};
use rayon::prelude::*;

use crate::Args;

const FILE_CONTENTS: &'static str = include_str!("../inputs/day_twelve.txt");

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

fn hole_groups<T: Iterator<Item = usize>>(mut holes: T) -> Vec<(usize, usize)> {
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

fn is_combination(buf: &Vec<u8>, seq: &Vec<u8>) -> bool {
    let groups = buf
        .split(|c| c == &b'.')
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

fn build_zones(bytes: &[u8]) -> Vec<(usize, usize)> {
    let len = bytes.len();
    let mut res = Vec::new();
    let mut iter = bytes.iter().enumerate();
    while let Some((i, &byte)) = iter.next() {
        match byte {
            b'.' => {}
            b'#' | b'?' => {
                // loop until next '.' or end of bytes
                let start = i;
                let mut end = len;
                while let Some((j, &byte)) = iter.next() {
                    if byte == b'.' {
                        end = j - 1;
                        break;
                    }
                }
                res.push((start, end));
            }
            _ => unreachable!(),
        }
    }

    res
}

fn solve(bytes: Vec<u8>, sequence: Vec<u8>) -> usize {
    let holes = find_holes(&bytes).collect_vec();
    let zones = build_zones(&bytes);

    println!("{:?}\n{:?}", holes, zones);

    unimplemented!()
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
        .into_iter()
        .enumerate()
        .map(|(i, (bytes, sequence))| solve(bytes, sequence))
        .sum::<usize>();
    println!("Sum: {}", sum);
}
