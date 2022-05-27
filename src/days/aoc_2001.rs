use aoc_lib::{Bench, BenchResult, Day, UserError};
use color_eyre::eyre::{eyre, Result};

pub const DAY: Day = Day {
    day: 1,
    name: "Report Repair",
    part_1: run_part1,
    part_2: Some(run_part2),
    other: &[],
};

fn run_part1(input: &str, b: Bench) -> BenchResult {
    let inputs: Vec<_> = input
        .lines()
        .map(str::trim)
        .map(str::parse::<u32>)
        .collect::<Result<_, _>>()
        .map_err(UserError)?;

    b.bench(|| part1(&inputs, 2020))
}

fn run_part2(input: &str, b: Bench) -> BenchResult {
    let inputs: Vec<_> = input
        .lines()
        .map(str::trim)
        .map(str::parse::<u32>)
        .collect::<Result<_, _>>()
        .map_err(UserError)?;

    b.bench(|| part2(&inputs, 2020))
}

fn part1(inputs: &[u32], target: u32) -> Result<u32> {
    for (i, lhs) in inputs.iter().enumerate() {
        for rhs in &inputs[i..] {
            if rhs + lhs == target {
                return Ok(rhs * lhs);
            }
        }
    }

    Err(eyre!("Unable to find result"))
}

fn part2(inputs: &[u32], target: u32) -> Result<u32> {
    for (i, lhs) in inputs.iter().enumerate() {
        for (i, mhs) in inputs[i..].iter().enumerate() {
            for rhs in &inputs[i..] {
                if rhs + mhs + lhs == target {
                    return Ok(rhs * mhs * lhs);
                }
            }
        }
    }

    Err(eyre!("Unable to find result"))
}

#[cfg(test)]
mod tests_2001 {
    use std::num::ParseIntError;

    use aoc_lib::Example;

    use super::*;

    #[test]
    fn part1_example() {
        let input = aoc_lib::input(1).example(Example::Part1, 1).open().unwrap();

        let inputs: Vec<_> = input
            .lines()
            .map(str::trim)
            .map(str::parse)
            .collect::<Result<_, ParseIntError>>()
            .unwrap();

        assert_eq!(514579, part1(&inputs, 2020).unwrap());
    }

    #[test]
    fn part2_example() {
        let input = aoc_lib::input(1).example(Example::Part1, 1).open().unwrap();

        let inputs: Vec<_> = input
            .lines()
            .map(str::trim)
            .map(str::parse)
            .collect::<Result<_, ParseIntError>>()
            .unwrap();

        assert_eq!(241861950, part2(&inputs, 2020).unwrap());
    }
}
