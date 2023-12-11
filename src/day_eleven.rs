use std::collections::HashMap;

use itertools::Itertools;

use crate::Args;

const FILE_CONTENTS: &'static str = include_str!("../inputs/day_eleven.txt");

/// Duplicates any lines/columns that contain empty space.
fn expand_universe_small(universe: &'static str) -> String {
    let lines = universe.lines().collect_vec();
    let height = lines.iter().count();
    let width = lines.iter().take(1).next().unwrap().len();

    let rows = (0..height)
        .map(|row| {
            let row = universe.lines().nth(row).unwrap();
            if !row.bytes().all(|b| b == b'.') {
                return false;
            }
            return true;
        })
        .collect_vec();

    let cols = (0..width)
        .map(|col: usize| {
            universe
                .lines()
                .map(move |row| row.bytes().nth(col).unwrap().to_owned())
                .collect_vec()
        })
        .map(|cols| {
            if cols.iter().all(|c| c == &b'.') {
                return true;
            }
            return false;
        })
        .collect_vec();

    let new_width = width + cols.iter().filter(|&&c| c).count();
    // let new_height = height + rows.iter().filter(|&&c| c).count();

    rows.iter()
        .enumerate()
        .flat_map(|(i, &row)| {
            if row == true {
                let empty_row = ".".repeat(new_width);
                return vec![empty_row.clone(), empty_row];
            }

            return vec![cols
                .iter()
                .enumerate()
                .flat_map(|(j, &col)| {
                    if col == true {
                        return vec!['.', '.'];
                    }
                    return vec![lines[i].chars().nth(j).unwrap()];
                })
                .join("")];
        })
        .join("\n")
}

/// Duplicates any lines/columns that contain empty space.
fn expand_universe_huge(universe: &'static str) -> Vec<(usize, usize)> {
    let lines = universe.lines().collect_vec();
    let height = lines.len();
    let width = lines.iter().take(1).next().unwrap().len();

    let mut rows = 0..height;

    let mut row_idx = 0;
    let mut coords = Vec::new();
    while let Some(row) = rows.next() {
        let line = lines[row];
        if line.bytes().all(|b| b == b'.') {
            row_idx += 1_000_000;
            continue;
        }

        let mut cols = 0..width;
        let mut col_idx = 0;
        while let Some(col) = cols.next() {
            if lines
                .iter()
                .map(|&l| l.bytes().nth(col).unwrap())
                .all(|c| c == b'.')
            {
                col_idx += 1_000_000;
                continue;
            }

            let character = line.bytes().nth(col).unwrap();
            if character == b'#' {
                coords.push((row_idx, col_idx));
            }

            col_idx += 1;
        }

        row_idx += 1;
    }

    coords
}

fn get_coords(universe: &str) -> Vec<(usize, usize)> {
    let lines = universe.lines().collect_vec();
    let height = lines.len();
    let width = lines[0].len();

    (0..height)
        .flat_map(|row| {
            let line = lines[row];
            (0..width).filter_map(move |col| {
                if line.bytes().nth(col).unwrap() == b'#' {
                    return Some((row, col));
                }
                return None;
            })
        })
        .collect_vec()
}

fn get_unique_combinations<T: IntoIterator<Item = usize>>(coords: T) -> Vec<(usize, usize)> {
    let len = coords.into_iter().count();
    (0..len - 1)
        .flat_map(|i| (i + 1..len).map(move |j| (i, j)))
        .collect_vec()
}

fn sum_lengths(galaxies: HashMap<usize, &(usize, usize)>) -> usize {
    let combinations = get_unique_combinations(galaxies.keys().cloned());

    combinations
        .into_iter()
        .map(|(a_idx, b_idx)| {
            let (a_x, a_y) = *galaxies[&a_idx];
            let (b_x, b_y) = *galaxies[&b_idx];

            let (a, b) = (a_x.abs_diff(b_x), a_y.abs_diff(b_y));
            a + b
        })
        .sum()
}

pub fn part_one(_args: Args) {
    println!("Original Universe:\n{}", FILE_CONTENTS);
    let expanded = expand_universe_small(FILE_CONTENTS);
    println!("Expanded Universe:\n{}", expanded);
    let coords = get_coords(&expanded);
    let galaxies = coords.iter().enumerate().collect::<HashMap<_, _>>();
    let sum = sum_lengths(galaxies);
    println!("Sum of all galaxy distances: {}", sum);
}

pub fn part_two(_args: Args) {
    println!("Original Universe:\n{}", FILE_CONTENTS);
    let coords = expand_universe_huge(FILE_CONTENTS);
    println!("Coords:\n{:?}", coords);
    let galaxies = coords.iter().enumerate().collect::<HashMap<_, _>>();
    let sum = sum_lengths(galaxies);
    println!("Sum of all galaxy distances: {}", sum);
}
