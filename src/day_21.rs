use std::{collections::HashSet, io::Write};

use crate::Args;

const FILE_CONTENTS: &'static str = include_str!("../inputs/day_21.txt");

fn input() -> Vec<Vec<u8>> {
    FILE_CONTENTS
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect()
}

fn starting_point(input: &[Vec<u8>]) -> (usize, usize) {
    input
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter().enumerate().find_map(
                |(x, &cell)| {
                    if cell == b'S' {
                        Some((x, y))
                    } else {
                        None
                    }
                },
            )
        })
        .unwrap()
}

fn adjacent_positions(input: &[Vec<u8>], (x, y): (usize, usize)) -> Vec<(usize, usize)> {
    let mut positions = Vec::new();
    if x > 0 {
        positions.push((x - 1, y));
    }
    if y > 0 {
        positions.push((x, y - 1));
    }
    if x < input[y].len() - 1 {
        positions.push((x + 1, y));
    }
    if y < input.len() - 1 {
        positions.push((x, y + 1));
    }
    positions
}

fn adjacent_positions_infinite((x, y): (isize, isize)) -> Vec<(isize, isize)> {
    vec![(x - 1, y), (x, y - 1), (x + 1, y), (x, y + 1)]
}

fn step(input: &[Vec<u8>], positions: &mut Vec<(usize, usize)>) {
    let mut new_positions = HashSet::with_capacity(positions.len() * 4);

    for (x, y) in positions.drain(..) {
        for (x, y) in adjacent_positions(input, (x, y)) {
            if input[y][x] == b'.' || input[y][x] == b'S' {
                new_positions.insert((x, y));
            }
        }
    }

    positions.extend(new_positions)
}

fn step_infinite(
    input: &[Vec<u8>],
    positions: &mut Vec<(isize, isize)>,
    new_positions: &mut HashSet<(isize, isize)>,
) {
    let y_len = input.len();
    let x_len = input[0].len();

    for (x, y) in positions.drain(..) {
        for (x, y) in adjacent_positions_infinite((x, y)) {
            let inp =
                input[y.rem_euclid(y_len as isize) as usize][x.rem_euclid(x_len as isize) as usize];
            if inp == b'.' || inp == b'S' {
                new_positions.insert((x, y));
            }
        }
    }

    positions.extend(new_positions.iter());
    new_positions.clear();
}

fn debug(input: &[Vec<u8>], positions: &[(usize, usize)]) {
    for (y, row) in input.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if positions.contains(&(x, y)) {
                print!("O");
            } else {
                print!("{}", *cell as char);
            }
        }
        println!();
    }
}

pub fn part_one(_args: Args) {
    let input = input();
    let starting_point = starting_point(&input);
    let mut positions = vec![starting_point];

    for _ in 0..64 {
        step(&input, &mut positions);
    }

    debug(&input, &positions);
    println!("Number of available spots: {}", positions.len());
}

pub fn part_two(_args: Args) {
    let input = input();
    let starting_point = starting_point(&input);
    let mut positions = Vec::with_capacity(1024 * 1024 * 32);
    positions.push((starting_point.0 as _, starting_point.1 as _));
    let mut new_positions = HashSet::with_capacity(1024 * 1024 * 32);
    let mut csv = std::fs::File::create("day_21.csv").unwrap();

    for i in 0..26501365 {
        if i % 100 == 0 {
            println!("Iteration: {}", i);
        }
        step_infinite(&input, &mut positions, &mut new_positions);
        csv.write(format!("{},{}\n", i + 1, positions.len()).as_bytes())
            .unwrap();
    }

    println!("Number of available spots: {}", positions.len());
}
