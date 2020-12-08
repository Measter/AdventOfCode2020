use std::collections::{HashMap, HashSet};

use aoc_lib::{parsers::unsigned_number, TracingAlloc};
use color_eyre::eyre::{eyre, Result};
use nom::{
    bytes::complete::{tag, take_until},
    error::ErrorKind,
    sequence::tuple,
};

#[global_allocator]
static ALLOC: TracingAlloc = TracingAlloc::new();

fn parse_bags(input: &str) -> Result<HashMap<&str, HashMap<&str, usize>>> {
    let mut bags = HashMap::new();

    for line in input.lines().map(str::trim) {
        let (contained, (this_bag, _)) =
            tuple::<_, _, (&str, ErrorKind), _>((take_until(" bags"), tag(" bags contain ")))(line)
                .map_err(|e| eyre!("Parse Error: {}", e))?;

        let bag_entry = bags.entry(this_bag).or_insert(HashMap::new());

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

fn main() -> Result<()> {
    color_eyre::install()?;

    let input = std::fs::read_to_string("inputs/aoc_2007.txt")?;
    let rules = parse_bags(&input)?;

    aoc_lib::run(
        &ALLOC,
        "Day 7: Handy Haversacks",
        &rules,
        &|bag_rules| part1(bag_rules, "shiny gold"),
        &|bag_rules| part2(bag_rules, "shiny gold"),
    )
}

#[cfg(test)]
mod tests_2007 {
    use super::*;
    use maplit::hashmap;

    #[test]
    fn parse_test() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
        dark orange bags contain 3 bright white bags, 4 muted yellow bags.
        bright white bags contain 1 shiny gold bag.
        muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
        shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
        dark olive bags contain 3 faded blue bags, 4 dotted black bags.
        vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
        faded blue bags contain no other bags.
        dotted black bags contain no other bags.";

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
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
        dark orange bags contain 3 bright white bags, 4 muted yellow bags.
        bright white bags contain 1 shiny gold bag.
        muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
        shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
        dark olive bags contain 3 faded blue bags, 4 dotted black bags.
        vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
        faded blue bags contain no other bags.
        dotted black bags contain no other bags.";

        let bags = parse_bags(input).unwrap();

        let expected = 4;
        let actual = part1(&bags, "shiny gold").unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn part2_example1() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
        dark orange bags contain 3 bright white bags, 4 muted yellow bags.
        bright white bags contain 1 shiny gold bag.
        muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
        shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
        dark olive bags contain 3 faded blue bags, 4 dotted black bags.
        vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
        faded blue bags contain no other bags.
        dotted black bags contain no other bags.";

        let bags = parse_bags(input).unwrap();

        let expected = 32;
        let actual = part2(&bags, "shiny gold").unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn part2_example2() {
        let input = "shiny gold bags contain 2 dark red bags.
        dark red bags contain 2 dark orange bags.
        dark orange bags contain 2 dark yellow bags.
        dark yellow bags contain 2 dark green bags.
        dark green bags contain 2 dark blue bags.
        dark blue bags contain 2 dark violet bags.
        dark violet bags contain no other bags.";

        let bags = parse_bags(input).unwrap();

        let expected = 126;
        let actual = part2(&bags, "shiny gold").unwrap();

        assert_eq!(expected, actual);
    }
}