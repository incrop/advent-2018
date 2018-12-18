extern crate lazy_static;
extern crate regex;

use std::collections::HashSet;
use std::io::BufReader;
use std::io::Read;
use std::io::Result;
use std::io::prelude::*;

use self::lazy_static::lazy_static;
use self::regex::Regex;

#[derive(Copy, Clone)]
enum Cell {
    Unclaimed,
    ClaimedBy(usize),
    Overclaimed,
}

struct Claim {
    id: usize,
    top: usize,
    left: usize,
    width: usize,
    height: usize,
}

const SIZE: usize = 1000;
type Fabric = [Vec<Cell>];

pub fn challenge1<R: Read>(reader: BufReader<R>) -> Result<()> {
    let mut fabric = vec![vec![Cell::Unclaimed; SIZE]; SIZE];
    for line in reader.lines() {
        let claim = read_claim(&line?);
        apply_claim(&mut fabric, &claim);
    }
    println!("Answer: {}", count_overclaimed(&fabric));
    Ok(())
}

fn read_claim(line: &str) -> Claim {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"#(?P<id>\d+) @ (?P<left>\d+),(?P<top>\d+): (?P<width>\d+)x(?P<height>\d+)").unwrap();
    }
    let cap = RE.captures(&line).unwrap();
    let extract = |name| cap.name(name).unwrap().as_str().parse().unwrap();
    Claim {
        id: extract("id"),
        top: extract("top"),
        left: extract("left"),
        width: extract("width"),
        height: extract("height"),
    }
}

fn apply_claim(fabric: &mut Fabric, claim: &Claim) {
    for row in &mut fabric[claim.top .. claim.top + claim.height] {
        for cell in &mut row[claim.left .. claim.left + claim.width] {
            match cell {
                Cell::Unclaimed => *cell = Cell::ClaimedBy(claim.id),
                Cell::ClaimedBy(_) => *cell = Cell::Overclaimed,
                Cell::Overclaimed => (),
            }
        }
    }
}

fn count_overclaimed(fabric: &Fabric) -> usize {
    fabric.iter()
        .map(|row| row.iter())
        .flatten()
        .map(|cell| match cell {
            Cell::Overclaimed => 1,
            _ => 0,
        })
        .sum()
}

pub fn challenge2<R: Read>(reader: BufReader<R>) -> Result<()> {
    let mut fabric = vec![vec![Cell::Unclaimed; SIZE]; SIZE];
    let mut non_overlapping = HashSet::new();
    for line in reader.lines() {
        let claim = read_claim(&line?);
        non_overlapping.insert(claim.id);
        apply_claim_with_overlap_check(&mut fabric, &mut non_overlapping, &claim);
    }
    assert!(non_overlapping.len() == 1);
    println!("Answer: {}", non_overlapping.iter().next().unwrap());
    Ok(())
}

fn apply_claim_with_overlap_check(fabric: &mut Fabric, non_overlapping: &mut HashSet<usize>, claim: &Claim) {
    for row in &mut fabric[claim.top .. claim.top + claim.height] {
        for cell in &mut row[claim.left .. claim.left + claim.width] {
            match cell {
                Cell::Unclaimed => *cell = Cell::ClaimedBy(claim.id),
                Cell::ClaimedBy(ref other_id) => {
                    non_overlapping.remove(&claim.id);
                    non_overlapping.remove(other_id);
                    *cell = Cell::Overclaimed;
                },
                Cell::Overclaimed => {
                    non_overlapping.remove(&claim.id);
                },
            }
        }
    }
}
