use std::collections::HashSet;

use itertools::{iproduct, Itertools, Product};

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

fn find_holes(bytes: &Vec<u8>) -> Vec<usize> {
    bytes
        .iter()
        .enumerate()
        .filter_map(|(i, byte)| {
            if byte == &b'?' {
                return Some(i);
            } else {
                return None;
            }
        })
        .collect_vec()
}

fn hole_groups(holes: Vec<usize>) -> Vec<(usize, usize)> {
    let mut res = Vec::new();
    let mut iter = holes.iter();
    let mut prev = *iter.next().unwrap();
    let mut buf = vec![prev];

    while let Some(&hole_idx) = iter.next() {
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

pub fn part_one(_args: Args) {
    let input = read_input();
    for (bytes, sequence) in input {
        let holes = find_holes(&bytes);
        let groups = hole_groups(holes);
        let combos = groups
            .iter()
            .map(|(_, len)| {
                [b'.', b'#']
                    .repeat(*len)
                    .into_iter()
                    .combinations(*len)
                    .map(|c| String::from_utf8(c).unwrap())
                    .collect::<HashSet<_>>()
            })
            .collect_vec();

        // let mut buf = Vec::new();
        let all_possible_combos = combos.iter().skip(1).fold(
            combos
                .first()
                .unwrap()
                .iter()
                .cloned()
                .map(|i| vec![i])
                .collect_vec(),
            |acc, x| {
                iproduct!(acc, x)
                    .map(|(map, next_value)| {
                        let mut replacement = map.clone();
                        replacement.push(next_value.to_owned());
                        replacement
                    })
                    .collect_vec()
            },
        );

        let input = String::from_utf8(bytes).unwrap();

        for possible_combo in all_possible_combos {
            let mut buf = Vec::new();
            let mut next_in_combo = possible_combo.iter();
            let mut not_holes = input.split("?").filter(|s| s != &"");
            for (c, _grp) in input.bytes().group_by(|i| *i).into_iter() {
                match c {
                    b'.' | b'#' => buf.extend(not_holes.next().unwrap().bytes()),
                    b'?' => buf.extend(next_in_combo.next().unwrap().bytes()),
                    _ => unreachable!(),
                }
            }

            // check buf for the answer here
        }
    }
}

pub fn part_two(_args: Args) {}
