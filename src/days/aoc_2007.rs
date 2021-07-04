use std::collections::{HashMap, HashSet};

use aoc_lib::{day, parsers::unsigned_number, Bench, BenchResult, UserError};
use color_eyre::eyre::{eyre, Result};
use nom::{
    bytes::complete::{tag, take_until},
    error::ErrorKind,
    sequence::tuple,
};

day! {
    day 7: "Handy Haversacks"
    1: run_part1
    2: run_part2
}

fn run_part1(input: &str, b: Bench) -> BenchResult {
    let rules = parse_bags(input).map_err(UserError)?;
    b.bench(|| part1(&rules, "shiny gold"))
}

fn run_part2(input: &str, b: Bench) -> BenchResult {
    let rules = parse_bags(input).map_err(UserError)?;
    b.bench(|| part2(&rules, "shiny gold"))
}

fn parse_bags(input: &str) -> Result<HashMap<&str, HashMap<&str, usize>>> {
    let mut bags = HashMap::new();

    for line in input.lines().map(str::trim) {
        let (contained, (this_bag, _)) =
            tuple::<_, _, (&str, ErrorKind), _>((take_until(" bags"), tag(" bags contain ")))(line)
                .map_err(|e| eyre!("Parse Error: {}", e))?;

        let bag_entry = bags.entry(this_bag).or_insert_with(HashMap::new);

        if contained == "no other bags." {
            continue;
        }

        for sub_bag in contained.split_terminator(&[',', '.'][..]).map(str::trim) {
            let (_, (count, sub_bag)) = tuple((unsigned_number, take_until("bag")))(sub_bag)
                .map_err(|e| eyre!("Parse Error: {}", e))?;

            bag_entry.insert(sub_bag.trim(), count?);
        }
    }

    Ok(bags)
}

fn recursive_search<'a>(
    bag_rules: &HashMap<&'a str, HashMap<&'a str, usize>>,
    bag: &str,
    seen: &mut HashSet<&'a str>,
) {
    for (cur_bag, contains) in bag_rules {
        if contains.contains_key(bag) {
            seen.insert(*cur_bag);

            recursive_search(bag_rules, cur_bag, seen)
        }
    }
}

fn part1(bag_rules: &HashMap<&str, HashMap<&str, usize>>, bag: &str) -> Result<usize> {
    let mut known_bags = HashSet::new();

    recursive_search(bag_rules, bag, &mut known_bags);

    Ok(known_bags.len())
}

fn recursive_count(bag_rules: &HashMap<&str, HashMap<&str, usize>>, bag: &str) -> usize {
    let mut count = 1;

    for (sub_bag, sub_count) in &bag_rules[bag] {
        count += sub_count * recursive_count(bag_rules, sub_bag);
    }

    count
}

fn part2(bag_rules: &HashMap<&str, HashMap<&str, usize>>, bag: &str) -> Result<usize> {
    Ok(recursive_count(bag_rules, bag) - 1)
}

#[cfg(test)]
mod tests_2007 {
    use super::*;
    use aoc_lib::Example;
    use maplit::hashmap;

    #[test]
    fn parse_test() {
        let input = aoc_lib::input(2020, 7)
            .example(Example::Part1, 1)
            .open()
            .unwrap();

        let expected = hashmap! {
            "light red" => hashmap!{
                "bright white" => 1,
                "muted yellow" => 2,
            },
            "dark orange" => hashmap! {
                "bright white" => 3,
                "muted yellow" => 4,
            },
            "bright white" => hashmap! {
                "shiny gold" => 1,
            },
            "muted yellow" => hashmap! {
                "shiny gold" => 2,
                "faded blue" => 9,
            },
            "shiny gold" => hashmap! {
                "dark olive" => 1,
                "vibrant plum" => 2,
            },
            "dark olive" => hashmap! {
                "faded blue" => 3,
                "dotted black" => 4,
            },
            "vibrant plum" => hashmap! {
                "faded blue" => 5,
                "dotted black" => 6,
            },
            "faded blue" => HashMap::new(),
            "dotted black" => HashMap::new(),
        };

        let actual = parse_bags(&input).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn part1_example() {
        let input = aoc_lib::input(2020, 7)
            .example(Example::Part1, 1)
            .open()
            .unwrap();
        let bags = parse_bags(&input).unwrap();

        let expected = 4;
        let actual = part1(&bags, "shiny gold").unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn part2_example1() {
        let input = aoc_lib::input(2020, 7)
            .example(Example::Part1, 1)
            .open()
            .unwrap();
        let bags = parse_bags(&input).unwrap();

        let expected = 32;
        let actual = part2(&bags, "shiny gold").unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn part2_example2() {
        let input = aoc_lib::input(2020, 7)
            .example(Example::Part2, 1)
            .open()
            .unwrap();
        let bags = parse_bags(&input).unwrap();

        let expected = 126;
        let actual = part2(&bags, "shiny gold").unwrap();

        assert_eq!(expected, actual);
    }
}
