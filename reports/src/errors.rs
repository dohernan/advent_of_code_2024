use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum ReportsError {
    InvalidReportsLength(usize),
}

impl Display for ReportsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReportsError::InvalidReportsLength(length) => {
                write!(f, "Length of reports invalid: {length}")
            }
        }
    }
}

impl Error for ReportsError {}
