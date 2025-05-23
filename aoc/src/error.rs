use std::string::FromUtf8Error;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("IO error while loading input")]
    Io(#[from] std::io::Error),
    #[error("malformed UTF8 string in input")]
    Utf8(#[from] FromUtf8Error),
    #[error(transparent)]
    Other(#[from] eyre::Report),
    #[error("no output for day {day} part {part}{}", variant.map(|v| format!(r#" (variant "{v}""#)).unwrap_or_default())]
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
