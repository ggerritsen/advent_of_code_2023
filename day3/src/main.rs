fn main() {
    let input: &str = include_str!("input.txt");

    let _input: &str = "467..114..
...*......
..35..6330
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
        let max_x = if x < grid.len() - 1 {
            x + 1
        } else {
            grid.len() - 1
        };

        let row: &Vec<char> = &grid[x];
        let min_y = if y - number_length > 0 {
            y - number_length - 1
        } else {
            0
        };
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
        let mut number = String::new();
        for j in 0..grid[i].len() {
            let cell = grid[i][j];
            if cell.is_ascii_digit() {
                println!("Found number {cell} at location {i},{j}");
                number.push(cell);

                // if we're at the end of a row, then the number is full
                if j == grid[i].len() - 1 {
                    let full_number = number.parse::<i32>().unwrap();
                    // println!("Found full number {} at location {},{}", full_number, i, j);

                    if has_nearby_symbol(i, j, number.len()) {
                        println!("Found part number {full_number} at location {i},{j}");
                        sum += full_number;
                    }
                    number.clear();
                }

                continue;
            }
            if !number.is_empty() {
                let full_number = number.parse::<i32>().unwrap();
                // println!("Found full number {} at location {},{}", full_number, i, j);

                if has_nearby_symbol(i, j, number.len()) {
                    println!("Found part number {full_number} at location {i},{j}");
                    sum += full_number;
                }
                number.clear();
            }
        }
    }

    println!("Sum is {sum}");

    let find_two_nearby_parts = |x, y: usize| -> Option<i32> {
        return Some(0);
    };

    let mut product_sum = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let cell = grid[i][j];
            if cell == '*' {
                let product = find_two_nearby_parts(i, j);
                if product.is_some() {
                    println!("Found gear at location {i},{j}");
                    product_sum += product.unwrap();
                }
            }
        }
    }

    println!("Product sum is {product_sum}");
}
