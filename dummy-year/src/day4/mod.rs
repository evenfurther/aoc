pub mod parse;

// The attribute is deliberately split over several lines, and this module
// lives in a subdirectory, to exercise solution discovery in `aoc-build`.
#[aoc(day4, part1)]
fn part1(input: &str) -> usize {
    parse::words(input).count()
}
