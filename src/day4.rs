extern crate chrono;
extern crate lazy_static;
extern crate regex;

use std::collections::HashMap;
use std::fmt;
use std::io::BufReader;
use std::io::Read;
use std::io::Result;
use std::io::prelude::*;

use self::chrono::{NaiveDate, NaiveDateTime, Datelike, Timelike};
use self::lazy_static::lazy_static;
use self::regex::Regex;


#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Event {
    BeginShift(GuardId),
    FallAsleep,
    WakeUp,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct LogEntry {
    timestamp: NaiveDateTime,
    event: Event,
}

impl fmt::Debug for LogEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let d = self.timestamp.date();
        let t = self.timestamp.time();
        write!(f, "[{:02}-{:02} {:02}] ", d.month(), d.day(), t.minute())?;
        match self.event {
            Event::BeginShift(id) => write!(f, "#{} begin", id),
            Event::FallAsleep => write!(f, "sleep"),
            Event::WakeUp => write!(f, "wake"),
        }
    }
}

type SleepMinutes = Box<[u16; 60]>;

type GuardId = u32;

pub fn challenge1<R: Read>(reader: BufReader<R>) -> Result<()> {
    challenge(reader, strategy1)
}

pub fn challenge2<R: Read>(reader: BufReader<R>) -> Result<()> {
    challenge(reader, strategy2)
}

fn challenge<R: Read>(reader: BufReader<R>,
        strategy: fn(&HashMap<GuardId, SleepMinutes>) -> (GuardId, u8)) -> Result<()> {
    let mut log = Vec::new();
    for line in reader.lines() {
        log.push(read_entry(&line?));
    }
    log.sort();
    let guard_sleep_minutes = fill_sleep_minutes(log);
    let (id, minute) = strategy(&guard_sleep_minutes);
    println!("Answer: {} x {} = {}", id, minute, id * u32::from(minute));
    Ok(())
}

fn read_entry(line: &str) -> LogEntry {
    lazy_static! {
        static ref ENTRY: Regex = Regex::new(r"\[(\d+)-(\d+)-(\d+) (\d+):(\d+)\] (?P<event>.+)").unwrap();
        static ref SHIFT: Regex = Regex::new(r"Guard #(\d+) begins shift").unwrap();
    }
    let cap = ENTRY.captures(&line).unwrap();
    let dt: Vec<u32> = (1..6).map(|i| cap[i].parse::<u32>().unwrap()).collect();
    LogEntry {
        timestamp: NaiveDate::from_ymd(dt[0] as i32, dt[1], dt[2]).and_hms(dt[3], dt[4], 0),
        event: match cap.name("event").unwrap().as_str() {
            "falls asleep" => Event::FallAsleep,
            "wakes up" => Event::WakeUp,
            begin_shift => Event::BeginShift(SHIFT.captures(begin_shift).unwrap()[1].parse().unwrap()),
        }
    }
}

fn fill_sleep_minutes(log: Vec<LogEntry>) -> HashMap<GuardId, SleepMinutes> {
    let mut gsm = HashMap::new();
    let mut minute = 0;
    let mut asleep = false;
    let mut sleep_minutes = &mut Box::new([0; 60]);
    for entry in log {
        // println!("{:?}", entry);
        let next_minute = entry.timestamp.time().minute() as usize;
        match entry.event {
            Event::BeginShift(id) => {
                if asleep {
                    for sm in &mut sleep_minutes[minute..] {
                        *sm += 1;
                    }
                }
                minute = 0;
                asleep = false;
                sleep_minutes = gsm.entry(id).or_insert_with(|| Box::new([0; 60]));
            },
            Event::FallAsleep => {
                minute = next_minute;
                asleep = true;
            },
            Event::WakeUp => {
                for sm in &mut sleep_minutes[minute..next_minute] {
                    *sm += 1;
                }
                minute = next_minute;
                asleep = false;
            },
        }
    }
    gsm
}

fn strategy1(gsm: &HashMap<GuardId, SleepMinutes>) -> (GuardId, u8) {
    let (id, minutes) = gsm.iter().max_by_key(|(_, minutes)| -> u16 { minutes.iter().sum() }).unwrap();
    let (minute, _) = minutes.iter().enumerate().max_by_key(|(_, v)| *v).unwrap();
    (*id, minute as u8)
}

fn strategy2(gsm: &HashMap<GuardId, SleepMinutes>) -> (GuardId, u8) {
    let (id, minutes) = gsm.iter().max_by_key(|(_, minutes)| minutes.iter().max().unwrap()).unwrap();
    let (minute, _) = minutes.iter().enumerate().max_by_key(|(_, v)| *v).unwrap();
    (*id, minute as u8)
}
