use std::{borrow::BorrowMut, cell::RefCell, cmp::min, fmt::Display, rc::Rc, str::FromStr};

use itertools::Itertools;
use prettytable::{Cell, Row, Table};

use crate::Args;

const FILE_CONTENTS: &'static str = include_str!("../inputs/day_ten.txt");

#[derive(Debug, Clone)]
enum PipeType {
    Start,
    Vertical,
    Horizontal,
    NorthAndEast,
    NorthAndWest,
    SouthAndWest,
    SouthAndEast,
    Ground,
}

#[derive(Debug, Clone)]
struct Pipe {
    pub distance: u64,
    pub coords: (u64, u64),
    pub pipe_type: PipeType,
}

impl Display for Pipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self.pipe_type {
                PipeType::Start => 'S'.to_string(),
                PipeType::Ground => '.'.to_string(),
                _ => format!("{}", self.distance),
            }
        )
    }
}

impl Pipe {
    pub fn new(x: u64, y: u64, pipe_type: PipeType) -> Self {
        Self {
            distance: 0,
            coords: (x, y),
            pipe_type,
        }
    }

    pub fn get_neighbors(&self, bounding_height: u64, bounding_width: u64) -> Vec<(u64, u64)> {
        let (x, y) = self.coords;

        match self.pipe_type {
            PipeType::Start => {
                let mut ret = Vec::with_capacity(4);

                if x != 0 {
                    ret.push((x - 1, y));
                }

                if x < bounding_width {
                    ret.push((x + 1, y));
                }

                if y != 0 {
                    ret.push((x, y - 1));
                }

                if y < bounding_height {
                    ret.push((x, y + 1));
                }

                ret
            }
            PipeType::Vertical => {
                let mut ret = Vec::with_capacity(2);

                if y != 0 {
                    ret.push((x, y - 1));
                }

                if y < bounding_height {
                    ret.push((x, y + 1));
                }

                ret
            }
            PipeType::Horizontal => {
                let mut ret = Vec::with_capacity(2);

                if x != 0 {
                    ret.push((x - 1, y));
                }

                if x < bounding_width {
                    ret.push((x + 1, y));
                }

                ret
            }
            PipeType::NorthAndEast => {
                let mut ret = Vec::with_capacity(2);

                if x < bounding_width {
                    ret.push((x + 1, y));
                }

                if y != 0 {
                    ret.push((x, y - 1));
                }

                ret
            }
            PipeType::NorthAndWest => {
                let mut ret = Vec::with_capacity(2);

                if x != 0 {
                    ret.push((x - 1, y));
                }

                if y != 0 {
                    ret.push((x, y - 1));
                }

                ret
            }
            PipeType::SouthAndWest => {
                let mut ret = Vec::with_capacity(2);

                if x != 0 {
                    ret.push((x - 1, y));
                }

                if y < bounding_height {
                    ret.push((x, y + 1));
                }

                ret
            }
            PipeType::SouthAndEast => {
                let mut ret = Vec::with_capacity(2);

                if x < bounding_width {
                    ret.push((x + 1, y));
                }

                if y < bounding_height {
                    ret.push((x, y + 1));
                }

                ret
            }
            PipeType::Ground => vec![],
        }
    }
}

#[derive(Debug, Clone)]
struct DayTen {
    pipes: Vec<Vec<Rc<RefCell<Pipe>>>>,
    starting_pipe: Rc<RefCell<Pipe>>,
}

impl Display for DayTen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut table = Table::new();

        for row in &self.pipes {
            let mut cells = Vec::with_capacity(row.len());
            for pipe in row {
                cells.push(Cell::new(format!("{}", pipe.borrow()).as_str()))
            }
            table.add_row(Row::new(cells));
        }

        write!(f, "{}", table)
    }
}

impl DayTen {
    pub fn longest_distance(&mut self) -> u64 {
        let mut pipes_to_check = vec![self.starting_pipe.clone()];
        let mut neighbors = Vec::new();
        let mut distance = 0_u64;

        let (height, width) = (self.pipes.len(), self.pipes[0].len());

        while pipes_to_check.len() > 0 {
            neighbors.clear();
            for p in &pipes_to_check {
                let mut pipe = p.as_ref().borrow_mut();

                if pipe.distance != 0 {
                    continue;
                }

                pipe.distance = distance;

                let n = pipe.get_neighbors(height as _, width as _);

                neighbors.extend(
                    n.iter()
                        .map(|(x, y)| self.pipes[*y as usize][*x as usize].to_owned())
                        .filter(|p| match p.borrow().pipe_type {
                            PipeType::Ground | PipeType::Start => false,
                            _ => true,
                        }),
                );
            }

            pipes_to_check.clear();
            pipes_to_check.extend_from_slice(&neighbors);

            if neighbors.len() > 0 {
                distance += 1;
            }
        }

        distance - 1
    }
}

impl FromStr for DayTen {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pipes = s
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(x, c)| {
                        Rc::new(RefCell::new(Pipe::new(
                            x as u64,
                            y as u64,
                            match c {
                                'S' => PipeType::Start,
                                '|' => PipeType::Vertical,
                                '-' => PipeType::Horizontal,
                                'L' => PipeType::NorthAndEast,
                                'J' => PipeType::NorthAndWest,
                                '7' => PipeType::SouthAndWest,
                                'F' => PipeType::SouthAndEast,
                                '.' => PipeType::Ground,
                                _ => unreachable!(),
                            },
                        )))
                    })
                    .collect_vec()
            })
            .collect_vec();

        Ok(Self {
            starting_pipe: pipes
                .iter()
                .find(|l| {
                    l.iter()
                        .find(|p| match p.as_ref().borrow().pipe_type {
                            PipeType::Start => true,
                            _ => false,
                        })
                        .is_some()
                })
                .map(|l| {
                    l.iter()
                        .find(|p| match p.as_ref().borrow().pipe_type {
                            PipeType::Start => true,
                            _ => false,
                        })
                        .unwrap()
                        .to_owned()
                })
                .unwrap(),
            pipes,
        })
    }
}

pub fn part_one(_args: Args) {
    let mut input = FILE_CONTENTS.parse::<DayTen>().unwrap();
    let longest = input.longest_distance();
    println!("Input:\n{}", input);
    println!("Longest: {}", longest);
}
pub fn part_two(_args: Args) {}
