pub use aoc_derive::*;

pub mod error;
pub mod input;
mod run;
mod runners;

pub use run::run;
pub use runners::register_runner;
