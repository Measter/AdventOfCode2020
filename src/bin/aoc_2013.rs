use aoc_lib::{parsers::split_pair, TracingAlloc};
use color_eyre::eyre::{eyre, Result, WrapErr};

use std::num::ParseIntError;

#[global_allocator]
static ALLOC: TracingAlloc = TracingAlloc::new();

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

fn main() -> Result<()> {
    color_eyre::install()?;

    let input = aoc_lib::input(2020, 13).open()?;
    let (depart_time, busses) = parse_input(&input)?;

    aoc_lib::run(
        &ALLOC,
        "Day 13: Shuttle Search",
        (depart_time, &*busses),
        &|(depart_time, busses)| part1(depart_time, busses),
        &|(_, busses)| part2(busses),
    )
}

#[cfg(test)]
mod tests_2013 {
    use super::*;

    #[test]
    fn part1_example() {
        let input = aoc_lib::input(2020, 13).example(1, 1).open().unwrap();
        let (depart_time, busses) = parse_input(&input).unwrap();

        let expected = 295;
        let actual = part1(depart_time, &busses).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn part2_example() {
        let input = aoc_lib::input(2020, 13).example(2, 1).open().unwrap();
        let inputs = input.split("\n\n");
        let expecteds = [1068781, 3417, 754018, 779210, 1261476, 1202161486];

        for (id, (test, expected)) in inputs.zip(expecteds.iter()).enumerate() {
            let (_, busses) = parse_input(&test).unwrap();
            let actual = part2(&busses).unwrap();

            assert_eq!(*expected, actual, "{}", id);
        }
    }
}
