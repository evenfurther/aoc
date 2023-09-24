#![deny(clippy::pedantic)]
#![allow(clippy::missing_errors_doc, clippy::missing_panics_doc)]

pub use aoc_derive::*;

pub mod error;
pub mod input;
mod run;
mod runners;
pub mod test;

pub use run::run;
pub use runners::register_runner;
