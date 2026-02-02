#![warn(clippy::pedantic)]
#![allow(
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::module_name_repetitions,
    clippy::similar_names
)]

#[macro_use]
extern crate aoc;

pub mod register {
    include!(concat!(env!("OUT_DIR"), "/register.rs"));
}

pub mod day1;
pub mod day2;
pub mod day3;
