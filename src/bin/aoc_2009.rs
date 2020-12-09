use aoc_lib::TracingAlloc;
use color_eyre::eyre::{eyre, Result};
use std::num::ParseIntError;

#[global_allocator]
static ALLOC: TracingAlloc = TracingAlloc::new();

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

fn main() -> Result<()> {
    color_eyre::install()?;

    let input = std::fs::read_to_string("inputs/aoc_2009.txt")?;
    let sequence: Vec<_> = input
        .lines()
        .map(str::trim)
        .map(str::parse)
        .collect::<Result<_, ParseIntError>>()?;

    aoc_lib::run(
        &ALLOC,
        "Day 9: Encoding Error",
        &*sequence,
        &|input| part1(input, 25).map(|(_, r)| r),
        &|input| part2(input, 25),
    )
}

#[cfg(test)]
mod tests_2009 {
    use super::*;

    #[test]
    fn part1_example() {
        let input = "35
        20
        15
        25
        47
        40
        62
        55
        65
        95
        102
        117
        150
        182
        127
        219
        299
        277
        309
        576";

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
        let input = "35
        20
        15
        25
        47
        40
        62
        55
        65
        95
        102
        117
        150
        182
        127
        219
        299
        277
        309
        576";

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
