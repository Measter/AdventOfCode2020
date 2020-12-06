use std::collections::HashSet;

use color_eyre::eyre::Result;

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

    let input = std::fs::read_to_string("inputs/aoc_2006.txt")?;

    aoc_lib::run("Day 6: Custom Customs", &*input, &part1, &part2)
}

#[cfg(test)]
mod tests_2006 {
    use super::*;

    #[test]
    fn part1_example() {
        let input = "abc

        a
        b
        c

        ab
        ac

        a
        a
        a
        a

        b";

        let expected = 11;
        let actual = part1(&input).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn part2_example() {
        let input = "abc

        a
        b
        c

        ab
        ac

        a
        a
        a
        a

        b";

        let expected = 6;
        let actual = part2(&input).unwrap();

        assert_eq!(actual, expected);
    }
}
