use std::convert::TryInto;

use aoc_lib::{Bench, BenchResult, Day, UserError};
use color_eyre::eyre::Result;

pub const DAY: Day = Day {
    day: 15,
    name: "Rambunctious Recitation",
    part_1: run_part1,
    part_2: Some(run_part2),
    other: &[],
};

fn run_part1(input: &str, b: Bench) -> BenchResult {
    let numbers: Vec<_> = input
        .split(',')
        .map(str::parse)
        .collect::<Result<_, _>>()
        .map_err(UserError)?;

    b.bench(|| part1::<2020>(&numbers))
}

fn run_part2(input: &str, b: Bench) -> BenchResult {
    let numbers: Vec<_> = input
        .split(',')
        .map(str::parse)
        .collect::<Result<_, _>>()
        .map_err(UserError)?;

    b.bench(|| part1::<30000000>(&numbers))
}

fn part1<const TURNS: usize>(numbers: &[u32]) -> Result<u32> {
    assert!(numbers.iter().all(|&n| n < TURNS as u32));
    let (last, rest) = numbers.split_last().unwrap();

    let mut last_seen = vec![u32::MAX; TURNS as usize];
    let last_seen: &mut [u32; TURNS] = last_seen.as_mut_slice().try_into().unwrap();
    rest.iter()
        .copied()
        .zip(1u32..)
        .for_each(|(n, turn)| last_seen[n as usize] = turn);

    let start = numbers.len() as u32;
    let cur_number = (start..TURNS as u32).fold(*last, |cur_number, i| {
        // SAFETY: cur_number will always be < last_seen.len()
        let last_turn_seen = unsafe { last_seen.get_unchecked_mut(cur_number as usize) };
        i.saturating_sub(std::mem::replace(last_turn_seen, i))
    });

    Ok(cur_number)
}

#[cfg(test)]
mod tests_2015 {
    use super::*;
    use aoc_lib::{parsers::split_pair, Example};
    use std::num::ParseIntError;

    #[test]
    fn part1_example() {
        let input = aoc_lib::input(15)
            .example(Example::Part1, 1)
            .open()
            .unwrap();

        for line in input.lines().map(str::trim) {
            let (input, expected) = split_pair(line, ";").unwrap();
            let expected = expected.parse().unwrap();
            let input: Vec<_> = input
                .split(',')
                .map(str::parse)
                .collect::<Result<_, ParseIntError>>()
                .unwrap();

            let actual = part1::<2020>(&input).unwrap();
            assert_eq!(actual, expected);
        }
    }

    #[allow(unused)]
    // #[test]
    fn part2_example() {
        let input = aoc_lib::input(15)
            .example(Example::Part2, 1)
            .open()
            .unwrap();

        for line in input.lines().map(str::trim) {
            let (input, expected) = split_pair(line, ";").unwrap();
            let expected = expected.parse().unwrap();
            let input: Vec<_> = input
                .split(',')
                .map(str::parse)
                .collect::<Result<_, ParseIntError>>()
                .unwrap();

            let actual = part1::<30000000>(&input).unwrap();
            assert_eq!(actual, expected);
        }
    }
}
