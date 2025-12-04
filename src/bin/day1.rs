use std::cmp::{max, min};
use std::fmt::Display;
use std::fs::File;
use std::io::prelude::*;
use std::num::ParseIntError;

#[derive(Debug)]

enum MainError {
    IOError(std::io::Error),
    ParseError(ParseIntError),
}

impl From<std::io::Error> for MainError {
    fn from(value: std::io::Error) -> Self {
        return Self::IOError(value);
    }
}

impl From<ParseIntError> for MainError {
    fn from(value: ParseIntError) -> Self {
        return Self::ParseError(value);
    }
}

impl Display for MainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IOError(err) => write!(f, "{}", err),
            Self::ParseError(err) => write!(f, "{}", err),
        }
    }
}

// Baked-in statically allcated string: interesting method
// But I want to learn File IO, so won't do this
//const INPUT_STR: &str = include_str!("../../input/day1");

const INPUT_PATH: &'static str = "./src/inputs/day1";

fn line_to_num(l: &str) -> i32 {
    assert!(l.len() > 1); // sanity check

    let (op, num_s) = l.split_at(1);

    let mut sgn: i32;
    let parse_num = num_s.parse::<i32>();
    let num = match parse_num {
        Ok(x) => x,
        Err(err) => panic!(
            "Failed while parsing line {} as i32, got err: {}",
            l,
            MainError::ParseError(err)
        ),
    };

    if op == "L" {
        sgn = -1;
    } else {
        sgn = 1;
    }

    //println!("line: {}, parsed as: {}", l, sgn * num);
    sgn * num
}

fn is_pos(num: i32) -> i32 {
    match num >= 0 {
        true => 1,
        _ => 0,
    }
}

fn is_zero(num: i32) -> i32 {
    match num == 0 {
        true => 1,
        _ => 0,
    }
}

fn main() -> Result<(), MainError> {
    println!("Day 1");

    let mut fp = File::open(INPUT_PATH)?;
    let mut buf = String::new();
    fp.read_to_string(&mut buf)?;

    let nums = buf.lines().map(line_to_num).collect::<Vec<i32>>();
    let ops_zeros = nums.iter().fold((50, 0), |acc, x| {
        let y = (acc.0 + x) % 100;
        (
            y,
            acc.1
                + match y == 0 {
                    true => 1,
                    _ => 0,
                },
        )
    });

    let ops_zero_turns = nums.iter().fold((50_i32, 0_i32), |acc, impulse| {
        let start = acc.0;
        let big_total = start + impulse;

        let mut count = 0;

        let whole = (start + impulse) / 100;
        count += whole.abs();

        let rem = impulse - 100 * whole;
        let smol_total = start + rem;
        print!(
            "start: {}, impulse: {}, big_total: {}, rem: {}, ",
            start, impulse, big_total, rem,
        );
        if smol_total >= 100 || smol_total <= 0 {
            count += 1;
        }

        let end = (smol_total + 100) % 100;
        println!(", count: {}, end: {}", count, end);

        (end, acc.1 + count)
    });

    println!("-5 / 100: {}", (-5 + 100) % 100);
    println!("zeros: {}", ops_zeros.1);
    println!("zeros encountered while turning: {}", ops_zero_turns.1);

    Ok(())
}
