use ferris_says::say;
use itertools::Itertools;
use std::collections::HashMap;
use std::io::{stdout, BufWriter};

/*
--- Day 3: Gear Ratios ---
You and the Elf eventually reach a gondola lift station; he says the gondola lift will take you up to the water source, but this is as far as he can bring you. You go inside.

It doesn't take long to find the gondolas, but there seems to be a problem: they're not moving.

"Aaah!"

You turn around to see a slightly-greasy Elf with a wrench and a look of surprise. "Sorry, I wasn't expecting anyone! The gondola lift isn't working right now; it'll still be a while before I can fix it." You offer to help.

The engineer explains that an engine part seems to be missing from the engine, but nobody can figure out which one. If you can add up all the part numbers in the engine schematic, it should be easy to work out which part is missing.

The engine schematic (your puzzle input) consists of a visual representation of the engine. There are lots of numbers and symbols you don't really understand, but apparently any number adjacent to a symbol, even diagonally, is a "part number" and should be included in your sum. (Periods (.) do not count as a symbol.)

Here is an example engine schematic:

467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
In this schematic, two numbers are not part numbers because they are not adjacent to a symbol: 114 (top right) and 58 (middle right). Every other number is adjacent to a symbol and so is a part number; their sum is 4361.

Of course, the actual engine schematic is much larger. What is the sum of all of the part numbers in the engine schematic?
*/

fn extract_number_ranges(matrix: Vec<Vec<char>>) -> HashMap<Vec<usize>, usize> {
    let mut result = HashMap::new();
    let mut in_number = false;
    let mut start_pos = (0, 0);
    let mut current_number = String::new();

    for (row_idx, row) in matrix.iter().enumerate() {
        for (col_idx, &cell) in row.iter().enumerate() {
            if cell.is_ascii_digit() {
                if !in_number {
                    // Start of a new number
                    start_pos = (row_idx, col_idx);
                    in_number = true;
                    current_number.push(cell);
                } else {
                    // Continue current number
                    current_number.push(cell);
                }
            } else if in_number {
                // End of the current number
                let end_pos = (row_idx, col_idx - 1);
                let number_range = vec![start_pos.0, start_pos.1, end_pos.0, end_pos.1];
                result.insert(number_range.clone(), current_number.parse().unwrap());
                in_number = false;
                current_number.clear();
            }
        }

        // Check if the last cell was part of a number
        if in_number {
            let end_pos = (row_idx, row.len() - 1);
            let number_range = vec![start_pos.0, start_pos.1, end_pos.0, end_pos.1];
            result.insert(number_range.clone(), current_number.parse().unwrap());
            in_number = false;
            current_number.clear();
        }
    }

    result
}

fn is_point_within_direction(point: &[usize], direction: &[usize]) -> bool {
    let (start_x, start_y) = (direction[0], direction[1]);
    let (end_x, end_y) = (direction[2], direction[3]);

    let (point_x, point_y) = (point[0], point[1]);

    (start_x <= point_x && point_x <= end_x) && (start_y <= point_y && point_y <= end_y)
}

fn part_1(lines: Vec<String>) -> u32 {
    let matrix: Vec<Vec<char>> = lines
        .iter()
        .map(|line| line.trim().chars().collect())
        .collect();

    let number_ranges = extract_number_ranges(matrix.clone());


    let mut numbers: Vec<String> = Vec::new();
    let mut neighbour_number_ranges: Vec<&Vec<usize>> = Vec::new();
    for (row_idx, row) in matrix.iter().enumerate() {
        println!("{}", row_idx);

        for (col_idx, &ch) in row.iter().enumerate() {
            // is symbol
            if ch.is_numeric() || ch == '.' {
                continue;
            }
            println!("{}-{}", ch.is_numeric(), ch);

            for neighbour in [
                [row_idx, col_idx - 1],
                [row_idx, col_idx + 1],
                [row_idx - 1, col_idx],
                [row_idx + 1, col_idx],
                [row_idx - 1, col_idx - 1],
                [row_idx - 1, col_idx + 1],
                [row_idx + 1, col_idx - 1],
                [row_idx + 1, col_idx + 1],
            ] {
                for number_range in number_ranges.keys() {
                    if is_point_within_direction(&neighbour, number_range) {
                        neighbour_number_ranges.push(number_range);
                    }
                }
            }
        }
    }
    let unique_neighbour_number_ranges: Vec<&Vec<usize>> = neighbour_number_ranges.into_iter().unique().collect();
    for neighbour_number_range in unique_neighbour_number_ranges.iter() {
        let neighbour_number_range_ref: &Vec<usize> = neighbour_number_range;
        numbers.push(number_ranges.get(neighbour_number_range_ref).unwrap().to_string());
    }
    numbers
        .iter()
        .map(|s| s.parse::<u32>().unwrap())
        .sum()

}

// fn part_2(lines: &str) -> &str {
//     return lines;
// }

fn main() {
    let stdout = stdout();
    let lines: Vec<String> = std::fs::read_to_string(std::env::args().nth(1).unwrap())
        .unwrap()
        .lines()
        .map(|l| l.to_string())
        .collect();

    let result = part_1(lines);
    // let result = part_2(INPUT);
    let message = String::from("The solution is: ") + &result.to_string() + &String::from("\n");
    let width = message.chars().count();
    let mut writer = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {

        assert_eq!(
            part_1("467..114..
                   ...*......
                   ..35..633.
                   ......#...
                   617*......
                   .....+.58.
                   ..592.....
                   ......755.
                   ...$.*....
                   .664.598.."
                   .lines().map(|l| l.to_string()).collect()
                  ),
            4361
        );
    }

    // #[test]
    // fn test_part_2() {
    //     assert_eq!(part_2("0"), "0");
    // }
}
