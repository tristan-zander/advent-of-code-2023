use plotters::prelude::*;
use std::{num::ParseIntError, str::FromStr};

use itertools::Itertools;
use plotters::{chart::ChartBuilder, drawing::IntoDrawingArea};

use crate::Args;

const FILE_CONTENTS: &'static str = include_str!("../inputs/day_eighteen.txt");

struct PlanEntry {
    direction: Direction,
    meters: u16,
    color: (u32, u16),
}

impl FromStr for PlanEntry {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, meters, color) = s
            .split_whitespace()
            .collect_tuple()
            .ok_or_else(|| anyhow!("Input is malformed"))?;

        Ok(PlanEntry {
            direction: direction.parse()?,
            meters: meters.parse().map_err(|e: ParseIntError| anyhow!(e))?,
            color: (
                u32::from_str_radix(&color[2..=6], 16).map_err(|e| anyhow!(e))?,
                u16::from_str_radix(&color[7..=7], 16).map_err(|e| anyhow!(e))?,
            ),
        })
    }
}

enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn forward(&self) -> (i32, i32) {
        match self {
            Direction::North => (0, 1),
            Direction::East => (1, 0),
            Direction::South => (0, -1),
            Direction::West => (-1, 0),
        }
    }
}

impl FromStr for Direction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(Direction::East),
            "L" => Ok(Direction::West),
            "U" => Ok(Direction::North),
            "D" => Ok(Direction::South),
            _ => Err(anyhow!("Unknown enumeration")),
        }
    }
}

struct Lake {
    points: Vec<(i64, i64)>,
}

impl Lake {
    pub fn new() -> Self {
        Self { points: vec![] }
    }

    pub fn add_point(&mut self, x: i64, y: i64) {
        self.points.push((x, y));
    }

    pub fn draw(&self) {
        let backend = SVGBackend::new("visual.svg", (500, 500));
        let area = backend.into_drawing_area();
        area.fill(&WHITE).unwrap();
        let mut chart = ChartBuilder::on(&area)
            .build_cartesian_2d(
                (self.points.iter().map(|p| p.0 as i64).min().unwrap() - 1)
                    ..(self.points.iter().map(|p| p.0).max().unwrap() + 1),
                (self.points.iter().map(|p| p.1).min().unwrap() - 1)
                    ..(self.points.iter().map(|p| p.1).max().unwrap() + 1),
            )
            .unwrap();
        chart
            .draw_series(LineSeries::new(
                self.points.iter().map(|l| (l.0, l.1)),
                RED.filled(),
            ))
            .unwrap();
        chart.configure_mesh().draw().unwrap();
        area.present().unwrap();
    }

    pub fn area(&self) -> f64 {
        let mut area = 0;
        for (left, right) in self.points.iter().tuple_windows() {
            let (x1, y1) = left;
            let (x2, y2) = right;
            area += (y1 * x2) - (x1 * y2);
        }

        let perimeter = self
            .points
            .iter()
            .tuple_windows()
            .fold(0, |acc, (left, right)| {
                acc + ((left.0 - right.0) + (left.1 - right.1)).abs()
            });

        ((area / 2).abs() as f64 - (0.5 * perimeter as f64) + 1.) + perimeter as f64
    }
}

fn input() -> Vec<PlanEntry> {
    FILE_CONTENTS
        .lines()
        .map(|l| l.parse().unwrap())
        .collect_vec()
}

fn plan(calculate: fn(PlanEntry, (i64, i64)) -> (i64, i64)) {
    let input = input();
    let mut grid = Lake::new();
    let start = (0, 0);
    grid.add_point(start.0, start.1);
    let mut position = (start.0, start.1);

    for entry in input {
        let (x, y) = calculate(entry, position);
        grid.add_point(x, y);
        position = (x, y);
    }
    grid.add_point(start.0, start.1);
    grid.draw();

    println!("Area: {}", grid.area());
}

pub fn part_one(_args: Args) {
    plan(|entry, position| {
        let forward = entry.direction.forward();
        let x = position.0 + (forward.0 * entry.meters as i32) as i64;
        let y = position.1 + (forward.1 * entry.meters as i32) as i64;
        return (x, y);
    })
}
pub fn part_two(_args: Args) {
    plan(|entry, position| {
        let distance = entry.color.0 as i64;
        let direction = match entry.color.1 {
            0 => Direction::East,
            1 => Direction::South,
            2 => Direction::West,
            3 => Direction::North,
            _ => unreachable!(),
        };
        let forward = direction.forward();
        let x = position.0 + (forward.0 as i64 * distance) as i64;
        let y = position.1 + (forward.1 as i64 * distance) as i64;
        return (x, y);
    })
}
