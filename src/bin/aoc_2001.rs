use std::num::ParseIntError;

use color_eyre::eyre::{eyre, Result};

fn part1(inputs: &[u32], target: u32) -> Option<u32> {
    for (i, lhs) in inputs.iter().enumerate() {
        for rhs in &inputs[i..] {
            if rhs + lhs == target {
                return Some(rhs * lhs);
            }
        }
    }

    None
}

fn part2(inputs: &[u32], target: u32) -> Option<u32> {
    for (i, lhs) in inputs.iter().enumerate() {
        for (i, mhs) in inputs[i..].iter().enumerate() {
            for rhs in &inputs[i..] {
                if rhs + mhs + lhs == target {
                    return Some(rhs * mhs * lhs);
                }
            }
        }
    }

    None
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let input = std::fs::read_to_string("inputs/aoc_2001.txt")?;
    let inputs: Vec<_> = input
        .lines()
        .map(str::trim)
        .map(str::parse)
        .collect::<Result<_, ParseIntError>>()
        .unwrap();

    let start = std::time::Instant::now();

    let part1 = part1(&inputs, 2020).ok_or_else(|| eyre!("Unable to find part 1 answer"))?;
    let part2 = part2(&inputs, 2020).ok_or_else(|| eyre!("Unable to find part 2 answer"))?;

    let elapsed = start.elapsed();

    println!("Part 1 output: {}", part1);
    println!("Part 2 output: {}", part2);

    println!("Elapsed: {}us", elapsed.as_micros());

    Ok(())
}

#[cfg(test)]
mod tests_1519 {
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

        assert_eq!(Some(514579), part1(&inputs, 2020));
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

        assert_eq!(Some(241861950), part2(&inputs, 2020));
    }
}
