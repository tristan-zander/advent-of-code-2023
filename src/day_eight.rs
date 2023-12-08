use std::{collections::HashMap, str::FromStr};

use itertools::Itertools;

use crate::Args;

const FILE_CONTENTS: &'static str = include_str!("../inputs/day_eight.txt");

#[derive(Debug, Clone)]
enum LeftRight {
    Left,
    Right,
}

impl From<char> for LeftRight {
    fn from(value: char) -> Self {
        match value {
            'L' => LeftRight::Left,
            'R' => LeftRight::Right,
            _ => panic!("Unknown character for left-right list: {}", value),
        }
    }
}

struct DayEight {
    pub directions: Vec<LeftRight>,
    pub locations: HashMap<String, (String, String)>,
}

impl FromStr for DayEight {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction_line, location_lines) = s.split("\n\n").collect_tuple().unwrap();

        Ok(Self {
            directions: direction_line
                .trim_end()
                .chars()
                .map(|d| Into::<LeftRight>::into(d))
                .collect(),
            locations: location_lines
                .lines()
                .map(|line| {
                    let (start, paths) = line.split(" = ").collect_tuple().unwrap();
                    let (left, right) = paths
                        .split(", ")
                        .map(|p| p.replace('(', ""))
                        .map(|p| p.replace(')', ""))
                        .collect_tuple()
                        .unwrap();

                    return (start.to_owned(), (left, right));
                })
                .collect(),
        })
    }
}

pub fn part_one(_args: Args) {
    let input = FILE_CONTENTS.parse::<DayEight>().unwrap();
    let mut current = input.locations.get("AAA").unwrap();
    let mut i = 0;

    loop {
        i += 1;

        let left_or_right = &input.directions[(i - 1) % input.directions.len()];

        let key;
        match left_or_right {
            LeftRight::Left => key = &current.0,
            LeftRight::Right => key = &current.1,
        }

        current = input.locations.get(key).unwrap();

        if key == "ZZZ" {
            break;
        }
    }

    println!("Total steps: {}", i);
}

pub fn part_two(_args: Args) {
    let input = FILE_CONTENTS.parse::<DayEight>().unwrap();

    let counts = input
        .locations
        .iter()
        .filter(|&l| l.0.ends_with('A'))
        .map(|(_key, start)| {
            let mut current = start;
            let mut i = 0;
            loop {
                i += 1;

                let left_or_right = &input.directions[(i - 1) % input.directions.len()];

                let key;
                match left_or_right {
                    LeftRight::Left => key = &current.0,
                    LeftRight::Right => key = &current.1,
                }

                current = input.locations.get(key).unwrap();

                if key.ends_with('Z') {
                    break;
                }
            }

            i
        })
        .reduce(|acc, x| num::integer::lcm(acc, x))
        .unwrap();

    println!("Total steps: {}", counts);
}
