use std::{
    collections::{HashMap, HashSet},
    num::ParseIntError,
    ops::RangeInclusive,
};

use aoc_lib::{
    parsers::{split_pair, unsigned_number},
    TracingAlloc,
};
use color_eyre::eyre::{eyre, Result};
use nom::{
    bytes::complete::{tag, take_till1},
    sequence::tuple,
};

#[global_allocator]
static ALLOC: TracingAlloc = TracingAlloc::new();

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
                .find(|[first, second]| first.contains(field) || second.contains(field))
                .is_some()
        })
    }

    fn sum_invalid_fields(&self) -> Result<u64> {
        let invalids = self
            .nearby_tickets
            .iter()
            .map(|t| {
                t.iter()
                    .map(|field| {
                        self.validation_rules
                            .values()
                            .find(|[first, second]| first.contains(field) || second.contains(field))
                            .map(|_| 0)
                            .unwrap_or(*field)
                    })
                    .sum::<u64>()
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
    for (name, [first, second]) in &data.validation_rules {
        for ticket in &valid_tickets {
            for (idx, field_value) in ticket.iter().enumerate() {
                if !first.contains(field_value) && !second.contains(field_value) {
                    field_tracker[idx].remove(name);
                }
            }
        }
    }

    loop {
        let mut did_advance = false;

        // If we've narrowed one field down to a single possibility, remove it from the others.
        for i in 0..field_tracker.len() {
            if field_tracker[i].len() == 1 {
                let entry = *field_tracker[i].iter().next().unwrap();
                for j in (0..field_tracker.len()).filter(|&j| j != i) {
                    if field_tracker[j].remove(&entry) {
                        did_advance = true;
                    }
                }
            }
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
        .filter(|(f, _)| matches!(f.iter().next(), Some(f) if f.starts_with("departure")))
        .map(|(_, my_ticket)| *my_ticket)
        .product();

    Ok(result)
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let input = aoc_lib::input(2020, 16).open()?;
    let ticket_data = TicketData::parse(&input)?;

    aoc_lib::run(
        &ALLOC,
        "Day 16: Ticket Translation",
        &ticket_data,
        &|data| data.sum_invalid_fields(),
        &|data| part2(data),
    )
}

#[cfg(test)]
mod tests_2016 {
    use maplit::hashmap;

    use super::*;

    #[test]
    fn parse_test() {
        let input = aoc_lib::input(2020, 16).example(1, 1).open().unwrap();
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
        let input = aoc_lib::input(2020, 16).example(1, 1).open().unwrap();
        let data = TicketData::parse(&input).unwrap();

        let expected = 71;
        let actual = data.sum_invalid_fields().unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn validity_test() {
        let input = aoc_lib::input(2020, 16).example(1, 1).open().unwrap();
        let data = TicketData::parse(&input).unwrap();

        let expected = [true, false, false, false];

        for (ticket, expected) in data.nearby_tickets.iter().zip(expected.as_ref()) {
            assert_eq!(data.is_valid_ticket(ticket), *expected);
        }
    }
}
