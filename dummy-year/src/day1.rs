use eyre::{bail, Result};

#[aoc(day1, part1)]
fn part1(input: &str) -> usize {
    2 * bytecount::count(input.as_bytes(), b'(') - input.trim().len()
}

#[aoc(day1, part1, str_slice)]
fn part1_string_slice(input: &[&str]) -> usize {
    input.iter().copied().map(part1).sum()
}

#[aoc(day1, part2)]
fn part2(input: &str) -> usize {
    part2_result(input.as_bytes()).unwrap()
}

#[aoc(day1, part2, result)]
fn part2_result(input: &[u8]) -> Result<usize> {
    let mut floor = 0;
    for (i, c) in input.iter().enumerate() {
        match c {
            b'(' => floor += 1,
            b')' if floor == 0 => return Ok(i + 1),
            b')' => floor -= 1,
            _ => bail!("should not be present"),
        }
    }
    bail!("no answer");
}

#[aoc(day1, part2, result_string)]
fn part2_result_string(input: &str) -> Result<String> {
    let mut floor = 0;
    for (i, c) in input.trim().bytes().enumerate() {
        match c {
            b'(' => floor += 1,
            _ if floor == 0 => return Ok(format!("{}", i + 1)),
            _ => floor -= 1,
        }
    }
    bail!("no answer");
}
