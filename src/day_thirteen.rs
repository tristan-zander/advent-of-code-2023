use itertools::Itertools;

use crate::Args;

const FILE_CONTENTS: &'static str = include_str!("../inputs/day_thirteen.txt");

fn input(input: &str) -> Vec<(Vec<String>, Vec<String>)> {
    let patterns = input
        .split("\n\n")
        .map(|pattern| {
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
        })
        .collect_vec();

    patterns
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

    unreachable!()
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

fn row_symmetry(rows: &[String]) -> Option<usize> {
    symmetry(rows).map(|left| left * 100)
}

fn col_symmetry(cols: &[String]) -> Option<usize> {
    symmetry(cols)
}

fn solve(rows: &[String], cols: &[String]) -> usize {
    if let Some(num) = row_symmetry(rows) {
        return num;
    }

    if let Some(num) = col_symmetry(cols) {
        return num;
    }

    return 0;
}

/// ARGS:
/// ret.0 == true IF it was a column, otherwise it's a row
/// ret.1 is the index in the row/column that was matched.
fn solve_part_two(
    rows: &[String],
    cols: &[String],
    starting: Option<(bool, usize)>,
) -> Option<(bool, usize)> {
    if let Some(num) = row_symmetry(rows) {
        let res = (false, num / 100);
        if starting.map(|starting| res != starting).unwrap_or(true) {
            return Some(res);
        }
    }

    if let Some(num) = col_symmetry(cols) {
        return Some((true, num));
    }

    return None;
}

pub fn part_one(_args: Args) {
    let input = input(FILE_CONTENTS);
    let res = input
        .into_iter()
        .map(|(rows, cols)| solve(&rows, &cols))
        .sum::<usize>();
    println!("Sum: {}", res);
}

pub fn part_two(_args: Args) {
    let input = input(FILE_CONTENTS);
    let res = input
        .into_iter()
        .map(|(mut rows, mut cols)| {
            let len = rows[0].len();

            let (original_is_column, original_index) = solve_part_two(&rows, &cols, None).unwrap();

            for row in 0..rows.len() {
                for col in 0..len {
                    // SAFETY: I hope this is valid UTF-8
                    unsafe {
                        let old_char = rows[row].bytes().nth(col).unwrap();

                        rows[row].as_mut_vec()[col] = b'#';
                        cols[col].as_mut_vec()[row] = b'#';

                        if let Some((new_is_column, index)) =
                            solve_part_two(&rows, &cols, Some((original_is_column, original_index)))
                        {
                            if original_is_column && !new_is_column || (index != original_index) {
                                return solve(&rows, &cols);
                            }
                        }

                        rows[row].as_mut_vec()[col] = b'.';
                        cols[col].as_mut_vec()[row] = b'.';

                        if let Some((new_is_column, index)) =
                            solve_part_two(&rows, &cols, Some((original_is_column, original_index)))
                        {
                            if original_is_column && !new_is_column || (index != original_index) {
                                return solve(&rows, &cols);
                            }
                        }

                        rows[row].as_mut_vec()[col] = old_char;
                        cols[col].as_mut_vec()[row] = old_char;
                    }
                }
            }

            unreachable!()
        })
        .sum::<usize>();
    println!("Sum: {}", res);
}
