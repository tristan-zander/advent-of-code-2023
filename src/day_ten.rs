use std::{
    cell::RefCell,
    collections::{btree_map::Entry, BTreeMap, BTreeSet},
    fmt::Display,
    rc::Rc,
    str::FromStr,
};

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
    pub coords: (i64, i64),
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
    pub fn new(x: i64, y: i64, pipe_type: PipeType) -> Self {
        Self {
            distance: 0,
            coords: (x, y),
            pipe_type,
        }
    }

    pub fn get_neighbors(&self, bounding_height: i64, bounding_width: i64) -> Vec<(i64, i64)> {
        let (x, y) = self.coords;

        match self.pipe_type {
            PipeType::Start => {
                let mut ret = Vec::with_capacity(4);

                if y != 0 {
                    ret.push((x, y - 1));
                }

                if x < bounding_width {
                    ret.push((x + 1, y));
                }

                if y < bounding_height {
                    ret.push((x, y + 1));
                }

                if x != 0 {
                    ret.push((x - 1, y));
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

                if x < bounding_width {
                    ret.push((x + 1, y));
                }

                if x != 0 {
                    ret.push((x - 1, y));
                }

                ret
            }
            PipeType::NorthAndEast => {
                let mut ret = Vec::with_capacity(2);

                if y != 0 {
                    ret.push((x, y - 1));
                }

                if x < bounding_width {
                    ret.push((x + 1, y));
                }

                ret
            }
            PipeType::NorthAndWest => {
                let mut ret = Vec::with_capacity(2);

                if y != 0 {
                    ret.push((x, y - 1));
                }

                if x != 0 {
                    ret.push((x - 1, y));
                }

                ret
            }
            PipeType::SouthAndWest => {
                let mut ret = Vec::with_capacity(2);

                if y < bounding_height {
                    ret.push((x, y + 1));
                }

                if x != 0 {
                    ret.push((x - 1, y));
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

    pub fn can_accept(&self, other_x: i64, other_y: i64) -> bool {
        let (x, y) = (self.coords.0 as i64, self.coords.1 as i64);
        let (difference_x, difference_y) = (other_x as i64 - x, other_y as i64 - y);
        match self.pipe_type {
            PipeType::Start | PipeType::Ground => false,
            PipeType::Vertical => difference_x == 0 && difference_y.abs_diff(0) == 1,
            PipeType::Horizontal => difference_y == 0 && difference_x.abs_diff(0) == 1,
            PipeType::NorthAndEast | PipeType::NorthAndWest
                if (difference_x, difference_y) == (0, -1) =>
            {
                true
            }
            PipeType::NorthAndEast | PipeType::SouthAndEast
                if (difference_x, difference_y) == (1, 0) =>
            {
                true
            }
            PipeType::SouthAndEast | PipeType::SouthAndWest
                if (difference_x, difference_y) == (0, 1) =>
            {
                true
            }
            PipeType::SouthAndWest | PipeType::NorthAndWest
                if (difference_x, difference_y) == (-1, 0) =>
            {
                true
            }
            _ => false,
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
        let mut stack = vec![self.starting_pipe.clone()];
        let mut distances: BTreeMap<(i64, i64), Rc<RefCell<Pipe>>> = BTreeMap::new();

        let (height, width) = (self.pipes.len(), self.pipes[0].len());

        while let Some(p) = stack.pop() {
            let pipe = p.as_ref().borrow_mut();
            let (x, y) = pipe.coords;
            let new_distance = pipe.distance + 1;
            let neighbors = pipe.get_neighbors(height as i64, width as i64);

            let neighbors = neighbors
                .iter()
                .map(|(x, y)| self.pipes[*y as usize][*x as usize].to_owned())
                .filter(|p| match p.borrow().pipe_type {
                    PipeType::Ground | PipeType::Start => false,
                    _ => true,
                })
                .filter(|p| p.as_ref().borrow().can_accept(x, y));

            for n in neighbors {
                let mut neighbor = n.as_ref().borrow_mut();
                match distances.entry(neighbor.coords) {
                    Entry::Vacant(entry) => {
                        neighbor.distance = new_distance;
                        entry.insert(n.clone());
                        stack.push(n.clone());
                    }
                    Entry::Occupied(mut entry) => {
                        if neighbor.distance > new_distance {
                            neighbor.distance = new_distance;
                            entry.insert(n.clone());
                            stack.push(n.clone());
                        }
                    }
                }
            }
        }

        distances
            .values()
            .map(|p| p.borrow().distance)
            .max()
            .unwrap()
    }

    pub fn enclosed_tiles(&mut self) -> i64 {
        let mut stack = vec![self.starting_pipe.clone()];
        let mut visited: BTreeSet<(i64, i64)> = BTreeSet::new();
        visited.insert(self.starting_pipe.borrow().coords);
        let mut pipe_loop = vec![self.starting_pipe.as_ref().borrow().coords];

        let (height, width) = (self.pipes.len(), self.pipes[0].len());

        while let Some(p) = stack.pop() {
            let pipe = p.as_ref().borrow_mut();
            let (x, y) = pipe.coords;
            let neighbors = pipe.get_neighbors(height as i64, width as i64);

            let mut neighbors = neighbors
                .iter()
                .map(|(x, y)| self.pipes[*y as usize][*x as usize].to_owned())
                .filter(|p| match p.borrow().pipe_type {
                    PipeType::Ground | PipeType::Start => false,
                    _ if p.borrow().can_accept(x, y) && !visited.contains(&p.borrow().coords) => {
                        true
                    }
                    _ => false,
                });

            if let Some(n) = neighbors.next() {
                let neighbor = n.as_ref().borrow_mut();
                if visited.insert(neighbor.coords) {
                    stack.push(n.clone());
                    pipe_loop.push(neighbor.coords);
                }
            }
        }

        // Shoelace Formula (Pick's Theorem)
        // https://en.wikipedia.org/wiki/Shoelace_formula
        let pipe_len = pipe_loop.len() as i64;
        pipe_loop.push(self.starting_pipe.as_ref().borrow().coords);

        println!("{:?}", pipe_loop);

        let twice_area = pipe_loop
            .into_iter()
            .tuple_windows::<((i64, i64), (i64, i64))>()
            .map(|((x_1, y_1), (x_2, y_2))| (x_1 * y_2) - (x_2 * y_1))
            .sum::<i64>();

        let area = (twice_area / 2).abs();
        println!("Pipe Length: {}, Area: {}", pipe_len, area);
        return area - (pipe_len / 2) + 1;
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
                            x as i64,
                            y as i64,
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

pub fn part_two(_args: Args) {
    let mut input = FILE_CONTENTS.parse::<DayTen>().unwrap();
    let enclosed = input.enclosed_tiles();
    println!("Input:\n{}", input);
    println!("Enclosed: {}", enclosed);
}
