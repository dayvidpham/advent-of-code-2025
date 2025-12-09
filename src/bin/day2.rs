extern crate advent;
use crate::advent::prelude::*;
pub use std::io::prelude::*;

use std::{iter::zip, ops::Index, str::Chars};

const INPUT_PATH: &'static str = "./src/inputs/day2";

fn main() -> Result<(), MainError> {
    let mut input: String = Default::default();
    let mut data = File::open(INPUT_PATH)?;
    data.read_to_string(&mut input)?;

    println!("--- Read in as pairs of int ---");

    let bounds = parse_input(&input)?;
    bounds.iter().for_each(|xy| print!("{xy:?}    "));
    print!("\n");

    println!("--- Experiment: Re-parse back into string, stupid but oh well ---");

    let starts_prefix: Vec<String> = bounds.iter().map(|xy| xy.0.to_string()).collect();
    println!("starts: {starts_prefix:?}\n");

    println!("--- Extract prefix from starts, bound by ends ---");

    let zipped = zip(&starts_prefix, bounds.iter());
    //.collect::<Vec<(String, String)>>();

    let s_rep_len = &zipped
        .clone()
        .last()
        .map_or_else(|| 0, |(s_beg, (u_beg, u_end))| s_beg.len() + 1);

    let mut s_rep = String::with_capacity(*s_rep_len);
    let mut sum_invalid_ids: u64 = 0;

    // imperative style
    for (s_beg, (u_beg, u_end)) in zipped {
        println!("(start: {s_beg:?}, end: {u_end:?})");
        // stupid but ok
        let str_beg_prefix = extract_prefix(&s_beg);
        let mut u_prefix: u64 = str_beg_prefix.parse()?;
        if u_prefix < 100 {
            // even mor stupid
            u_prefix = 1;
        }

        loop {
            s_rep.clone_from(&u_prefix.to_string());
            println!("\tprefix: {s_rep:?}");

            s_rep.extend_from_within(..);
            let u_rep: u64 = s_rep.parse::<u64>()?;
            if u_rep > *u_end {
                break;
            } else if u_rep >= *u_beg {
                println!("\t\tinvalid id: {s_rep:?}");
                sum_invalid_ids += u_rep;
            }

            u_prefix += 1;
        }
    }

    println!("--- Experiment: Option and Iter interactions ---");

    {
        println!("Output of flat_map with identity fn");
        let xs: Vec<Option<u64>> = vec![Some(5), Some(6), None, Some(9)];
        xs.iter()
            .flat_map(|ox| ox)
            .for_each(|x| println!("Found an {x:?}"));
    }

    {
        println!("Output of flat_map() calling ox.iter().next()");
        let xs: Vec<Option<u64>> = vec![Some(5), Some(6), None, Some(9)];
        xs.iter()
            .flat_map(|ox| ox.iter().next())
            .for_each(|x| println!("Found an {x:?}"));
    }

    println!("--- Sum of All Invalid IDs ---");
    println!("{sum_invalid_ids:?}");
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

fn parse_input(input_str: &str) -> Result<(Vec<(u64, u64)>), MainError> {
    input_str
        .trim()
        .split(",")
        .map(|s| {
            s.split_once("-")
                .ok_or_else(|| MainError::SplitError(s.into()))
        })
        .map(|ss| {
            let (s1, s2) = ss?;
            let x1 = s1.parse::<u64>().or_else(|err| {
                println!("Failed to parse {s1:?} as u64");
                Err(MainError::ParseError(err))
            })?;
            let x2 = s2.parse::<u64>().or_else(|err| {
                println!("Failed to parse {s2:?} as u64");
                Err(MainError::ParseError(err))
            })?;
            Ok((x1, x2))
        })
        .collect::<Result<Vec<(u64, u64)>, MainError>>()
        .and_then(|mut bounds| {
            bounds.sort_by_key(|xy| xy.0);
            Ok(bounds)
        })
}
