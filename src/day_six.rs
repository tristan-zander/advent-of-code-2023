use std::str::FromStr;

use crate::Args;

const FILE_CONTENTS: &'static str = include_str!("../inputs/day_six.txt");

struct Race {
    milliseconds: u64,
    millimeters: u64,
}

impl Race {
    pub fn winning_combinations(&self) -> u64 {
        let mut winning_combinations = 0;
        for i in 1..self.milliseconds {
            let speed = i;
            let time_spent_moving = self.milliseconds - i;

            let distance = speed * time_spent_moving;

            if distance > self.millimeters {
                winning_combinations += 1;
            }
        }

        return winning_combinations;
    }
}

struct DaySix {
    pub races: Vec<Race>,
    pub single_race: Race,
}

impl FromStr for DaySix {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (time_line, distance_line) = {
            let mut lines = s.lines();
            let time_line = lines.next().unwrap();
            let distance_line = lines.next().unwrap();

            (time_line, distance_line)
        };

        let time_line_numbers = time_line.split(':').skip(1).next().unwrap().trim();

        let distance_line_numbers = distance_line.split(':').skip(1).next().unwrap().trim();

        let times = time_line_numbers
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect::<Vec<u64>>();

        let distances = distance_line
            .split(':')
            .skip(1)
            .next()
            .unwrap()
            .trim()
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect::<Vec<u64>>();

        let races = times
            .iter()
            .zip(distances.iter())
            .map(|(time, distance)| Race {
                milliseconds: *time,
                millimeters: *distance,
            })
            .collect();

        let single_race = Race {
            millimeters: distance_line_numbers
                .split_whitespace()
                .collect::<String>()
                .parse()
                .unwrap(),
            milliseconds: time_line_numbers
                .split_whitespace()
                .collect::<String>()
                .parse()
                .unwrap(),
        };

        Ok(Self { races, single_race })
    }
}

pub fn part_one(_args: Args) {
    let input = FILE_CONTENTS.parse::<DaySix>().unwrap();
    let res = input
        .races
        .iter()
        .map(|r| r.winning_combinations())
        .reduce(|acc, x| acc * x)
        .unwrap();

    println!("Total combinations: {}", res);
}

pub fn part_two(_args: Args) {
    let input = FILE_CONTENTS.parse::<DaySix>().unwrap();
    let res = input.single_race.winning_combinations();
    println!("Total combinations: {}", res);
}
