use aoc_lib::{parsers::unsigned_number, Bench, BenchResult, Day, ParseResult, UserError};
use color_eyre::{
    eyre::{eyre, Result},
    Report,
};
use nom::{
    bytes::complete::{tag, take_till1, take_while1},
    sequence::tuple,
};

pub const DAY: Day = Day {
    day: 2,
    name: "Passward Philosophy",
    part_1: run_part1,
    part_2: Some(run_part2),
    other: &[("Parse", run_parse)],
};

fn run_part1(input: &str, b: Bench) -> BenchResult {
    let inputs: Vec<_> = input
        .lines()
        .map(str::trim)
        .map(Password::parse)
        .collect::<Result<_, _>>()
        .map_err(UserError)?;

    b.bench(|| part1(&inputs))
}

fn run_part2(input: &str, b: Bench) -> BenchResult {
    let inputs: Vec<_> = input
        .lines()
        .map(str::trim)
        .map(Password::parse)
        .collect::<Result<_, _>>()
        .map_err(UserError)?;

    b.bench(|| part2(&inputs))
}

fn run_parse(input: &str, b: Bench) -> BenchResult {
    b.bench(|| {
        let data: Vec<_> = input
            .lines()
            .map(str::trim)
            .map(Password::parse)
            .collect::<Result<_, _>>()?;
        Ok::<_, Report>(ParseResult(data))
    })
}

#[derive(Debug, PartialEq)]
struct Password<'a> {
    min: usize,
    max: usize,
    check: &'a str,
    pswd: &'a str,
}

impl<'a> Password<'a> {
    fn parse(input: &'a str) -> Result<Password<'a>> {
        let (_, (min, _, max, _, check, _, pswd)) = tuple((
            unsigned_number::<usize>,
            tag("-"),
            unsigned_number::<usize>,
            tag(" "),
            take_till1(|c: char| c == ':'),
            tag(": "),
            take_while1(|_| true),
        ))(input)
        .map_err(|e| eyre!("Error while parsing: {}", e))?;

        Ok(Password {
            min: min?,
            max: max?,
            check,
            pswd,
        })
    }

    fn part1_is_valid(&self) -> bool {
        (self.min..=self.max).contains(&self.pswd.matches(self.check).count())
    }

    fn part2_is_valid(&self) -> bool {
        let first = self.pswd[self.min - 1..].starts_with(self.check);
        let second = self.pswd[self.max - 1..].starts_with(self.check);

        first ^ second
    }
}

fn part1(inputs: &[Password]) -> Result<usize> {
    Ok(inputs.iter().filter(|p| p.part1_is_valid()).count())
}

fn part2(inputs: &[Password]) -> Result<usize> {
    Ok(inputs.iter().filter(|p| p.part2_is_valid()).count())
}

#[cfg(test)]
mod tests_2002 {
    use aoc_lib::Example;

    use super::*;

    #[test]
    fn parse_test() {
        let input = aoc_lib::input(2).example(Example::Part1, 1).open().unwrap();

        let expected = vec![
            Password {
                min: 1,
                max: 3,
                check: "a",
                pswd: "abcde",
            },
            Password {
                min: 1,
                max: 3,
                check: "b",
                pswd: "cdefg",
            },
            Password {
                min: 2,
                max: 9,
                check: "c",
                pswd: "ccccccccc",
            },
        ];

        let actual: Vec<_> = input
            .lines()
            .map(str::trim)
            .map(Password::parse)
            .collect::<Result<_>>()
            .unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn part1_test() {
        let tests = [
            (
                Password {
                    min: 1,
                    max: 3,
                    check: "a",
                    pswd: "abcde",
                },
                true,
            ),
            (
                Password {
                    min: 1,
                    max: 3,
                    check: "b",
                    pswd: "cdefg",
                },
                false,
            ),
            (
                Password {
                    min: 2,
                    max: 9,
                    check: "c",
                    pswd: "ccccccccc",
                },
                true,
            ),
        ];

        for (pswd, expected) in &tests {
            assert_eq!(pswd.part1_is_valid(), *expected);
        }
    }

    #[test]
    fn part2_test() {
        let tests = [
            (
                Password {
                    min: 1,
                    max: 3,
                    check: "a",
                    pswd: "abcde",
                },
                true,
            ),
            (
                Password {
                    min: 1,
                    max: 3,
                    check: "b",
                    pswd: "cdefg",
                },
                false,
            ),
            (
                Password {
                    min: 2,
                    max: 9,
                    check: "c",
                    pswd: "ccccccccc",
                },
                false,
            ),
        ];

        for (pswd, expected) in &tests {
            assert_eq!(pswd.part2_is_valid(), *expected);
        }
    }
}
