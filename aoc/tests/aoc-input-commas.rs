use std::str::FromStr;

use aoc_derive::aoc;

#[aoc(day1, part1)]
fn day1_part1() -> usize {
    30
}

#[aoc(day1, part2)]
fn day1_part2() -> eyre::Result<usize> {
    Ok(40)
}

#[aoc(day1, part2, alternate)]
fn day1_part2_alternate() -> eyre::Result<usize> {
    Ok(50)
}

#[test]
fn result() {
    assert_eq!(30, day1_part1());
    assert_eq!(40, day1_part2().unwrap());
    assert_eq!(50, day1_part2_alternate().unwrap());
}

#[test]
fn runner() {
    assert_eq!(30, runner_1_1_none().unwrap());
    assert_eq!(40, runner_1_2_none().unwrap());
    assert_eq!(50, runner_1_2_alternate().unwrap());
}

#[aoc(day3, part1, str)]
fn d3p1s(input: &str) -> String {
    input.replace('\n', "")
}

#[aoc(day3, part1, vec_str)]
fn d3p1svec(mut input: Vec<&str>) -> String {
    input.sort_unstable();
    format!("{input:?}")
}

#[aoc(day3, part1, ref_str)]
fn d3p1sref(input: &[&str]) -> String {
    format!("{input:?}")
}

#[aoc(day3, part1, mut_str)]
fn d3p1smut(input: &mut [&str]) -> String {
    input.sort_unstable();
    format!("{input:?}")
}

#[aoc(day3, part1, bytes)]
fn d3p1b(input: &[u8]) -> eyre::Result<String> {
    Ok(d3p1s(std::str::from_utf8(input)?))
}

#[aoc(day3, part1, u32)]
fn d3p1u32(input: &[u32]) -> u32 {
    input.iter().sum()
}

#[aoc(day3, part1, u32_commas, separator = ',')]
fn d3p1u32_commas(input: &[u32]) -> u32 {
    input.iter().sum()
}

#[aoc(day3, part1, s)]
fn d1p1ss(input: &[S]) -> u8 {
    input.iter().map(|s| s.u).sum()
}

#[aoc(day3, part1, s_vec)]
fn d1p1ss_vec(input: Vec<S>) -> u8 {
    input.into_iter().map(|s| s.u).sum()
}

#[aoc(day3, part1, s_mut)]
fn d1p1ss_mut(input: &mut [S]) -> u8 {
    input.iter_mut().map(|s| s.u).sum()
}

#[aoc(day3, part1, separator = ',', s_commas)]
fn d1p1ss_commas(input: &[S]) -> u8 {
    input.iter().map(|s| s.u).sum()
}

struct S {
    u: u8,
}

impl FromStr for S {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(S {
            u: s.as_bytes()[0] - b'0',
        })
    }
}

#[test]
fn inputs_commas() {
    aoc::input::OVERRIDE_INPUT
        .set(String::from("tests/input-commas.txt"))
        .unwrap();
    assert_eq!(66, runner_3_1_u32_commas().unwrap());
    assert_eq!(12, runner_3_1_s_commas().unwrap());
}
