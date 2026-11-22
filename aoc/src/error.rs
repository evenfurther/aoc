use std::string::FromUtf8Error;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("IO error while loading input")]
    Io(#[from] std::io::Error),
    #[error("malformed UTF8 string in input")]
    Utf8(#[from] FromUtf8Error),
    #[error(transparent)]
    Other(#[from] eyre::Report),
    #[error("no output for day {day} part {part}{}", variant.map_or_default(|v| format!(r#" (variant "{v}")"#)))]
    NoOutput {
        day: usize,
        part: usize,
        variant: Option<&'static str>,
    },
}

impl Error {
    pub fn from_error<E>(e: E) -> Self
    where
        E: Into<eyre::Report>,
    {
        Error::Other(e.into())
    }
}

#[cfg(test)]
mod tests {
    use super::Error;

    #[test]
    fn no_output_message() {
        let e = Error::NoOutput {
            day: 3,
            part: 2,
            variant: Some("fast"),
        };
        assert_eq!(
            e.to_string(),
            r#"no output for day 3 part 2 (variant "fast")"#
        );
        let e = Error::NoOutput {
            day: 3,
            part: 2,
            variant: None,
        };
        assert_eq!(e.to_string(), "no output for day 3 part 2");
    }
}
