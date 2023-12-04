use crate::Args;

const INPUT: &'static str = include_str!("../inputs/day_three.txt");

/// Returns a list of numbers it finds around a symbol.
fn find_numbers_around_symbol(input: &Vec<Vec<char>>, symbol_index: (usize, usize)) -> Vec<u32> {
    let mut nums = vec![];
    for line_number in -1..=1 {
        let line_index = symbol_index.0.checked_add_signed(line_number);
        if line_index.is_none() {
            continue;
        }
        let line_index = line_index.unwrap();

        if line_index >= input.len() {
            continue;
        }

        let line = &input[line_index];

        let mut col_number_iter = -1..=1;

        while let Some(num) = col_number_iter.next() {
            let col_index = symbol_index.1.checked_add_signed(num);
            if col_index.is_none() {
                continue;
            }
            let col_index = col_index.unwrap();

            if col_index >= line.len() {
                continue;
            }

            if !line[col_index].is_numeric() {
                continue;
            }

            // walk backwards until the start of the number
            let mut start_index = col_index;
            while start_index > 0 {
                if !line[start_index - 1].is_numeric() {
                    break;
                }

                start_index -= 1
            }

            // walk forward until the end of the number
            let mut end_index = col_index;
            while end_index + 1 < line.len() {
                // advance iterator
                col_number_iter.next();

                if !line[end_index + 1].is_numeric() {
                    break;
                }

                end_index += 1;
            }

            let num_text = line[start_index..=end_index]
                .into_iter()
                .collect::<String>();

            nums.push(num_text.parse().unwrap());
        }
    }

    return nums;
}

fn get_all_product_numbers() -> Vec<u32> {
    let input = INPUT
        .split('\n')
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut nums = vec![];
    for (i, line) in input.iter().enumerate() {
        for (j, letter) in line.iter().enumerate() {
            if *letter == '.' {
                continue;
            }

            if !letter.is_numeric() {
                // letter is symbol
                find_numbers_around_symbol(&input, (i, j))
                    .iter()
                    .for_each(|n| nums.push(*n));
            }
        }
    }

    return nums;
}

pub fn part_one(_args: Args) {
    let product_nums = get_all_product_numbers();
    println!("Sum: {}", product_nums.into_iter().sum::<u32>());
}

pub fn part_two(_args: Args) {}
