pub fn words(input: &str) -> impl Iterator<Item = &str> {
    input.split_whitespace()
}

#[aoc(day4, part2)]
fn part2(input: &str) -> usize {
    words(input).map(str::len).max().unwrap_or(0)
}
