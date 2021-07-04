use std::{
    collections::{HashMap, HashSet},
    num::ParseIntError,
    ops::RangeInclusive,
};

use aoc_lib::{
    day,
    parsers::{split_pair, unsigned_number},
    Bench, BenchResult, UserError,
};
use color_eyre::eyre::{eyre, Result};
use itertools::Itertools;
use nom::{
    bytes::complete::{tag, take_till1},
    sequence::tuple,
};

day! {
    day 16: "Ticket Translation"
    1: run_part1
    2: run_part2
}

fn run_part1(input: &str, b: Bench) -> BenchResult {
    let ticket_data = TicketData::parse(input).map_err(UserError)?;
    b.bench(|| ticket_data.sum_invalid_fields())
}

fn run_part2(input: &str, b: Bench) -> BenchResult {
    let ticket_data = TicketData::parse(input).map_err(UserError)?;
    b.bench(|| part2(&ticket_data))
}

type ValidationInfo<'a> = HashMap<&'a str, [RangeInclusive<u64>; 2]>;

#[derive(Debug, PartialEq)]
struct TicketData<'a> {
    validation_rules: ValidationInfo<'a>,
    my_ticket: Vec<u64>,
    nearby_tickets: Vec<Vec<u64>>,
}

impl<'a> TicketData<'a> {
    fn parse(input: &'a str) -> Result<TicketData> {
        let (rules, rest) = split_pair(input, "your ticket:")?;
        let (my_ticket, nearby_tickets) = split_pair(rest, "nearby tickets:")?;

        let mut validation_rules = HashMap::new();

        for rule in rules.trim().lines().map(str::trim) {
            let (_, (rule_name, _, min1, _, max1, _, min2, _, max2)) = tuple((
                take_till1(|c: char| c == ':'),
                tag(": "),
                unsigned_number::<u64>,
                tag("-"),
                unsigned_number::<u64>,
                tag(" or "),
                unsigned_number::<u64>,
                tag("-"),
                unsigned_number::<u64>,
            ))(rule)
            .map_err(|e| eyre!("Error parsing input: {}", e))?;

            validation_rules.insert(rule_name, [min1?..=max1?, min2?..=max2?]);
        }

        let my_ticket = my_ticket
            .trim()
            .split(',')
            .map(str::parse)
            .collect::<Result<_, ParseIntError>>()?;
        let nearby_tickets = nearby_tickets
            .trim()
            .lines()
            .map(str::trim)
            .map(|l| {
                l.split(',')
                    .map(str::parse)
                    .collect::<Result<Vec<_>, ParseIntError>>()
            })
            .collect::<Result<_, ParseIntError>>()?;

        Ok(TicketData {
            validation_rules,
            my_ticket,
            nearby_tickets,
        })
    }

    fn is_valid_ticket(&self, ticket: &[u64]) -> bool {
        ticket.iter().all(|field| {
            self.validation_rules
                .values()
                .any(|[first, second]| first.contains(field) || second.contains(field))
        })
    }

    fn sum_invalid_fields(&self) -> Result<u64> {
        let invalids = self
            .nearby_tickets
            .iter()
            .flat_map(|i| i.iter())
            .map(|field| {
                self.validation_rules
                    .values()
                    .find(|[first, second]| first.contains(field) || second.contains(field))
                    .map(|_| 0)
                    .unwrap_or(*field)
            })
            .sum();

        Ok(invalids)
    }
}

fn part2(data: &TicketData) -> Result<u64> {
    let valid_tickets: Vec<_> = data
        .nearby_tickets
        .iter()
        .map(Vec::as_slice)
        .filter(|t| data.is_valid_ticket(t))
        .collect();

    let mut field_tracker: Vec<HashSet<_>> =
        std::iter::repeat_with(|| data.validation_rules.keys().copied().collect())
            .take(data.my_ticket.len())
            .collect();

    // Check if any of the field values are out of the valid ranges, and remove them if they are.
    for ((name, [first, second]), ticket) in data
        .validation_rules
        .iter()
        .cartesian_product(&valid_tickets)
    {
        ticket
            .iter()
            .zip(&mut field_tracker)
            .filter(|(field_value, _)| {
                !first.contains(field_value) && !second.contains(field_value)
            })
            .for_each(|(_, tracker)| {
                tracker.remove(name);
            });
    }

    loop {
        let mut did_advance = false;

        // If we've narrowed one field down to a single possibility, remove it from the others.
        for i in 0..field_tracker.len() {
            if field_tracker[i].len() != 1 {
                continue;
            }
            let entry = *field_tracker[i].iter().next().unwrap();

            field_tracker
                .iter_mut()
                .enumerate()
                .filter(|(j, _)| *j != i)
                .for_each(|(_, field)| did_advance |= field.remove(&entry));
        }

        if !did_advance {
            return Err(eyre!("Didn't advance filter"));
        }

        if field_tracker.iter().any(|f| f.is_empty()) {
            return Err(eyre!("One field tracker ended up completely empty"));
        }

        if field_tracker.iter().all(|f| f.len() == 1) {
            break;
        }
    }

    let result = field_tracker
        .iter()
        .zip(&data.my_ticket)
        .filter_map(|(f, ticket)| {
            f.iter()
                .next()
                .filter(|f| f.starts_with("departure"))
                .map(|_| *ticket)
        })
        .product();

    Ok(result)
}

#[cfg(test)]
mod tests_2016 {
    use aoc_lib::Example;
    use maplit::hashmap;

    use super::*;

    #[test]
    fn parse_test() {
        let input = aoc_lib::input(2020, 16)
            .example(Example::Part1, 1)
            .open()
            .unwrap();
        let actual = TicketData::parse(&input).unwrap();

        let expected = TicketData {
            validation_rules: hashmap! {
                "class" => [1..=3, 5..=7],
                "row" => [6..=11, 33..=44],
                "seat" => [13..=40, 45..=50],
            },

            my_ticket: vec![7, 1, 14],
            nearby_tickets: vec![
                vec![7, 3, 47],
                vec![40, 4, 50],
                vec![55, 2, 20],
                vec![38, 6, 12],
            ],
        };

        assert_eq!(expected, actual)
    }

    #[test]
    fn part1_example() {
        let input = aoc_lib::input(2020, 16)
            .example(Example::Part1, 1)
            .open()
            .unwrap();
        let data = TicketData::parse(&input).unwrap();

        let expected = 71;
        let actual = data.sum_invalid_fields().unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn validity_test() {
        let input = aoc_lib::input(2020, 16)
            .example(Example::Part1, 1)
            .open()
            .unwrap();
        let data = TicketData::parse(&input).unwrap();

        let expected = [true, false, false, false];

        for (ticket, expected) in data.nearby_tickets.iter().zip(expected.as_ref()) {
            assert_eq!(data.is_valid_ticket(ticket), *expected);
        }
    }
}
