use std::num::ParseIntError;

use aoc_lib::TracingAlloc;
use color_eyre::eyre::{eyre, Result};

#[global_allocator]
static ALLOC: TracingAlloc = TracingAlloc::new();

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

    let input = aoc_lib::input(2020, 1).open()?;
    let (inputs, parse_bench) = aoc_lib::bench(&ALLOC, "Parse", &|| {
        let res: Vec<_> = input
            .lines()
            .map(str::trim)
            .map(str::parse)
            .collect::<Result<_, ParseIntError>>()?;
        Ok(res)
    })?;

    let (p1_res, p1_bench) = aoc_lib::bench(&ALLOC, "Part 1", &|| part1(&inputs, 2020))?;
    let (p2_res, p2_bench) = aoc_lib::bench(&ALLOC, "Part 2", &|| part2(&inputs, 2020))?;

    aoc_lib::display_results(
        "Day 1: Report Repair",
        &[(&"", parse_bench), (&p1_res, p1_bench), (&p2_res, p2_bench)],
    )
}

#[cfg(test)]
mod tests_2001 {
    use std::num::ParseIntError;

    use aoc_lib::Example;

    use super::*;

    #[test]
    fn part1_example() {
        let input = aoc_lib::input(2020, 1)
            .example(Example::Part1, 1)
            .open()
            .unwrap();

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
        let input = aoc_lib::input(2020, 1)
            .example(Example::Part1, 1)
            .open()
            .unwrap();

        let inputs: Vec<_> = input
            .lines()
            .map(str::trim)
            .map(str::parse)
            .collect::<Result<_, ParseIntError>>()
            .unwrap();

        assert_eq!(241861950, part2(&inputs, 2020).unwrap());
    }
}
