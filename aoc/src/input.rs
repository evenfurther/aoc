use crate::error::Error;
use std::str::FromStr;

pub static mut OVERRIDE_INPUT: Option<String> = None;

pub fn input_bytes(day: usize) -> Result<Vec<u8>, Error> {
    match { unsafe { &OVERRIDE_INPUT } } {
        Some(s) => Ok(std::fs::read(s).unwrap_or_else(|_| {
            let mut s = s.as_bytes().to_vec();
            s.push(b'\n');
            s
        })),
        None => Ok(std::fs::read(format!("input/day{}.txt", day))?),
    }
}

pub fn input_string(day: usize) -> Result<String, Error> {
    Ok(String::from_utf8(input_bytes(day)?)?)
}

/// Parse input as lines() if `sep` is absent, or as a single line
/// if `sep` is present.
pub fn parse_input<T>(input: &str, sep: Option<&str>) -> Result<Vec<T>, Error>
where
    T: FromStr,
    <T as FromStr>::Err: Into<eyre::Report>,
{
    match sep {
        Some(sep) => Ok(input
            .trim()
            .split(sep)
            .map(|s| s.parse().map_err(Error::from_error))
            .collect::<Result<Vec<T>, Error>>()?),
        None => Ok(input
            .lines()
            .map(|s| s.parse().map_err(Error::from_error))
            .collect::<Result<Vec<T>, _>>()?),
    }
}

pub fn parse_input_bytes(input: &[u8], sep: Option<u8>) -> Result<Vec<&[u8]>, Error> {
    let sep = sep.unwrap_or(b'\n');
    Ok(input
        .strip_suffix(&[sep])
        .unwrap_or(input)
        .split(|&b| b == sep)
        .collect())
}
