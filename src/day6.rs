use std::collections::HashMap;
use std::io::BufReader;
use std::io::Read;
use std::io::Result;
use std::io::prelude::*;

struct Coord {
    id: i32,
    top: i32,
    left: i32
}

struct Field {
    off_top: i32,
    off_left: i32,
    cells: Vec<Vec<i32>>
}

pub fn challenge1<R: Read>(reader: BufReader<R>) -> Result<()> {
    let coords = read_coords(reader);
    let mut field = Field::new(&coords);
    field.mark(&coords);
    println!("Answer: {}", field.max_inner());
    Ok(())
}

pub fn challenge2<R: Read>(reader: BufReader<R>) -> Result<()> {
    let coords = read_coords(reader);
    let field = Field::new(&coords);
    println!("Answer: {}", field.count_dist_sum_less_than(&coords, 10000));
    Ok(())
}

fn read_coords<R: Read>(reader: BufReader<R>) -> Vec<Coord> {
    let mut coords = Vec::new();
    for (id, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let mut split = line.split(", ");
        coords.push(Coord {
            id: id as i32 + 1,
            left: split.next().unwrap().parse().unwrap(),
            top: split.next().unwrap().parse().unwrap(),
        });
    }
    coords
}

impl Field {
    fn new(coords: &[Coord]) -> Field {
        let min_top = coords.iter().map(|c| c.top).min().unwrap();
        let max_top = coords.iter().map(|c| c.top).max().unwrap();
        let min_left = coords.iter().map(|c| c.left).min().unwrap();
        let max_left = coords.iter().map(|c| c.left).max().unwrap();
        let height = max_top - min_top + 3;
        let width = max_left - min_left + 3;
        Field {
            off_top: 1 - min_top,
            off_left: 1 - min_left,
            cells: vec![vec![0; width as usize]; height as usize]
        }
    }

    fn mark(&mut self, coords: &[Coord]) {
        for top in 0..self.cells.len() {
            let row = &self.cells[top];
            for left in 0..row.len() {
                let mut nearest = 0;
                let mut tie = false;
                let mut nearest_dist = u32::max_value();
                for coord in coords {
                    let (c_top, c_left) = self.translate(coord);
                    let dist = ((c_top as i32 - top as i32).abs() + (c_left as i32 - left as i32).abs()) as u32;
                    if dist == nearest_dist {
                        tie = true;
                    } else if dist < nearest_dist {
                        tie = false;
                        nearest = coord.id;
                        nearest_dist = dist;
                    }
                    self.cells[top][left] = if tie { -1 } else { nearest };
                }
            }
        }
    }

    fn max_inner(&self) -> u32 {
        let mut counts: HashMap<u32, i32> = HashMap::new();
        let height = self.cells.len();
        for (top, row) in self.cells.iter().enumerate() {
            let width = row.len();
            for (left, nearest) in row.iter().enumerate() {
                if *nearest == -1 {
                    continue;
                }
                let nearest = *nearest as u32;
                if top == 0 || left == 0 || top == height - 1 || left == width - 1 {
                    counts.insert(nearest, -1);
                    continue;
                }
                counts.entry(nearest)
                    .and_modify(|n| if *n != -1 { *n += 1 })
                    .or_insert(1);
            }
        }
        *counts.values().max().unwrap() as u32
    }

    fn count_dist_sum_less_than(&self, coords: &[Coord], max_dist_sum: u32) -> u32 {
        let mut count = 0;
        for top in 0..self.cells.len() {
            for left in 0..self.cells[top].len() {
                let mut dist_sum = 0;
                for coord in coords {
                    let (c_top, c_left) = self.translate(coord);
                    let dist = ((c_top as i32 - top as i32).abs() + (c_left as i32 - left as i32).abs()) as u32;
                    dist_sum += dist;
                }
                if dist_sum < max_dist_sum {
                    count += 1;
                }
            }
        }
        count
    }

    fn translate(&self, coord: &Coord) -> (usize, usize) {
        ((coord.top + self.off_top) as usize, (coord.left + self.off_left) as usize)
    }

    #[allow(dead_code)]
    fn print(&self) {
        for row in self.cells.iter() {
            for c in row {
                print!("{}", match c {
                    -1 => '×',
                    0 => '·',
                    n @ 1...28 => char::from(96 + *n as u8),
                    n @ 29...64 => char::from(64 + *n as u8),
                    _ => panic!("TOO BIG"),
                })
            }
            println!();
        }
    }
}
