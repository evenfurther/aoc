use anyhow::Result;
use std::str::FromStr;

pub static mut OVERRIDE_INPUT: Option<String> = None;

pub fn input_bytes(day: usize) -> Result<Vec<u8>> {
    match { unsafe { &OVERRIDE_INPUT } } {
        Some(s) => Ok(std::fs::read(s).unwrap_or_else(|_| {
            let mut s = s.as_bytes().to_vec();
            s.push(b'\n');
            s
        })),
        None => Ok(std::fs::read(format!("input/day{}.txt", day))?),
    }
}

pub fn input_string(day: usize) -> Result<String> {
    Ok(String::from_utf8(input_bytes(day)?)?)
}

/// Parse input as lines() if `sep` is absent, or as a single line
/// if `sep` is present.
pub fn parse_input<T>(input: &str, sep: Option<char>) -> Result<Vec<T>>
where
    T: FromStr,
    <T as FromStr>::Err: std::error::Error + Send + Sync + 'static,
{
    match sep {
        Some(sep) => Ok(input
            .trim()
            .split(sep)
            .map(T::from_str)
            .collect::<std::result::Result<Vec<T>, _>>()?),
        None => Ok(input
            .lines()
            .map(T::from_str)
            .collect::<std::result::Result<Vec<T>, _>>()?),
    }
}

pub fn parse_input_bytes(input: &[u8], sep: Option<u8>) -> Result<Vec<&[u8]>> {
    let sep = sep.unwrap_or(b'\n');
    let input = if input[input.len() - 1] == sep {
        &input[..input.len() - 1]
    } else {
        input
    };
    Ok(input.split(|&b| b == sep).collect())
}
