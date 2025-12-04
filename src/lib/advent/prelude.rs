#![allow(unused)]

pub use std::error::Error;
pub use std::io::prelude::*;
pub use std::prelude::rust_2024::*;
pub use std::{fmt::Display, fs::File, num::ParseIntError};

#[derive(Debug)]
pub enum MainError {
    IOError(std::io::Error),
    ParseError(std::num::ParseIntError),
    SplitError(Box<str>),
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
            Self::IOError(err) => write!(f, "{err}, caused by {}", err.source().unwrap()),
            Self::ParseError(err) => write!(f, "{err}, caused by {}", err.source().unwrap()),
            Self::SplitError(err) => {
                write!(f, "Failed to split with delimiter '-' on: {}", err)
            }
        }
    }
}

impl Error for MainError {}
