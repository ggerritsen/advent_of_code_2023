use std::cmp::{max, min};

fn main() {

    let input: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let has_nearby_symbol = |x, y, number_length: usize| -> bool {
        let min_x = if x > 0 { x - 1 } else { 0 };
        let max_x = if x < grid.len() { x + 1 } else { grid.len()};

        let row: &Vec<char> = &grid[x];
        let min_y = if y - number_length > 0 { y - number_length - 1 } else { 0 };
        let max_y = if y < row.len() { y + 1 } else { row.len() };

        // println!("Checking for nearby symbol between {},{} and {},{}", min_x, min_y, max_x, max_y);

        for i in min_x..=max_x {
            for j in min_y..max_y {
                let cell = grid[i][j];
                // println!("Checking cell {},{}: {}", i, j, cell);
                if !cell.is_ascii_digit() && cell != '.' {
                    return true;
                }
            }
        }

        false
    };

    // find numbers
    // for each number,
    // * check if there's a symbol in the same row (x-1, x+1)
    // * check if there's a symbol in the previous row (y-1) (x-1, x+1)
    // * check if there's a symbol in the next row (y+1) (x-1, x+1)
    let mut sum = 0;
    for i in 0..grid.len() {
        let mut number = "".to_string();
        for j in 0..grid[i].len() {
            let cell  = grid[i][j];
            if cell.is_ascii_digit() {
                println!("Found number {} at location {},{}", cell, i, j);
                number.push(cell);
                continue;
            } else {
                if number.len() > 0 {
                    let full_number = number.parse::<i32>().unwrap();
                    // println!("Found full number {} at location {},{}", full_number, i, j);

                    if has_nearby_symbol(i, j, number.len()) {
                        println!("Found part number {} at location {},{}", full_number, i, j);
                        sum += full_number;
                    }
                    number.clear();
                }
            }

        }
    }

    println!("Sum is {}", sum);

}
