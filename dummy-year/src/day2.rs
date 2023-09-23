use eyre::Result;

fn generator(input: &str) -> Result<Vec<Vec<u32>>> {
    Ok(input
        .lines()
        .map(|l| {
            l.split('x')
                .map(str::parse)
                .collect::<Result<Vec<u32>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()?)
}

#[aoc(day2, part1)]
fn part1(input: &str) -> u32 {
    generator(input)
        .unwrap()
        .iter()
        .map(|v| {
            let mut v = [v[0] * v[1], v[0] * v[2], v[1] * v[2]];
            v.sort_unstable();
            v[0] * 3 + v[1] * 2 + v[2] * 2
        })
        .sum()
}

#[aoc(day2, part2)]
fn part2(input: &str) -> u32 {
    generator(input)
        .unwrap()
        .iter()
        .map(|v| {
            let mut v = v.clone();
            v.sort_unstable();
            (v[0] + v[1]) * 2 + v[0] * v[1] * v[2]
        })
        .sum()
}
