extern crate lazy_static;
extern crate regex;

use std::io::BufReader;
use std::io::Read;
use std::io::Result;
use std::io::prelude::*;

use self::lazy_static::lazy_static;
use self::regex::Regex;

struct Point {
    x: isize,
    y: isize,
    dx: isize,
    dy: isize,
}

pub fn challenge1<R: Read>(reader: BufReader<R>) -> Result<()> {
    let mut points = Vec::new();
    for line in reader.lines() {
        points.push(read_point(&line?));
    }
    calculate_message(&mut points);
    println!("Answer:");
    show_points(&points);
    Ok(())
}

pub fn challenge2<R: Read>(reader: BufReader<R>) -> Result<()> {
    let mut points = Vec::new();
    for line in reader.lines() {
        points.push(read_point(&line?));
    }
    println!("Answer: {}", calculate_message(&mut points));
    Ok(())
}

fn calculate_message(points: &mut Vec<Point>) -> usize {
    let mut best_score = calc_score(&points);
    let mut elapsed = 0;
    loop {
        move_points(points, 1);
        let score = calc_score(&points);
        if score > best_score {
            move_points(points, -1);
            break;
        }
        elapsed += 1;
        best_score = score;
    }
    elapsed
}

fn read_point(line: &str) -> Point {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"position=<(?P<x>[^,]+),(?P<y>[^>]+)> velocity=<(?P<dx>[^,]+),(?P<dy>[^>]+)>").unwrap();
    }
    let cap = RE.captures(&line).unwrap();
    let extract = |name| cap.name(name).unwrap().as_str().trim().parse().unwrap();
    Point {
        x: extract("x"),
        y: extract("y"),
        dx: extract("dx"),
        dy: extract("dy"),
    }
}

fn move_points(points: &mut Vec<Point>, mul: i8) {
    for point in points {
        point.x += point.dx * mul as isize;
        point.y += point.dy * mul as isize;
    }
}

fn calc_score(points: &[Point]) -> usize {
    let min_x = points.iter().map(|p| p.x).min().unwrap();
    let max_x = points.iter().map(|p| p.x).max().unwrap();
    let min_y = points.iter().map(|p| p.y).min().unwrap();
    let max_y = points.iter().map(|p| p.y).max().unwrap();
    ((max_x - min_x) + (max_y - min_y)) as usize
}

fn show_points(points: &[Point]) {
    let min_x = points.iter().map(|p| p.x).min().unwrap();
    let max_x = points.iter().map(|p| p.x).max().unwrap();
    let min_y = points.iter().map(|p| p.y).min().unwrap();
    let max_y = points.iter().map(|p| p.y).max().unwrap();
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if points.iter().any(|p| p.x == x && p.y == y) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
