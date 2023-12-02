use crate::Args;

const INPUT: &'static str = include_str!("../inputs/day_two.txt");
const DESIRED_COMBINATION: ColorSet = ColorSet {
    red: 12,
    green: 13,
    blue: 14,
};

struct ColorSet {
    red: u32,
    green: u32,
    blue: u32,
}

impl ColorSet {
    pub fn new() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
}

struct Game {
    pub index: u32,
    pub sets: Vec<ColorSet>,
}

impl Game {
    pub fn get_index<'a>(input: &'a str) -> (u32, &'a str) {
        let mut colon_idx = 0;
        for (i, c) in input.chars().enumerate() {
            if c != ':' {
                continue;
            }

            colon_idx = i;
            break;
        }

        return (
            input[..colon_idx].trim().parse().unwrap(),
            &input[(colon_idx + 1)..],
        );
    }
}

fn parse() -> Vec<Game> {
    let games = INPUT
        .split('\n')
        .map(|line| {
            let without_game = &line[4..];

            let (idx, rest) = Game::get_index(without_game);
            let trimmed = rest.trim();

            let colors = trimmed
                .split(';')
                .map(|color_part_str| {
                    let all_colors = color_part_str
                        .split(',')
                        .map(|c| c.trim())
                        .map(|color_str| {
                            let parts = color_str.split(' ').take(2).collect::<Vec<_>>();
                            assert_eq!(parts.len(), 2);

                            let num: u32 = parts[0].parse().unwrap();
                            let color_name = parts[1];

                            return (num, color_name);
                        });

                    let mut color_set = ColorSet::new();
                    for (num, color_name) in all_colors {
                        match color_name {
                            "blue" => {
                                color_set.blue += num;
                            }
                            "green" => {
                                color_set.green += num;
                            }
                            "red" => {
                                color_set.red += num;
                            }
                            _ => {
                                unimplemented!()
                            }
                        }
                    }

                    color_set
                })
                .collect::<Vec<_>>();

            Game {
                index: idx,
                sets: colors,
            }
        })
        .collect::<Vec<_>>();

    return games;
}

pub fn part_one(_args: Args) {
    let games = parse();

    let sum = games
        .iter()
        .filter(|g| {
            for set in &g.sets {
                if set.blue > DESIRED_COMBINATION.blue
                    || set.red > DESIRED_COMBINATION.red
                    || set.green > DESIRED_COMBINATION.green
                {
                    return false;
                }
            }

            return true;
        })
        .map(|g| g.index)
        .reduce(|acc, i| acc + i)
        .unwrap();

    println!("sum {}", sum);
}

pub fn part_two(_args: Args) {}
