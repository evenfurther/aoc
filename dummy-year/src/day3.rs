#[aoc(day3, part1)]
fn part1(mut input: Vec<String>) -> usize {
    input.push(String::from("Hello, world"));
    input.iter_mut().for_each(|l| l.push('x'));
    input.into_iter().map(|s| s.len()).sum()
}
