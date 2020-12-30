use aoc_lib::TracingAlloc;
use color_eyre::eyre::Result;

use std::{collections::HashMap, num::ParseIntError};

#[global_allocator]
static ALLOC: TracingAlloc = TracingAlloc::new();

fn part1(adaptors: &[u64]) -> Result<u64> {
    let [_, ones, _, threes] =
        adaptors
            .windows(2)
            .map(|pair| pair[1] - pair[0])
            .fold([1; 4], |mut counts, it| {
                counts[it as usize] += 1;
                counts
            });
    Ok(ones * threes)
}

fn part2_search(adaptors: &[u64], db: &mut HashMap<u64, u64>) -> u64 {
    match adaptors.split_first() {
        Some((_, [])) => 1,
        Some((first, rest)) => rest
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
        None => 0, // Shouldn't get an empty list, but just in case...
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

    let (mut adaptors, parse_bench) = aoc_lib::bench::<_, ParseIntError>(&ALLOC, "Parse", &|| {
        let res: Vec<_> = input
            .lines()
            .map(str::trim)
            .map(str::parse)
            .collect::<Result<_, _>>()?;
        Ok(res)
    })?;

    adaptors.sort();

    let (p1_res, p1_bench) = aoc_lib::bench(&ALLOC, "Part 1", &|| part1(&adaptors))?;
    let (p2_res, p2_bench) = aoc_lib::bench(&ALLOC, "Part 2", &|| part2(&adaptors))?;

    aoc_lib::display_results(
        "Day 10: Adapter Array",
        &[(&"", parse_bench), (&p1_res, p1_bench), (&p2_res, p2_bench)],
    );

    Ok(())
}

#[cfg(test)]
mod tests_2010 {
    use aoc_lib::Example;

    use super::*;

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
