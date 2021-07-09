use aoc_lib::{day, misc::ArrWindows, Bench, BenchResult, UserError};
use color_eyre::eyre::Result;

use std::collections::HashMap;

day! {
    day 10: "Adapter Array"
    1: run_part1
    2: run_part2
}

fn run_part1(input: &str, b: Bench) -> BenchResult {
    let mut adaptors: Vec<_> = input
        .lines()
        .map(str::trim)
        .map(str::parse)
        .collect::<Result<_, _>>()
        .map_err(UserError)?;

    adaptors.sort_unstable();

    b.bench(|| part1(&adaptors))
}

fn run_part2(input: &str, b: Bench) -> BenchResult {
    let mut adaptors: Vec<_> = input
        .lines()
        .map(str::trim)
        .map(str::parse)
        .collect::<Result<_, _>>()
        .map_err(UserError)?;

    adaptors.sort_unstable();

    b.bench(|| part2(&adaptors))
}

fn part1(adaptors: &[u64]) -> Result<u64> {
    let [_, ones, _, threes] =
        ArrWindows::new(adaptors)
            .map(|[a, b]| b - a)
            .fold([1; 4], |mut counts, it| {
                counts[it as usize] += 1;
                counts
            });
    Ok(ones * threes)
}

fn part2_search(adaptors: &[u64], db: &mut HashMap<u64, u64>) -> u64 {
    match adaptors {
        [] => 0, // Shouldn't get an empty list, but just in case...
        [_] => 1,
        [first, rest @ ..] => rest
            .iter()
            .take_while(|a| *a - first <= 3)
            .enumerate()
            .map(|(idx, val)| {
                db.get(val).copied().unwrap_or_else(|| {
                    let sub_count = part2_search(&rest[idx..], db);
                    *db.entry(*val).or_insert(sub_count)
                })
            })
            .sum(),
    }
}

fn part2(adaptors: &[u64]) -> Result<u64> {
    let mut new_adaptors = vec![0];
    new_adaptors.extend_from_slice(adaptors);
    new_adaptors.sort_unstable();

    let mut db = HashMap::new();
    Ok(part2_search(&new_adaptors, &mut db))
}

#[cfg(test)]
mod tests_2010 {
    use super::*;
    use aoc_lib::Example;
    use std::num::ParseIntError;

    #[test]
    fn part1_example1() {
        let input = aoc_lib::input(2020, 10)
            .example(Example::Part1, 1)
            .open()
            .unwrap();

        let mut adaptors: Vec<u64> = input
            .lines()
            .map(str::trim)
            .map(str::parse)
            .collect::<Result<_, ParseIntError>>()
            .unwrap();

        adaptors.sort();

        let expected = 35;
        let actual = part1(&adaptors).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn part1_example2() {
        let input = aoc_lib::input(2020, 10)
            .example(Example::Part1, 2)
            .open()
            .unwrap();

        let mut adaptors: Vec<u64> = input
            .lines()
            .map(str::trim)
            .map(str::parse)
            .collect::<Result<_, ParseIntError>>()
            .unwrap();

        adaptors.sort();

        let expected = 220;
        let actual = part1(&adaptors).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn part2_example1() {
        let input = aoc_lib::input(2020, 10)
            .example(Example::Part1, 1)
            .open()
            .unwrap();

        let mut adaptors: Vec<u64> = input
            .lines()
            .map(str::trim)
            .map(str::parse)
            .collect::<Result<_, ParseIntError>>()
            .unwrap();

        adaptors.sort();

        let expected = 8;
        let actual = part2(&adaptors).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn part2_example2() {
        let input = aoc_lib::input(2020, 10)
            .example(Example::Part1, 2)
            .open()
            .unwrap();

        let mut adaptors: Vec<u64> = input
            .lines()
            .map(str::trim)
            .map(str::parse)
            .collect::<Result<_, ParseIntError>>()
            .unwrap();

        adaptors.sort();

        let expected = 19208;
        let actual = part2(&adaptors).unwrap();

        assert_eq!(actual, expected);
    }
}
