use itertools::Itertools;

use crate::Args;

const FILE_CONTENTS: &'static str = include_str!("../inputs/day_thirteen.txt");

fn input(input: &str) -> Vec<(Vec<String>, Vec<String>)> {
    let patterns = input
        .split("\n\n")
        .map(|pattern| breakout_pattern(pattern))
        .collect_vec();

    patterns
}

fn breakout_pattern(pattern: &str) -> (Vec<String>, Vec<String>) {
    let rows = pattern.lines().map(|s| s.to_owned()).collect_vec();
    let rows_slice = rows.as_slice();
    let cols = (0..rows[0].len())
        .map(move |i| {
            String::from_utf8(
                rows_slice
                    .iter()
                    .map(move |r| r.bytes().nth(i).unwrap())
                    .collect_vec(),
            )
            .unwrap()
        })
        .collect_vec();
    (rows, cols)
}

fn get_matches(slice: &[String]) -> Vec<usize> {
    slice
        .iter()
        .enumerate()
        .rev()
        .skip(1)
        .rev()
        .filter_map(|(i, val)| {
            if val.as_str() == slice[i + 1].as_str() {
                return Some(i);
            }

            None
        })
        .collect_vec()
}

fn is_reflection(iter: &[String], left: usize) -> bool {
    // walk until left == right
    // left += 1 (wrap with length) and right -= 1 (wrap with length)
    // ensure that iter[left] == iter[right]

    let right = left + 1;

    for i in 0..iter.len() {
        let left_str = iter[left - i].as_str();
        let right_str = iter[right + i].as_str();

        if left_str != right_str {
            return false;
        }

        if left - i == 0 || right + i == iter.len() - 1 {
            return true;
        }
    }

    return false;
}

fn symmetry(iter: &[String]) -> Option<usize> {
    let matches = get_matches(iter);

    for possible_symmetry in matches {
        if is_reflection(iter, possible_symmetry) {
            return Some(possible_symmetry + 1);
        }
    }

    None
}

fn symmetry_p2(iter: &[String], original: usize) -> Option<usize> {
    let matches = get_matches(iter);

    for possible_symmetry in matches {
        if is_reflection(iter, possible_symmetry) && possible_symmetry + 1 != original {
            return Some(possible_symmetry + 1);
        }
    }

    None
}

fn row_symmetry(rows: &[String]) -> Option<usize> {
    symmetry(rows).map(|left| left * 100)
}

fn col_symmetry(cols: &[String]) -> Option<usize> {
    symmetry(cols)
}

fn solve(rows: &[String], cols: &[String]) -> Option<usize> {
    if let Some(num) = col_symmetry(cols) {
        return Some(num);
    }

    if let Some(num) = row_symmetry(rows) {
        return Some(num);
    }

    return None;
}

fn solve_p2(rows: &[String], cols: &[String], mut original: usize) -> Option<usize> {
    if let Some(num) = symmetry_p2(cols, original) {
        return Some(num);
    }

    if original >= 100 {
        original = original / 100;
    }

    if let Some(num) = symmetry_p2(rows, original).map(|r| r * 100) {
        return Some(num);
    }

    return None;
}

pub fn part_one(_args: Args) {
    let input = input(FILE_CONTENTS);
    let res = input
        .into_iter()
        .map(|(rows, cols)| {
            let res = solve(&rows, &cols).unwrap();
            if res == 0 {
                panic!("Part 1 should never be 0");
            }
            res
        })
        .sum::<usize>();
    println!("Sum: {}", res);
}

pub fn part_two(_args: Args) {
    let input = input(FILE_CONTENTS);
    let res = input
        .into_iter()
        .map(|(rows, cols)| {
            let normal_solution = solve(&rows, &cols).unwrap();
            let rows_with_one_difference = duplicate_with_differences(&rows);

            for (diffed_row, diffed_col) in rows_with_one_difference {
                let maybe_res = solve_p2(&diffed_row, &diffed_col, normal_solution);
                if maybe_res.map(|res| res != normal_solution).unwrap_or(false) {
                    return maybe_res.unwrap();
                }
            }

            let cols_with_one_difference = duplicate_with_differences(&cols);

            for (diffed_col, diffed_row) in cols_with_one_difference {
                let maybe_res = solve_p2(&diffed_row, &diffed_col, normal_solution);
                if maybe_res.map(|res| res != normal_solution).unwrap_or(false) {
                    return maybe_res.unwrap();
                }
            }

            unreachable!("Part 2 should never get here.");
        })
        .sum::<usize>();
    println!("Sum: {}", res);
}

fn duplicate_with_differences(iter: &[String]) -> Vec<(Vec<String>, Vec<String>)> {
    let differences = iter
        .iter()
        .enumerate()
        .filter_map(|(i, str_to_match)| {
            let mut counter = Vec::new();
            for (j, other) in iter.iter().map(|s| s.as_str()).enumerate() {
                let differences = str_to_match
                    .bytes()
                    .zip(other.bytes())
                    .enumerate()
                    .filter(|(_, (left, right))| left != right)
                    .collect_vec();

                if differences.len() != 1 {
                    continue;
                }

                counter.push((i, j, differences[0].0, differences[0].1));
            }

            if counter.len() == 0 {
                return None;
            }

            Some(counter)
        })
        .flat_map(|f| f);

    let mut res = Vec::new();
    for (left_row, right_row, str_idx, (left_byte, right_byte)) in differences {
        let left_str = iter[left_row].as_str();
        let right_str = iter[right_row].as_str();

        let mut left_swapped = iter.to_owned();
        left_swapped[left_row] = swap(left_str, right_byte, str_idx);

        let mut right_swapped = iter.to_owned();
        right_swapped[right_row] = swap(right_str, left_byte, str_idx);

        res.push(left_swapped);
        res.push(right_swapped);
    }

    res.into_iter()
        .map(|x| breakout_pattern(x.join("\n").as_str()))
        .collect_vec()
}

fn swap(original: &str, byte: u8, index: usize) -> String {
    let mut mutated = original.to_owned();
    // SAFETY: I'll ensure it's always valid UTF-8
    let buf = unsafe { mutated.as_bytes_mut() };
    buf[index] = byte;
    mutated
}
