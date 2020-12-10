use aoc_lib::TracingAlloc;
use color_eyre::eyre::Result;

use std::collections::HashMap;
use std::num::ParseIntError;

#[global_allocator]
static ALLOC: TracingAlloc = TracingAlloc::new();

fn part1(adaptors: &[u64]) -> Result<u64> {
    let mut diffs = HashMap::<u64, u64>::new();

    for diff in adaptors.windows(2).map(|pair| pair[1] - pair[0]) {
        *diffs.entry(diff).or_default() += 1
    }

    Ok((diffs.get(&1).unwrap_or(&0) + 1) * (diffs.get(&3).unwrap_or(&0) + 1))
}

fn part2_search(adaptors: &[u64], db: &mut HashMap<u64, u64>) -> u64 {
    let (first, rest) = if let Some(v) = adaptors.split_first() {
        v
    } else {
        return 0;
    };

    if rest.is_empty() {
        1
    } else {
        let nexts = rest.iter().take_while(|a| *a - first <= 3).enumerate();

        let mut count = 0;

        for (idx, val) in nexts {
            if let Some(sub_count) = db.get(val) {
                count += sub_count;
            } else {
                let sub_count = part2_search(&rest[idx..], db);
                db.insert(*val, sub_count);
                count += sub_count;
            }
        }

        count
    }
}

fn part2(adaptors: &[u64]) -> Result<u64> {
    let mut new_adaptors = vec![0];
    new_adaptors.extend_from_slice(adaptors);
    new_adaptors.sort();

    let mut db = HashMap::new();
    Ok(part2_search(&new_adaptors, &mut db))
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let input = aoc_lib::input(2020, 10).open()?;

    let mut adaptors: Vec<u64> = input
        .lines()
        .map(str::trim)
        .map(str::parse)
        .collect::<Result<_, ParseIntError>>()?;

    adaptors.sort();

    aoc_lib::run(
        &ALLOC,
        "Day 10: Adapter Array",
        &*adaptors,
        &|adaptors| part1(adaptors),
        &|adaptors| part2(adaptors),
    )
}

#[cfg(test)]
mod tests_2010 {
    use super::*;

    #[test]
    fn part1_example1() {
        let input = aoc_lib::input(2020, 10).example(1, 1).open().unwrap();

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
        let input = aoc_lib::input(2020, 10).example(1, 2).open().unwrap();

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
        let input = aoc_lib::input(2020, 10).example(1, 1).open().unwrap();

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
        let input = aoc_lib::input(2020, 10).example(1, 2).open().unwrap();

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
