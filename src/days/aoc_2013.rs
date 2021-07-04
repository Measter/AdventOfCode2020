use aoc_lib::{day, parsers::split_pair, Bench, BenchResult, UserError};
use color_eyre::eyre::{eyre, Result, WrapErr};

use std::num::ParseIntError;

day! {
    day 13: "Shuttle Search"
    1: run_part1
    2: run_part2
}

fn run_part1(input: &str, b: Bench) -> BenchResult {
    let (depart_time, busses) = parse_input(input).map_err(UserError)?;

    b.bench(|| part1(depart_time, &busses))
}

fn run_part2(input: &str, b: Bench) -> BenchResult {
    let (_, busses) = parse_input(input).map_err(UserError)?;

    b.bench(|| part2(&busses))
}

#[derive(Debug, Copy, Clone)]
struct Bus {
    id: u64,
    number: u64,
}

fn parse_input(input: &str) -> Result<(u64, Vec<Bus>)> {
    let (depart_time, busses) = split_pair(input, "\n")?;
    let depart_time = depart_time
        .trim()
        .parse()
        .with_context(|| eyre!("Unable to parse departure time: {}", depart_time))?;

    let busses = busses
        .trim()
        .split(',')
        .map(str::trim)
        .zip(0..)
        .filter(|(b, _)| *b != "x")
        .map(|(bus, id)| bus.parse().map(|b| Bus { id, number: b }))
        .collect::<Result<_, ParseIntError>>()
        .with_context(|| eyre!("Unable to parse busses"))?;

    Ok((depart_time, busses))
}

fn part1(depart_time: u64, busses: &[Bus]) -> Result<u64> {
    let (bus, bus_depart) = busses
        .iter()
        .map(|&Bus { number, .. }| {
            let mult = match (depart_time / number, depart_time % number) {
                (a, 0) => a,
                (a, _) => a + 1,
            };

            (number, number * mult)
        })
        .min_by_key(|(_, b)| *b)
        .ok_or_else(|| eyre!("Unable to find bus"))?;

    Ok(bus * (bus_depart - depart_time))
}

fn part2(busses: &[Bus]) -> Result<u64> {
    let mut start = 1;
    let mut step = 1;

    let mut remaining_busses = busses;
    while let [next, rest @ ..] = remaining_busses {
        remaining_busses = rest;

        for i in (0..next.number).map(|n| n * step + start) {
            if (i + next.id) % next.number == 0 {
                start = i;
                step *= next.number;
                break;
            }
        }
    }

    Ok(start)
}

#[cfg(test)]
mod tests_2013 {
    use aoc_lib::Example;

    use super::*;

    #[test]
    fn part1_example() {
        let input = aoc_lib::input(2020, 13)
            .example(Example::Part1, 1)
            .open()
            .unwrap();
        let (depart_time, busses) = parse_input(&input).unwrap();

        let expected = 295;
        let actual = part1(depart_time, &busses).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn part2_example() {
        let input = aoc_lib::input(2020, 13)
            .example(Example::Part2, 1)
            .open()
            .unwrap();
        let inputs = input.split("\n\n");
        let expecteds = [1068781, 3417, 754018, 779210, 1261476, 1202161486];

        for (id, (test, expected)) in inputs.zip(expecteds.iter()).enumerate() {
            let (_, busses) = parse_input(&test).unwrap();
            let actual = part2(&busses).unwrap();

            assert_eq!(*expected, actual, "{}", id);
        }
    }
}
