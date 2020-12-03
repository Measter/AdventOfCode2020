use std::num::ParseIntError;

use advent_of_code_2020::run;
use color_eyre::eyre::{eyre, Result};

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

fn main() -> Result<()> {
    color_eyre::install()?;

    let input = std::fs::read_to_string("inputs/aoc_2001.txt")?;
    let inputs: Vec<_> = input
        .lines()
        .map(str::trim)
        .map(str::parse)
        .collect::<Result<_, ParseIntError>>()?;

    run(&inputs, &[&|i| part1(i, 2020), &|i| part2(i, 2020)])?;

    Ok(())
}

#[cfg(test)]
mod tests_2001 {
    use std::num::ParseIntError;

    use super::*;

    #[test]
    fn part1_example() {
        let input = "1721
        979
        366
        299
        675
        1456";

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
        let input = "1721
        979
        366
        299
        675
        1456";

        let inputs: Vec<_> = input
            .lines()
            .map(str::trim)
            .map(str::parse)
            .collect::<Result<_, ParseIntError>>()
            .unwrap();

        assert_eq!(241861950, part2(&inputs, 2020).unwrap());
    }
}
