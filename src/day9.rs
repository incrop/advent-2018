extern crate lazy_static;
extern crate regex;

use std::io::BufReader;
use std::io::Read;
use std::io::Result;

use self::lazy_static::lazy_static;
use self::regex::Regex;

pub fn challenge<R: Read>(mut reader: BufReader<R>) -> Result<()> {
    let mut line = String::new();
    reader.read_to_string(&mut line)?;
    let (players, points) = read_input(&line);
    println!("Answer: {}", calc_high_score(players, points));
    Ok(())
}

fn read_input(line: &str) -> (usize, usize) {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"(?P<players>\d+) players; last marble is worth (?P<points>\d+) points").unwrap();
    }
    let cap = RE.captures(&line).unwrap();
    let extract = |name| cap.name(name).unwrap().as_str().parse().unwrap();
    (extract("players"), extract("points"))
}

#[derive(Debug)]
struct Node {
    prev_idx: usize,
    next_idx: usize,
}

impl Node {
    fn new(prev_idx: usize, next_idx: usize) -> Node {
        Node {prev_idx, next_idx}
    }
}

fn calc_high_score(players: usize, points: usize) -> u64 {
    let mut scores = vec![0u64; players];
    let mut board = Vec::with_capacity(points + 1);
    board.push(Node::new(0, 0));
    let mut curr_idx = 0;
    let mut player = 0;
    for score in 1..=points {
        if score % 23 == 0 {
            for _ in 0..7 {
                curr_idx = board[curr_idx].prev_idx;
            }
            scores[player] += score as u64 + curr_idx as u64;
            let Node {prev_idx, next_idx} = board[curr_idx];
            board[prev_idx].next_idx = next_idx;
            board[next_idx].prev_idx = prev_idx;
            curr_idx = next_idx;
            board.push(Node::new(0, 0));
        } else {
            let prev_idx = board[curr_idx].next_idx;
            let next_idx = board[prev_idx].next_idx;
            curr_idx = board.len();
            let node = Node::new(prev_idx, next_idx);
            board.push(node);
            board[prev_idx].next_idx = curr_idx;
            board[next_idx].prev_idx = curr_idx;
        }
        player = (player + 1) % players;
    }
    *scores.iter().max().unwrap()
}
