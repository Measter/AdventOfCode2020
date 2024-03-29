use std::collections::HashSet;

use aoc_lib::{Bench, BenchResult, Day};
use color_eyre::eyre::Result;

pub const DAY: Day = Day {
    day: 6,
    name: "Custom Customs",
    part_1: run_part1,
    part_2: Some(run_part2),
    other: &[],
};

fn run_part1(input: &str, b: Bench) -> BenchResult {
    b.bench(|| part1(input))
}

fn run_part2(input: &str, b: Bench) -> BenchResult {
    b.bench(|| part2(input))
}

fn part1(input: &str) -> Result<usize> {
    let groups = input.split("\n\n");

    let mut sum = 0;
    let mut buffer = HashSet::new();

    for group in groups {
        let answers = group.chars().filter(|c| c.is_alphabetic());
        buffer.extend(answers);

        sum += buffer.len();
        buffer.clear();
    }

    Ok(sum)
}

fn part2(input: &str) -> Result<usize> {
    let groups = input.split("\n\n");

    let mut sum = 0;
    let mut group_buffer = HashSet::new();
    let mut person_buffer = HashSet::new();

    for group in groups {
        let mut people = group.lines().map(str::trim);

        group_buffer.extend(people.next().unwrap().chars());

        for person in people {
            person_buffer.extend(person.chars());

            group_buffer.retain(|u| person_buffer.contains(u));

            person_buffer.clear()
        }

        sum += group_buffer.len();
        group_buffer.clear();
    }

    Ok(sum)
}

#[cfg(test)]
mod tests_2006 {
    use aoc_lib::Example;

    use super::*;

    #[test]
    fn part1_example() {
        let input = aoc_lib::input(6).example(Example::Part1, 1).open().unwrap();

        let expected = 11;
        let actual = part1(&input).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn part2_example() {
        let input = aoc_lib::input(6).example(Example::Part1, 1).open().unwrap();

        let expected = 6;
        let actual = part2(&input).unwrap();

        assert_eq!(actual, expected);
    }
}
