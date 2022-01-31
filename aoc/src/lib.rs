pub use aoc_derive::*;

pub mod input;
mod run;
mod runners;

pub use run::run;
pub use runners::register_runner;
