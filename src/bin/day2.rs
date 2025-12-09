extern crate advent;
use crate::advent::prelude::*;
pub use std::io::prelude::*;

use std::{iter::zip, str::Chars};

const INPUT_PATH: &'static str = "./src/inputs/day2";

fn main() -> Result<(), MainError> {
    let mut input: String = Default::default();
    let mut data = File::open(INPUT_PATH)?;
    data.read_to_string(&mut input)?;

    println!("--- Read in as pairs of int ---");

    let (starts, ends) = parse_input(&input)?;
    zip(&starts, &ends).for_each(|xy| print!("{xy:?}    "));
    print!("\n");

    println!("--- Experiment: Re-parse back into string, stupid but oh well ---");

    let starts_prefix: Vec<String> = starts.iter().map(|x| x.to_string()).collect();
    println!("starts: {starts_prefix:?}\n");

    println!("--- Extract prefix from starts, bound by ends ---");

    let zipped = zip(&starts, &ends).map(|(x, y)| (x.to_string(), y.to_string()));
    //.collect::<Vec<(String, String)>>();

    // imperative style
    let s_rep_len = &zipped
        .clone()
        .last()
        .map_or_else(|| 0, |(s_beg, s_end)| s_end.len());

    let s_rep = String::with_capacity(s_rep_len.clone());

    for (s_beg, s_end) in zipped {
        println!("(start: {s_beg:?}, end: {s_end:?})");
        let prefix = extract_prefix(&s_beg);
        println!("\thalf start: {prefix:?}");

        let prefix_repeated = prefix.repeat(2);
        println!("\thalf start repeated: {prefix_repeated:?}");
    }

    println!("--- Experiment: Option and Iter interactions ---");

    {
        println!("Output of flat_map with identity fn");
        let xs: Vec<Option<i64>> = vec![Some(5), Some(6), None, Some(9)];
        xs.iter()
            .flat_map(|ox| ox)
            .for_each(|x| println!("Found an {x:?}"));
    }

    {
        println!("Output of flat_map() calling ox.iter().next()");
        let xs: Vec<Option<i64>> = vec![Some(5), Some(6), None, Some(9)];
        xs.iter()
            .flat_map(|ox| ox.iter().next())
            .for_each(|x| println!("Found an {x:?}"));
    }

    Ok(())
}

fn extract_prefix(s: &str) -> &str {
    let n = s.chars().count();
    let npre = match n % 2 {
        1 => n / 2 + 1,
        _ => n / 2,
    };
    let nbytes = char_offset(s, npre);
    let pre = &s[0..=nbytes];
    pre
}

fn char_offset(s: &str, nchars: usize) -> usize {
    s.char_indices()
        .nth(nchars - 1)
        .map(|(i, _)| i)
        .unwrap_or(0)
}

fn parse_input(input_str: &str) -> Result<(Vec<i64>, Vec<i64>), MainError> {
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
