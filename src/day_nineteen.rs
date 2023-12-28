use std::{collections::HashMap, iter::Sum, str::FromStr};

use itertools::{iproduct, Itertools};

use crate::Args;

const FILE_CONTENTS: &'static str = include_str!("../inputs/day_nineteen.txt");

type Destination = String;
#[derive(Debug, Clone)]
enum Logic {
    GreaterThan(char, u32, Command),
    LessThan(char, u32, Command),
    Command(Command),
}

#[derive(Debug, Clone)]
enum Command {
    Accept,
    Reject,
    Redirect(Destination),
}

impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Command::Accept),
            "R" => Ok(Command::Reject),
            _ => Ok(Command::Redirect(s.to_owned())),
        }
    }
}

impl FromStr for Logic {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((left, dest)) = s.split(':').collect_tuple() {
            let is_greater_than = left.contains('>');

            if is_greater_than {
                let (part, num) = left.split('>').collect_tuple().unwrap();
                return Ok(Self::GreaterThan(
                    part.chars().next().unwrap(),
                    num.parse().unwrap(),
                    dest.parse().unwrap(),
                ));
            }

            let (part, num) = left.split('<').collect_tuple().unwrap();
            return Ok(Self::LessThan(
                part.chars().next().unwrap(),
                num.parse().unwrap(),
                dest.parse().unwrap(),
            ));
        }
        Ok(Self::Command(s.parse().unwrap()))
    }
}

#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct PartRating {
    pub x: u32,
    pub m: u32,
    pub a: u32,
    pub s: u32,
}

impl Sum<PartRating> for u64 {
    fn sum<I: Iterator<Item = PartRating>>(iter: I) -> Self {
        iter.fold(0, |acc, p| acc + (p.x + p.m + p.a + p.s) as u64)
    }
}

impl FromStr for PartRating {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut this = Self {
            ..Default::default()
        };

        for (left, right) in s
            .replace("{", "")
            .replace("}", "")
            .split(",")
            .map(|s| s.split("=").collect_tuple::<(_, _)>().unwrap())
        {
            match left {
                "x" => this.x = right.parse().unwrap(),
                "m" => this.m = right.parse().unwrap(),
                "a" => this.a = right.parse().unwrap(),
                "s" => this.s = right.parse().unwrap(),
                _ => unreachable!(),
            }
        }

        Ok(this)
    }
}

fn input() -> (HashMap<Destination, Box<[Logic]>>, Vec<PartRating>) {
    let (rules, parts) = FILE_CONTENTS.split("\n\n").collect_tuple().unwrap();
    let rules = rules
        .lines()
        .map(|l| {
            let (name, rules) = l.split("{").collect_tuple().unwrap();
            let rules = rules.replace("}", "");
            let rules = rules
                .split(",")
                .map(|r| r.parse::<Logic>().unwrap())
                .collect();
            (name.to_owned(), rules)
        })
        .collect::<HashMap<_, _>>();

    let parts = parts
        .lines()
        .map(|l| l.parse::<PartRating>().unwrap())
        .collect();

    (rules, parts)
}

fn handle_rule(part: &PartRating, rules: &[Logic]) -> Command {
    for rule in rules {
        match rule {
            Logic::GreaterThan(category, num, command) => match category {
                'x' => {
                    if part.x > *num {
                        return command.to_owned();
                    }
                }
                'm' => {
                    if part.m > *num {
                        return command.to_owned();
                    }
                }
                'a' => {
                    if part.a > *num {
                        return command.to_owned();
                    }
                }
                's' => {
                    if part.s > *num {
                        return command.to_owned();
                    }
                }
                _ => unreachable!(),
            },
            Logic::LessThan(category, num, command) => match category {
                'x' => {
                    if part.x < *num {
                        return command.to_owned();
                    }
                }
                'm' => {
                    if part.m < *num {
                        return command.to_owned();
                    }
                }
                'a' => {
                    if part.a < *num {
                        return command.to_owned();
                    }
                }
                's' => {
                    if part.s < *num {
                        return command.to_owned();
                    }
                }
                _ => unreachable!(),
            },
            Logic::Command(comm) => return comm.to_owned(),
        }
    }

    unreachable!()
}

fn should_be_accepted(part: &PartRating, rules: &HashMap<String, Box<[Logic]>>) -> bool {
    let mut rule = rules.get("in").unwrap().as_ref();

    loop {
        match handle_rule(part, rule) {
            Command::Accept => return true,
            Command::Reject => return false,
            Command::Redirect(r) => rule = rules.get(r.as_str()).unwrap().as_ref(),
        }
    }
}

pub fn part_one(_args: Args) {
    let (rules, parts) = input();

    let sum = parts
        .into_iter()
        .filter(|p| should_be_accepted(p, &rules))
        .sum::<u64>();

    println!("Sum: {}", sum);
}

#[derive(Debug, Clone)]
struct PartRange {
    pub x: std::ops::Range<u32>,
    pub m: std::ops::Range<u32>,
    pub a: std::ops::Range<u32>,
    pub s: std::ops::Range<u32>,
}

