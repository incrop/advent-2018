use std::collections::HashMap;
use std::io::{BufReader, Read, BufRead, Result};

#[derive(Copy, Clone)]
enum Dir {
    North,
    East,
    West,
    South,
}

impl Dir {
    fn apply_turn(self, turn: Turn) -> Dir {
        match (self, turn) {
            (Dir::North, Turn::Left) => Dir::West,
            (Dir::East, Turn::Left) => Dir::North,
            (Dir::West, Turn::Left) => Dir::South,
            (Dir::South, Turn::Left) => Dir::East,
            (Dir::North, Turn::Right) => Dir::East,
            (Dir::East, Turn::Right) => Dir::South,
            (Dir::West, Turn::Right) => Dir::North,
            (Dir::South, Turn::Right) => Dir::West,
            (dir, Turn::Forward) => dir,
        }
    }
}

#[derive(Copy, Clone)]
enum Cell {
    Empty,
    Vertical,
    Horizontal,
    ForwardSlash,
    BackwardSlash,
    Intersection,
}

#[derive(Copy, Clone)]
enum Turn {
    Left,
    Right,
    Forward,
}

struct Cart {
    x: usize,
    y: usize,
    dir: Dir,
    turn: Turn,
}

impl Cart {
    fn new(x: usize, y: usize, dir: Dir) -> Cart {
        Cart {x, y, dir, turn: Turn::Left}
    }

    fn try_advance(&mut self, collisions: &mut HashMap<(usize, usize), usize>) -> bool {
        if *collisions.get(&(self.x , self.y)).unwrap() > 1 {
            return false;
        }
        collisions.remove(&(self.x , self.y));
        match self.dir {
            Dir::North => self.y -= 1,
            Dir::East => self.x += 1,
            Dir::West => self.x -= 1,
            Dir::South => self.y += 1,
        }
        let collision = collisions.entry((self.x , self.y)).or_insert(0);
        *collision += 1;
        *collision == 1
    }

    fn turn(&mut self, map: &[Vec<Cell>]) {
        let cell = map[self.y][self.x];
        let turn = match cell {
            Cell::ForwardSlash => match self.dir {
                Dir::North | Dir::South => Turn::Right,
                Dir::East | Dir::West => Turn::Left,
            }
            Cell::BackwardSlash => match self.dir {
                Dir::North | Dir::South => Turn::Left,
                Dir::East | Dir::West => Turn::Right,
            }
            Cell::Intersection => self.turn,
            _ => Turn::Forward,
        };
        self.dir = self.dir.apply_turn(turn);
        if let Cell::Intersection = cell {
            self.turn = match self.turn {
                Turn::Left => Turn::Forward,
                Turn::Forward => Turn::Right,
                Turn::Right => Turn::Left,
            }
        }
    }
}

pub fn challenge1<R: Read>(reader: BufReader<R>) -> Result<()> {
    let (map, mut carts) = read_map(reader.lines().map(|l| l.unwrap()));
    let (x, y) = find_collision(&map, &mut carts);
    println!("Answer: {},{}", x, y);
    Ok(())
}

pub fn challenge2<R: Read>(reader: BufReader<R>) -> Result<()> {
    let (map, mut carts) = read_map(reader.lines().map(|l| l.unwrap()));
    let (x, y) = find_last_standing(&map, &mut carts);
    println!("Answer: {},{}", x, y);
    Ok(())
}

fn read_map<I: Iterator<Item=String>>(lines: I) -> (Vec<Vec<Cell>>, Vec<Cart>) {
    let mut map = Vec::new();
    let mut carts = Vec::new();
    for (y, line) in lines.enumerate() {
        let mut row = Vec::new();
        for (x, chr) in line.chars().enumerate() {
            row.push(match chr {
                ' ' => Cell::Empty,
                '/' => Cell::ForwardSlash,
                '\\' => Cell::BackwardSlash,
                '|' => Cell::Vertical,
                '-' => Cell::Horizontal,
                '+' => Cell::Intersection,
                'v' => {
                    carts.push(Cart::new(x, y, Dir::South));
                    Cell::Vertical
                },
                '^' => {
                    carts.push(Cart::new(x, y, Dir::North));
                    Cell::Vertical
                },
                '>' => {
                    carts.push(Cart::new(x, y, Dir::East));
                    Cell::Horizontal
                },
                '<' => {
                    carts.push(Cart::new(x, y, Dir::West));
                    Cell::Horizontal
                },
                c => panic!("Unexpected character: {}", c),
            });
        }
        map.push(row);
    }
    (map, carts)
}

fn find_collision(map: &[Vec<Cell>], carts: &mut Vec<Cart>) -> (usize, usize) {
    let mut collisions: HashMap<(usize, usize), usize> = carts.iter()
        .map(|cart| ((cart.x, cart.y), 1))
        .collect();
    loop {
        carts.sort_by_key(|cart| (cart.y, cart.x));
        for cart in carts.iter_mut() {
            if !cart.try_advance(&mut collisions) {
                return (cart.x, cart.y);
            }
            cart.turn(&map);
        }
    }
}

fn find_last_standing(map: &[Vec<Cell>], carts: &mut Vec<Cart>) -> (usize, usize) {
    let mut collisions: HashMap<(usize, usize), usize> = carts.iter()
        .map(|cart| ((cart.x, cart.y), 1))
        .collect();
    while carts.len() > 1 {
        carts.sort_by_key(|cart| (cart.y, cart.x));
        let mut crash_happened = false;
        for cart in carts.iter_mut() {
            if !cart.try_advance(&mut collisions) {
                crash_happened = true
            }
            cart.turn(&map);
        }
        if crash_happened {
            carts.retain(|cart| collisions[&(cart.x , cart.y)] == 1);
            collisions.retain(|_, n| *n == 1);
        }
    }
    (carts[0].x, carts[0].y)
}
