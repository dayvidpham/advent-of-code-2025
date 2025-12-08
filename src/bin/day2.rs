extern crate advent;
use crate::advent::prelude::*;
pub use std::io::prelude::*;

use std::{iter::zip, str::Chars};

const INPUT_PATH: &'static str = "./src/inputs/day2";

fn main() -> Result<(), MainError> {
    let mut input: String = Default::default();
    let mut data = File::open(INPUT_PATH)?;
    data.read_to_string(&mut input)?;
    let (starts, ends) = parse_input(input)?;

    println!("-----------------------------");

    let starts_prefix: Vec<String> = starts.iter().map(|x| x.to_string()).collect();
    println!("starts back to string: {starts_prefix:?}");

    println!("-----------------------------");

    let zipped = zip(starts, ends).map(|(x, y)| (x.to_string(), y.to_string()));

    for (start, end) in zipped {
        println!("(start: {start:?}, end: {end:?})");
        let prefix = extract_prefix(&start);
        println!("\thalf start: {prefix:?}");

        let prefix_repeated = prefix.repeat(2);
        println!("\thalf start repeated: {prefix_repeated:?}");
    }

    Ok(())
}

fn extract_prefix(s: &String) -> String {
    let n = s.chars().count();
    let npre = match n % 2 {
        1 => n / 2 + 1,
        _ => n / 2,
    };
    let nbytes = char_offset(s, npre);
    let pre: String = String::from(&s[0..=nbytes]);

    pre
}

fn char_offset(s: &String, nchars: usize) -> usize {
    s.char_indices()
        .nth(nchars - 1)
        .map(|(i, _)| i)
        .unwrap_or(0)
}

fn parse_input(input_str: String) -> Result<(Vec<i64>, Vec<i64>), MainError> {
    input_str
        .trim()
        .split(",")
        .map(|s| {
            s.split_once("-")
                .ok_or_else(|| MainError::SplitError(s.into()))
        })
        .map(|ss| {
            let (s1, s2) = ss?;
            let x1 = s1.parse::<i64>().or_else(|err| {
                println!("Failed to parse {s1:?} as i64");
                Err(MainError::ParseError(err))
            })?;
            let x2 = s2.parse::<i64>().or_else(|err| {
                println!("Failed to parse {s2:?} as i64");
                Err(MainError::ParseError(err))
            })?;
            Ok((x1, x2))
        })
        .collect::<Result<(Vec<i64>, Vec<i64>), MainError>>()
}
