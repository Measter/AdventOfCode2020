use std::collections::HashSet;

use aoc_lib::TracingAlloc;
use color_eyre::eyre::Result;

#[global_allocator]
static ALLOC: TracingAlloc = TracingAlloc::new();

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

fn main() -> Result<()> {
    color_eyre::install()?;

    let input = aoc_lib::input(2020, 6).open()?;
    let (p1_res, p1_bench) = aoc_lib::bench(&ALLOC, "Part 1", &|| part1(&input))?;
    let (p2_res, p2_bench) = aoc_lib::bench(&ALLOC, "Part 2", &|| part2(&input))?;

    aoc_lib::display_results(
        "Day 6: Custom Customs",
        &[(&p1_res, p1_bench), (&p2_res, p2_bench)],
    );

    Ok(())
}

#[cfg(test)]
mod tests_2006 {
    use aoc_lib::Example;

    use super::*;

    #[test]
    fn part1_example() {
        let input = aoc_lib::input(2020, 6)
            .example(Example::Part1, 1)
            .open()
            .unwrap();

        let expected = 11;
        let actual = part1(&input).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn part2_example() {
        let input = aoc_lib::input(2020, 6)
            .example(Example::Part1, 1)
            .open()
            .unwrap();

        let expected = 6;
        let actual = part2(&input).unwrap();

        assert_eq!(actual, expected);
    }
}