pub fn part_two(_args: Args) {
    let (rule_map, _) = input();

    let starting_range = PartRange {
        x: 1..4001,
        m: 1..4001,
        a: 1..4001,
        s: 1..4001,
    };

    let mut rules_to_process = vec![(starting_range, rule_map.get("in").unwrap().as_ref())];
    let mut accepted: Vec<PartRange> = Vec::new();
    while let Some((mut part_range, rules)) = rules_to_process.pop() {
        for rule in rules {
            match rule {
                Logic::GreaterThan(c, num, command) => match c {
                    'x' => {
                        if !part_range.x.contains(num) {
                            continue;
                        }

                        let min = part_range.x.start;
                        let max = part_range.x.end;

                        let mut part_clone = part_range.clone();
                        part_clone.x = num + 1..max;
                        part_range.x = min..*num + 1;

                        let rule = match command {
                            Command::Accept => {
                                accepted.push(part_clone);
                                continue;
                            }
                            Command::Reject => {
                                continue;
                            }
                            Command::Redirect(r) => rule_map.get(r.as_str()).unwrap().as_ref(),
                        };

                        rules_to_process.push((part_clone, rule));
                    }
                    'm' => {
                        if !part_range.m.contains(num) {
                            continue;
                        }

                        let min = part_range.m.start;
                        let max = part_range.m.end;

                        let mut part_clone = part_range.clone();
                        part_clone.m = num + 1..max;
                        part_range.m = min..*num + 1;

                        let rule = match command {
                            Command::Accept => {
                                accepted.push(part_clone);
                                continue;
                            }
                            Command::Reject => {
                                continue;
                            }
                            Command::Redirect(r) => rule_map.get(r.as_str()).unwrap().as_ref(),
                        };

                        rules_to_process.push((part_clone, rule));
                    }
                    'a' => {
                        if !part_range.a.contains(num) {
                            continue;
                        }

                        let min = part_range.a.start;
                        let max = part_range.a.end;

                        let mut part_clone = part_range.clone();
                        part_clone.a = num + 1..max;
                        part_range.a = min..*num + 1;

                        let rule = match command {
                            Command::Accept => {
                                accepted.push(part_clone);
                                continue;
                            }
                            Command::Reject => {
                                continue;
                            }
                            Command::Redirect(r) => rule_map.get(r.as_str()).unwrap().as_ref(),
                        };

                        rules_to_process.push((part_clone, rule));
                    }
                    's' => {
                        if !part_range.s.contains(num) {
                            continue;
                        }

                        let min = part_range.s.start;
                        let max = part_range.s.end;

                        let mut part_clone = part_range.clone();
                        part_clone.s = num + 1..max;
                        part_range.s = min..*num + 1;

                        let rule = match command {
                            Command::Accept => {
                                accepted.push(part_clone);
                                continue;
                            }
                            Command::Reject => {
                                continue;
                            }
                            Command::Redirect(r) => rule_map.get(r.as_str()).unwrap().as_ref(),
                        };

                        rules_to_process.push((part_clone, rule));
                    }
                    _ => unreachable!(),
                },
                Logic::LessThan(c, num, command) => match c {
                    'x' => {
                        if !part_range.x.contains(num) {
                            continue;
                        }

                        let min = part_range.x.start;
                        let max = part_range.x.end;

                        let mut part_clone = part_range.clone();
                        part_clone.x = min..*num;
                        part_range.x = *num..max;

                        let rule = match command {
                            Command::Accept => {
                                accepted.push(part_clone);
                                continue;
                            }
                            Command::Reject => {
                                continue;
                            }
                            Command::Redirect(r) => rule_map.get(r.as_str()).unwrap().as_ref(),
                        };

                        rules_to_process.push((part_clone, rule));
                    }
                    'm' => {
                        if !part_range.m.contains(num) {
                            continue;
                        }

                        let min = part_range.m.start;
                        let max = part_range.m.end;

                        let mut part_clone = part_range.clone();
                        part_clone.m = min..*num;
                        part_range.m = *num..max;

                        let rule = match command {
                            Command::Accept => {
                                accepted.push(part_clone);
                                continue;
                            }
                            Command::Reject => {
                                continue;
                            }
                            Command::Redirect(r) => rule_map.get(r.as_str()).unwrap().as_ref(),
                        };

                        rules_to_process.push((part_clone, rule));
                    }
                    'a' => {
                        if !part_range.a.contains(num) {
                            continue;
                        }

                        let min = part_range.a.start;
                        let max = part_range.a.end;

                        let mut part_clone = part_range.clone();
                        part_clone.a = min..*num;
                        part_range.a = *num..max;

                        let rule = match command {
                            Command::Accept => {
                                accepted.push(part_clone);
                                continue;
                            }
                            Command::Reject => {
                                continue;
                            }
                            Command::Redirect(r) => rule_map.get(r.as_str()).unwrap().as_ref(),
                        };

                        rules_to_process.push((part_clone, rule));
                    }
                    's' => {
                        if !part_range.s.contains(num) {
                            continue;
                        }

                        let min = part_range.s.start;
                        let max = part_range.s.end;

                        let mut part_clone = part_range.clone();
                        part_clone.s = min..*num;
                        part_range.s = *num..max;

                        let rule = match command {
                            Command::Accept => {
                                accepted.push(part_clone);
                                continue;
                            }
                            Command::Reject => {
                                continue;
                            }
                            Command::Redirect(r) => rule_map.get(r.as_str()).unwrap().as_ref(),
                        };

                        rules_to_process.push((part_clone, rule));
                    }
                    _ => unreachable!(),
                },
                Logic::Command(c) => match c {
                    Command::Accept => {
                        accepted.push(part_range.clone());
                        continue;
                    }
                    Command::Reject => {
                        continue;
                    }
                    Command::Redirect(r) => rules_to_process.push((
                        part_range.clone(),
                        rule_map.get(r.as_str()).unwrap().as_ref(),
                    )),
                },
            }
        }
    }

    println!("{:#?}", accepted);

    let mut count = 0;
    for part_range in accepted {
        count += iproduct!(part_range.x, part_range.m, part_range.a, part_range.s).count();
    }

    println!("Count: {}", count);
}
