#[allow(clippy::too_many_lines)]
fn main() {
    let input: &str = include_str!("input.txt");

    let test_input: &str = ".467*35...
..........
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

    // find parts
    let mut parts: Vec<Part> = Vec::new();
    for (x, row) in grid.iter().enumerate() {
        let mut number: Vec<char> = Vec::new();
        for (y, cell) in row.iter().enumerate() {
            if cell.is_ascii_digit() {
                number.push(*cell);
            }

            if (y == row.len() - 1 || !cell.is_ascii_digit()) && !number.is_empty() {
                println!("found a number: {number:?}");
                // found a number
                let value: i32 = number
                    .iter()
                    .fold(String::new(), |collector, c| format!("{collector}{c}"))
                    .parse()
                    .unwrap();
                parts.push(Part {
                    value,
                    len: number.len(),
                    x,
                    y: y - 1,
                });
                number.clear();
            }
        }
    }

    for p in &parts {
        println!("part: {p:?}");
    }

    let find_two_nearby_parts = |x, y: usize| -> Option<i32> {
        let min_x = if x > 0 { x - 1 } else { 0 };
        let max_x = if x < grid.len() - 1 {
            x + 1
        } else {
            grid.len() - 1
        };
        println!("min_x, max_x: {min_x}, {max_x}");

        let min_y = if y > 0 { y - 1 } else { 0 };
        let max_y = if y < grid[0].len() - 1 {
            y + 1
        } else {
            grid[0].len() - 1
        };
        println!("min_y, max_y: {min_y}, {max_y}");

        let mut found_part_numbers: Vec<i32> = Vec::new();
        for p in &parts {
            if min_x <= p.x
                && p.x <= max_x
                && ((min_y <= p.y && p.y <= max_y) || (p.y > max_y && p.y - (p.len - 1) <= max_y))
            {
                found_part_numbers.push(p.value);
            }
        }
        if !found_part_numbers.is_empty() {
            println!("{found_part_numbers:?}");
        }

        if found_part_numbers.len() == 2 {
            return Some(found_part_numbers[0] * found_part_numbers[1]);
        }

        None
    };

    let mut product_sum = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let cell = grid[i][j];
            if cell == '*' {
                println!("Found gear at location {i},{j}");
                let product = find_two_nearby_parts(i, j);
                if product.is_some() {
                    product_sum += product.unwrap();
                }
            }
        }
    }

    println!("Product sum is {product_sum}");
}

#[derive(Debug)]
struct Part {
    value: i32,
    len: usize,
    x: usize,
    y: usize,
}
