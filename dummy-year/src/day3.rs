#[aoc(day3, part1)]
fn part1(mut input: Vec<String>) -> usize {
    input.push(String::from("Hello, world"));
    for l in &mut input {
        l.push('x');
    }
    input.into_iter().map(|s| s.len()).sum()
}
