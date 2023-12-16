use std::collections::{hash_map::DefaultHasher, HashMap};

use itertools::Itertools;

use crate::Args;

const FILE_CONTENTS: &'static str = include_str!("../inputs/day_fourteen.txt");

fn input() -> Vec<Vec<u8>> {
    FILE_CONTENTS
        .lines()
        .map(|l| l.bytes().collect_vec())
        .collect_vec()
}

fn roll_north(dish: &mut Vec<Vec<u8>>) {
    let row_len = dish.len();
    for row_idx in 0..row_len {
        let col_len = dish[row_idx].len();
        for col_idx in 0..col_len {
            let byte = dish[row_idx][col_idx];
            if byte == b'O' {
                let idx = get_northmost_index(dish.as_slice(), (row_idx, col_idx));
                dish[row_idx][col_idx] = b'.';
                dish[idx][col_idx] = b'O';
            }
        }
    }
}

fn roll_south(dish: &mut Vec<Vec<u8>>) {
    let row_len = dish.len();
    for row_idx in (0..row_len).rev() {
        let col_len = dish[row_idx].len();
        for col_idx in 0..col_len {
            let byte = dish[row_idx][col_idx];
            if byte == b'O' {
                let idx = get_southmost_index(dish.as_slice(), (row_idx, col_idx));
                dish[row_idx][col_idx] = b'.';
                dish[idx][col_idx] = b'O';
            }
        }
    }
}

fn roll_east(dish: &mut Vec<Vec<u8>>) {
    let row_len = dish.len();
    for row_idx in 0..row_len {
        let col_len = dish[row_idx].len();
        for col_idx in (0..col_len).rev() {
            let byte = dish[row_idx][col_idx];
            if byte == b'O' {
                let idx = get_eastmost_index(dish.as_slice(), (row_idx, col_idx));
                dish[row_idx][col_idx] = b'.';
                dish[row_idx][idx] = b'O';
            }
        }
    }
}

fn roll_west(dish: &mut Vec<Vec<u8>>) {
    let row_len = dish.len();
    for row_idx in 0..row_len {
        let col_len = dish[row_idx].len();
        for col_idx in 0..col_len {
            let byte = dish[row_idx][col_idx];
            if byte == b'O' {
                let idx = get_westmost_index(dish.as_slice(), (row_idx, col_idx));
                dish[row_idx][col_idx] = b'.';
                dish[row_idx][idx] = b'O';
            }
        }
    }
}

fn get_northmost_index(iter: &[Vec<u8>], starting_index: (usize, usize)) -> usize {
    let (start_row, start_col) = starting_index;

    for i in (0..start_row).rev() {
        let byte = iter[i][start_col];
        if byte == b'#' || byte == b'O' {
            return i + 1;
        }
    }

    return 0;
}

fn get_eastmost_index(iter: &[Vec<u8>], starting_index: (usize, usize)) -> usize {
    let (start_row, start_col) = starting_index;
    let len = iter[0].len();
    for i in start_col + 1..len {
        let byte = iter[start_row][i];
        if byte == b'#' || byte == b'O' {
            return i - 1;
        }
    }

    return len - 1;
}

fn get_westmost_index(iter: &[Vec<u8>], starting_index: (usize, usize)) -> usize {
    let (start_row, start_col) = starting_index;
    for i in (0..start_col).rev() {
        let byte = iter[start_row][i];
        if byte == b'#' || byte == b'O' {
            return i + 1;
        }
    }

    return 0;
}

fn get_southmost_index(iter: &[Vec<u8>], starting_index: (usize, usize)) -> usize {
    let (start_row, start_col) = starting_index;

    let len = iter.len();
    for i in start_row + 1..len {
        let byte = iter[i][start_col];
        if byte == b'#' || byte == b'O' {
            return i - 1;
        }
    }

    return len - 1;
}

pub fn part_one(_args: Args) {
    let mut input = input();
    roll_north(&mut input);
    println!(
        "{}",
        input
            .iter()
            .map(|r| String::from_utf8(r.to_owned()).unwrap())
            .join("\n")
    );
    let len = input.len();
    let res = input
        .iter()
        .enumerate()
        .map(|(i, row)| row.iter().cloned().filter(|b| b == &b'O').count() * (len - i))
        .sum::<usize>();
    println!("Sum: {}", res);
}

fn hash(iter: &[Vec<u8>]) -> u64 {
    use std::hash::Hasher;
    let mut hasher = DefaultHasher::new();

    for row in iter {
        for byte in row {
            hasher.write_u8(*byte);
        }
        hasher.write_u8(b'\n');
    }

    hasher.finish()
}

pub fn part_two(_args: Args) {
    let mut input = input();

    let mut hashed = HashMap::new();

    for i in 0..1_000_000_000 {
        if i % 1000 == 0 {
            println!("{}", i);
        }

        roll_north(&mut input);
        roll_west(&mut input);
        roll_south(&mut input);
        roll_east(&mut input);

        let hash = hash(input.as_slice());
        if hashed.contains_key(&hash) {
            // We've already encountered this once, so we're in a loop
            let prev_idx = hashed.get(&hash).unwrap();
            let repeat_start = 1_000_000_000 - prev_idx;
            let diff = i - prev_idx;
            let num_iterations_left_in_repetition = (repeat_start % diff) - 1;
            for _ in 0..num_iterations_left_in_repetition {
                roll_north(&mut input);
                roll_west(&mut input);
                roll_south(&mut input);
                roll_east(&mut input);
            }
            break;
        }
        hashed.insert(hash, i);
    }
    println!(
        "{}",
        input
            .iter()
            .map(|r| String::from_utf8(r.to_owned()).unwrap())
            .join("\n")
    );
    let len = input.len();
    let res = input
        .iter()
        .enumerate()
        .map(|(i, row)| row.iter().cloned().filter(|b| b == &b'O').count() * (len - i))
        .sum::<usize>();
    println!("Sum: {}", res);
}
