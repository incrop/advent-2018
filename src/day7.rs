extern crate lazy_static;
extern crate regex;

use std::cmp::{Ordering};
use std::collections::{HashSet, HashMap, BinaryHeap};
use std::io::BufReader;
use std::io::Read;
use std::io::Result;
use std::io::prelude::*;

use self::lazy_static::lazy_static;
use self::regex::Regex;

pub fn challenge1<R: Read>(reader: BufReader<R>) -> Result<()> {
    let mut deps = read_deps(reader);
    let mut result = String::new();
    while !deps.is_empty() {
        let step = pick_step(&deps);
        result.push(step);
        finish_step(&mut deps, step);
    }
    println!("Answer: {}", result);
    Ok(())
}

type Deps = HashMap<char, HashSet<char>>;

fn read_deps<R: Read>(reader: BufReader<R>) -> Deps {
    let mut deps = HashMap::new();
    for line in reader.lines() {
        let (before, after) = read_dep(&line.unwrap());
        deps.entry(before)
            .or_insert_with(HashSet::new);
        deps.entry(after)
            .or_insert_with(HashSet::new)
            .insert(before);
    }
    deps
}

fn pick_step(deps: &Deps) -> char {
    deps.iter()
        .filter(|(_, d)| d.is_empty())
        .map(|(step, _)| *step)
        .min()
        .unwrap()
}

fn pick_avail_step(deps: &Deps, workers: &BinaryHeap<Worker>) -> Option<char> {
    deps.iter()
        .filter(|(_, d)| d.is_empty())
        .filter(|(step, _)| !workers.iter().any(|w| w.step == **step))
        .map(|(step, _)| *step)
        .min()
}

fn finish_step(deps: &mut Deps, step: char) {
    deps.remove(&step);
    for d in deps.values_mut() {
        d.remove(&step);
    }
}

fn read_dep(line: &str) -> (char, char) {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"Step (?P<before>[A-Z]) must be finished before step (?P<after>[A-Z]) can begin.").unwrap();
    }
    let cap = RE.captures(&line).unwrap();
    let extract = |name| cap.name(name).unwrap().as_str().chars().next().unwrap();
    (extract("before"), extract("after"))
}

#[derive(Clone, Copy, Debug)]
struct Worker {
    step: char,
    finish_at: usize,
}

impl Worker {
    fn new() -> Worker {
        Worker {
            step: ' ',
            finish_at: 0,
        }
    }
}

impl Ord for Worker {
    fn cmp(&self, other: &Self) -> Ordering {
        (other.finish_at).cmp(&self.finish_at) // reverse order for BinaryHeap
    }
}

impl PartialOrd for Worker {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Worker {
    fn eq(&self, other: &Self) -> bool {
        self.finish_at == other.finish_at
    }
}

impl Eq for Worker {}

pub fn challenge2<R: Read>(reader: BufReader<R>) -> Result<()> {
    let mut deps = read_deps(reader);
    let mut workers: BinaryHeap<Worker> = BinaryHeap::new();
    let mut idle = 5;
    let mut now = 0;
    while !deps.is_empty() {
        match (pick_avail_step(&deps, &workers), idle) {
            (Some(step), 0) => {
                let mut worker = workers.pop().unwrap();
                finish_step(&mut deps, worker.step);
                now = worker.finish_at;
                worker.step = step;
                worker.finish_at = now + 60 + (step as u32 - 'A' as u32 + 1) as usize;
                workers.push(worker);
            },
            (Some(step), _) => {
                idle -= 1;
                let mut worker = Worker::new();
                worker.step = step;
                worker.finish_at = now + 60 + (step as u32 - 'A' as u32 + 1) as usize;
                workers.push(worker);
            }
            (None, _) => {
                let worker = workers.pop().unwrap();
                finish_step(&mut deps, worker.step);
                now = worker.finish_at;
                idle += 1;
            }
        }
    }
    println!("Answer: {}", now);
    Ok(())
}
