use aoc_lib::TracingAlloc;
use color_eyre::eyre::{eyre, Result};
use std::num::ParseIntError;

#[global_allocator]
static ALLOC: TracingAlloc = TracingAlloc::new();

fn part1(input: &[u64], preamble_len: usize) -> Result<(usize, u64)> {
    'outer: for (idx, window) in input.windows(preamble_len + 1).enumerate() {
        let (&last, preamble) = window.split_last().unwrap();

        for (idx, lhs) in preamble.iter().enumerate() {
            for rhs in preamble[idx..].iter().filter(|&rhs| rhs != lhs) {
                if last == lhs + rhs {
                    continue 'outer;
                }
            }
        }

        return Ok((idx + preamble_len, last));
    }

    Err(eyre!("No invalid number found"))
}

fn part2(input: &[u64], preamble_len: usize) -> Result<u64> {
    let (idx, res) = part1(input, preamble_len)?;

    // No need to search beyond the index of the found value.
    let prefix = &input[..idx];

    for window_len in 2..=idx {
        for window in prefix.windows(window_len) {
            if res == window.iter().sum() {
                let (min, max) = window
                    .iter()
                    .fold((u64::MAX, 0), |(min, max), &i| (min.min(i), max.max(i)));

                return Ok(min + max);
            }
        }
    }

    Err(eyre!("No sequence found"))
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let input = aoc_lib::input(2020, 9).open()?;
    let (sequence, parse_bench) = aoc_lib::bench::<_, ParseIntError>(&ALLOC, "Parse", &|| {
        let res: Vec<_> = input
            .lines()
            .map(str::trim)
            .map(str::parse)
            .collect::<Result<_, _>>()?;
        Ok(res)
    })?;
    let (p1_res, p1_bench) =
        aoc_lib::bench(&ALLOC, "Part 1", &|| part1(&sequence, 25).map(|(_, r)| r))?;
    let (p2_res, p2_bench) = aoc_lib::bench(&ALLOC, "Part 2", &|| part2(&sequence, 25))?;

    aoc_lib::display_results(
        "Day 9: Encoding Error",
        &[(&"", parse_bench), (&p1_res, p1_bench), (&p2_res, p2_bench)],
    );

    Ok(())
}

#[cfg(test)]
mod tests_2009 {
    use aoc_lib::Example;

    use super::*;

    #[test]
    fn part1_example() {
        let input = aoc_lib::input(2020, 9)
            .example(Example::Part1, 1)
            .open()
            .unwrap();

        let parsed: Vec<_> = input
            .lines()
            .map(str::trim)
            .map(str::parse)
            .collect::<Result<_, ParseIntError>>()
            .unwrap();

        let expected = 127;
        let (_, actual) = part1(&parsed, 5).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn part2_example() {
        let input = aoc_lib::input(2020, 9)
            .example(Example::Part1, 1)
            .open()
            .unwrap();

        let parsed: Vec<_> = input
            .lines()
            .map(str::trim)
            .map(str::parse)
            .collect::<Result<_, ParseIntError>>()
            .unwrap();

        let expected = 62;
        let actual = part2(&parsed, 5).unwrap();
        assert_eq!(expected, actual);
    }
}
