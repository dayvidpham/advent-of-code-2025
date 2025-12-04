extern crate advent;
use crate::advent::prelude::*;
pub use std::io::prelude::*;

const INPUT_PATH: &'static str = "./src/inputs/day2";

fn main() -> Result<(), MainError> {
    let mut input: String = Default::default();
    let mut data = File::open(INPUT_PATH)?;
    data.read_to_string(&mut input)?;
    let (starts, ends) = parse_input(input)?;

    Ok(())
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

