extern crate lazy_static;
extern crate regex;

use std::io::BufReader;
use std::io::Read;
use std::io::Result;

pub fn challenge1<R: Read>(mut reader: BufReader<R>) -> Result<()> {
    let mut line = String::new();
    reader.read_to_string(&mut line)?;
    let serial: usize = line.trim().parse().unwrap();
    let grid = generate_grid(serial);
    let (x, y) = find_best_3x3(&grid);
    println!("Answer: {},{}", x, y);
    Ok(())
}

const SIZE: usize = 300;

fn generate_grid(serial: usize) -> Vec<Vec<i8>> {
    let mut grid = Vec::with_capacity(SIZE);
    for y in 1..=SIZE {
        let mut row = Vec::with_capacity(SIZE);
        for x in 1..=SIZE {
            let rack_id = x + 10;
            let mut power = rack_id * y;
            power += serial;
            power *= rack_id;
            power = (power - (power / 1000 * 1000)) / 100;
            row.push(power as i8 - 5);
        }
        grid.push(row);
    }
    grid
}

fn find_best_3x3(grid: &[Vec<i8>]) -> (usize, usize) {
    let mut best_sum = i8::min_value();
    let mut best_x = 0;
    let mut best_y = 0;
    for (y, three_rows) in grid.windows(3).enumerate() {
        let wins1 = three_rows[0].windows(3);
        let wins2 = three_rows[1].windows(3);
        let wins3 = three_rows[2].windows(3);
        for (x, ((row1, row2), row3)) in wins1.zip(wins2).zip(wins3).enumerate() {
            let sum = [row1, row2, row3].iter()
                .flat_map(|row| row.iter())
                .sum();
            if sum > best_sum {
                best_sum = sum;
                best_x = x;
                best_y = y;
            }
        }
    }
    (best_x + 1, best_y + 1)
}

pub fn challenge2<R: Read>(mut reader: BufReader<R>) -> Result<()> {
    let mut line = String::new();
    reader.read_to_string(&mut line)?;
    let serial: usize = line.trim().parse().unwrap();
    let grid = generate_grid(serial);
    let (x, y, size) = find_best_any_size(&grid);
    println!("Answer: {},{},{}", x, y, size);
    Ok(())
}

fn find_best_any_size(grid: &[Vec<i8>]) -> (usize, usize, usize) {
    let mut best_sum = i32::min_value();
    let mut best_x = 0;
    let mut best_y = 0;
    let mut best_size = 0;
    for size in 1..=SIZE {
        for y in 0..=SIZE-size {
            for x in 0..=SIZE-size {
                let mut sum = 0;
                for i in 0..size {
                    for j in 0..size {
                        sum += i32::from(grid[y + i][x + j]);
                    }
                }
                if sum > best_sum {
                    best_sum = sum;
                    best_x = x;
                    best_y = y;
                    best_size = size;
                }
            }
        }
    }
    (best_x + 1, best_y + 1, best_size)
}
