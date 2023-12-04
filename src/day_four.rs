use std::{collections::HashSet, str::FromStr};

use crate::Args;

const FILE_CONTENTS: &'static str = include_str!("../inputs/day_four.txt");

struct Card {
    pub _index: u32,
    pub points: u32,
}

impl Card {
    fn get_points_from_scratchcard(scratchcard: &str) -> u32 {
        let mut sides = scratchcard.split('|').map(|s| s.trim());
        let answer_side = sides.next().unwrap();
        let numbers_side = sides.next().unwrap();

        let winning_numbers = answer_side
            .split(' ')
            .filter(|n| !n.is_empty())
            .map(|n| n.trim())
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<HashSet<_>>();
        let numbers = numbers_side
            .split(' ')
            .map(|n| n.trim())
            .filter(|n| !n.is_empty())
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<HashSet<_>>();

        let mut matches = 0;
        for num in numbers {
            if !winning_numbers.contains(&num) {
                continue;
            }

            matches += 1;
        }

        if matches == 0 {
            return matches;
        }

        return 2_u32.pow(matches - 1);
    }
}

impl FromStr for Card {
    type Err = std::fmt::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut index = 0;
        let mut semicolon_idx = 5;
        for i in 5..s.len() {
            let letter = s.chars().nth(i).unwrap();
            if letter != ':' {
                continue;
            }

            let num = s.get(5..i).unwrap();
            let trimmed = num.trim_start();
            index = trimmed.parse().unwrap();
            semicolon_idx = i;
        }

        let scratchcard = s.get(semicolon_idx + 1..).unwrap();
        println!("scratchcard: '{}'", scratchcard);
        let points = Card::get_points_from_scratchcard(scratchcard);

        println!("Card {}: {} points", index, points);
        return Ok(Self { _index: index, points });
    }
}

pub fn part_one(_args: Args) {
    let sum = FILE_CONTENTS
        .lines()
        .map(|line| line.parse::<Card>().unwrap().points)
        .reduce(|acc, x| acc + x)
        .unwrap();
    println!("Sum: {}", sum);
}

pub fn part_two(_args: Args) {}
