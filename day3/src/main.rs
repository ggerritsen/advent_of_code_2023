use std::cmp::{max, min};
use grid::*;

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

    let mut grid: Grid<char> = Grid::new(input.lines().count(), input.lines().nth(0).unwrap().len());
    for line_count in 0..input.lines().count() {
        let col = input.lines().nth(line_count).unwrap();
        for col_count in 0..col.len() {
            let _ = grid.get_mut(line_count, col_count).insert(col.chars().nth(col_count).as_mut().unwrap());
        }
    }

    //  = input.lines().map(|line| line.chars().collect::<Vec<char>>()).collect();

    grid.iter().for_each(|x| println!("{:?}", x));

    // // find symbols
    // let mut symbol_locations: Vec<Vec<i32>> = Vec::new();
    // for row in &grid {
    //     let mut cols: Vec<i32> = Vec::new();
    //     for i in 0..row.len() {
    //         if !row[i].is_ascii_digit() && row[i] != '.' {
    //             println!("Found symbol {} at location {}", row[i], i);
    //             cols.push(i as i32);
    //         }
    //     }
    //     symbol_locations.push(cols);
    // }
    //
    // // find numbers
    // // for each number,
    // // * check if there's a symbol in the same row (x-1, x+1)
    // // * check if there's a symbol in the previous row (y-1) (x-1, x+1)
    // // * check if there's a symbol in the next row (y+1) (x-1, x+1)
    // let mut sum = 0;
    // for i in 0..grid.len() {
    //     let mut number: Vec<char> = Vec::new();
    //     for j in 0..grid[i].len() {
    //         let cell  = grid[i][j-1];
    //         if cell.is_ascii_digit() {
    //             println!("Found number {} at location {},{}", cell, i, j-1);
    //             number.push(cell);
    //             continue;
    //         } else {
    //             if number.len() > 0 {
    //                 println!("Found full number len {} at location {},{}", number.len(), i, j-1);
    //
    //                 let do_it = |ui| -> Result<i32, String> {
    //                     return Result::Ok(j-number.len()-1);
    //                 };
    //                 let res = do_it(j);
    //                 let min_x = max(0, res.unwrap_or(0));
    //
    //                 // let min_x = max(j-number.len()-1, 0);
    //                 let max_x = min(j+1, grid[i].len());
    //
    //                 symbol_locations[i].iter().for_each(|&x| {
    //                     if x >= min_x as i32 && x < max_x as i32 {
    //                         println!("Found symbol {} at location {},{}", grid[i][x as usize], i, x);
    //                         let mut g = "".to_string();
    //                         number.iter().for_each(|&x| g.push(x));
    //                         println!("Adding to sum {}", g);
    //                         sum += g.parse::<i32>().unwrap();
    //                     }
    //                 });
    //
    //                 let min_y = max((i as i32)-1, 0);
    //                 symbol_locations[min_y as usize].iter().for_each(|&x| {
    //                     if x >= min_x as i32 && x < max_x as i32 {
    //                         println!("Found symbol {} at location {},{}", grid[i][x as usize], i, x);
    //                         let mut g = "".to_string();
    //                         number.iter().for_each(|&x| g.push(x));
    //                         println!("Adding to sum {}", g);
    //                         sum += g.parse::<i32>().unwrap();
    //                     }
    //                 });
    //
    //                 let max_y = min(i+1, grid[i].len());
    //                 symbol_locations[max_y-1].iter().for_each(|&x| {
    //                     if x >= min_x as i32 && x < max_x as i32 {
    //                         println!("Found symbol {} at location {},{}", grid[i][x as usize], i, x);
    //                         let mut g = "".to_string();
    //                         number.iter().for_each(|&x| g.push(x));
    //                         println!("Adding to sum {}", g);
    //                         sum += g.parse::<i32>().unwrap();
    //                     }
    //                 });
    //
    //                 number.clear();
    //             }
    //         }
    //
    //     }
    // }
    //
    // println!("Sum is {}", sum);

    // for row in &grid {
    //     for col in row {
    //         print!("{}", col);
    //     }
    //     println!("");
    // }

}
