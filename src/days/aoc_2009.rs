use aoc_lib::{Bench, BenchResult, Day, UserError};
use color_eyre::eyre::{eyre, Result};

pub const DAY: Day = Day {
    day: 9,
    name: "Encoding Error",
    part_1: run_part1,
    part_2: Some(run_part2),
    other: &[],
};

fn run_part1(input: &str, b: Bench) -> BenchResult {
    let sequence: Vec<_> = input
        .lines()
        .map(str::trim)
        .map(str::parse)
        .collect::<Result<_, _>>()
        .map_err(UserError)?;

    b.bench(|| part1(&sequence, 25).map(|(_, r)| r))
}

fn run_part2(input: &str, b: Bench) -> BenchResult {
    let sequence: Vec<_> = input
        .lines()
        .map(str::trim)
        .map(str::parse)
        .collect::<Result<_, _>>()
        .map_err(UserError)?;

    b.bench(|| part2(&sequence, 25))
}

fn part1(input: &[u64], preamble_len: usize) -> Result<(usize, u64)> {
    'outer: for (idx, window) in input.windows(preamble_len + 1).enumerate() {
        let (&last, preamble) = window.split_last().unwrap();

        for (idx, lhs) in preamble.iter().enumerate() {
            for rhs in preamble[idx..].iter().filter(|&rhs| rhs != lhs) {
                if last == lhs + rhs {
                    continue 'outer;
                }
            }
        }

        return Ok((idx + preamble_len, last));
    }

    Err(eyre!("No invalid number found"))
}

fn part2(input: &[u64], preamble_len: usize) -> Result<u64> {
    let (idx, res) = part1(input, preamble_len)?;

    // No need to search beyond the index of the found value.
    let prefix = &input[..idx];

    for window_len in 2..=idx {
        for window in prefix.windows(window_len) {
            if res == window.iter().sum() {
                let (min, max) = window
                    .iter()
                    .fold((u64::MAX, 0), |(min, max), &i| (min.min(i), max.max(i)));

                return Ok(min + max);
            }
        }
    }

    Err(eyre!("No sequence found"))
}

#[cfg(test)]
mod tests_2009 {
    use super::*;
    use aoc_lib::Example;
    use std::num::ParseIntError;

    #[test]
    fn part1_example() {
        let input = aoc_lib::input(9).example(Example::Part1, 1).open().unwrap();

        let parsed: Vec<_> = input
            .lines()
            .map(str::trim)
            .map(str::parse)
            .collect::<Result<_, ParseIntError>>()
            .unwrap();

        let expected = 127;
        let (_, actual) = part1(&parsed, 5).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn part2_example() {
        let input = aoc_lib::input(9).example(Example::Part1, 1).open().unwrap();

        let parsed: Vec<_> = input
            .lines()
            .map(str::trim)
            .map(str::parse)
            .collect::<Result<_, ParseIntError>>()
            .unwrap();

        let expected = 62;
        let actual = part2(&parsed, 5).unwrap();
        assert_eq!(expected, actual);
    }
}
