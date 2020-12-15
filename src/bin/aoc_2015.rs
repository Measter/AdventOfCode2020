use std::num::ParseIntError;

use aoc_lib::TracingAlloc;
use color_eyre::eyre::Result;

#[global_allocator]
static ALLOC: TracingAlloc = TracingAlloc::new();

fn part1(numbers: &[u32], turns: u32) -> Result<u32> {
    let (last, rest) = numbers.split_last().unwrap();

    let mut last_seen = vec![u32::MAX; turns as usize];
    rest.iter()
        .copied()
        .zip(1u32..)
        .for_each(|(n, turn)| last_seen[n as usize] = turn);

    let mut cur_number = *last;

    let start = numbers.len() as u32;
    for i in start..turns {
        let next_number = i.saturating_sub(last_seen[cur_number as usize]);
        last_seen[cur_number as usize] = i;
        cur_number = next_number;
    }

    return Ok(cur_number);
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let input = aoc_lib::input(2020, 15).open()?;
    let numbers: Vec<_> = input
        .split(',')
        .map(str::parse)
        .collect::<Result<_, ParseIntError>>()?;

    aoc_lib::run(
        &ALLOC,
        "Day 15: Rambunctious Recitation",
        &*numbers,
        &|numbers| part1(numbers, 2020),
        &|numbers| part1(numbers, 30000000),
    )
}

#[cfg(test)]
mod tests_2015 {
    use super::*;
    use aoc_lib::parsers::split_pair;

    #[test]
    fn part1_example() {
        let input = aoc_lib::input(2020, 15).example(1, 1).open().unwrap();

        for line in input.lines().map(str::trim) {
            let (input, expected) = split_pair(line, ";").unwrap();
            let expected = expected.parse().unwrap();
            let input: Vec<_> = input
                .split(',')
                .map(str::parse)
                .collect::<Result<_, ParseIntError>>()
                .unwrap();

            let actual = part1(&input, 2020).unwrap();
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn part2_example() {
        let input = aoc_lib::input(2020, 15).example(2, 1).open().unwrap();

        for line in input.lines().map(str::trim) {
            let (input, expected) = split_pair(line, ";").unwrap();
            let expected = expected.parse().unwrap();
            let input: Vec<_> = input
                .split(',')
                .map(str::parse)
                .collect::<Result<_, ParseIntError>>()
                .unwrap();

            let actual = part1(&input, 30000000).unwrap();
            assert_eq!(actual, expected);
        }
    }
}
