use std::collections::HashMap;

use itertools::Itertools;

use crate::Args;

const FILE_CONTENTS: &'static str = include_str!("../inputs/day_fifteen.txt");

fn input() -> Vec<&'static str> {
    FILE_CONTENTS.split(',').collect_vec()
}

fn hash(inp: &str) -> u8 {
    let mut hash_store = 0_u64;

    for byte in inp.bytes() {
        hash_store += byte as u64;
        hash_store *= 17;
        hash_store %= 256;
    }

    hash_store as u8
}

pub fn part_one(_args: Args) {
    let input = input();
    let sum = input.iter().map(|&s| hash(s) as u64).sum::<u64>();
    println!("Sum: {}", sum);
}
pub fn part_two(_args: Args) {
    let input = input();
    let mut boxes = HashMap::<u8, Vec<(&str, u8)>>::new();

    for step in input {
        let (label, lens) = if step.contains('-') {
            (step.split('-').next().unwrap(), None)
        } else {
            let (label, right) = step.split('=').collect_tuple::<(&str, &str)>().unwrap();
            (label, Some(right.parse::<u8>().unwrap()))
        };
        let hash = hash(label);

        if lens.is_none() {
            if let Some(prev) = boxes.get_mut(&hash) {
                if let Some(index) = prev
                    .iter()
                    .enumerate()
                    .filter_map(|(i, (l, _v))| {
                        if *l == label {
                            return Some(i);
                        }
                        None
                    })
                    .next()
                {
                    prev.remove(index);
                }
            }
            continue;
        }

        let lens = lens.unwrap();
        if let Some(prev) = boxes.get_mut(&hash) {
            if let Some(idx) = prev
                .iter()
                .enumerate()
                .filter_map(|(i, (l, _))| {
                    if *l == label {
                        return Some(i);
                    }
                    None
                })
                .next()
            {
                prev[idx] = (label, lens);
            } else {
                prev.push((label, lens));
            }
        } else {
            boxes.insert(hash, vec![(label, lens)]);
        }
    }

    println!("{:#?}", boxes);

    let sum = boxes
        .into_iter()
        .map(|(key, val)| {
            val.iter()
                .enumerate()
                .map(|(j, (_, val))| (key as usize + 1) * ((j + 1) * (*val as usize)))
                .sum::<usize>()
        })
        .sum::<usize>();

    println!("Sum: {}", sum);
}
