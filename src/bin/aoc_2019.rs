use aoc_lib::{
    parsers::{split_pair, unsigned_number},
    TracingAlloc,
};
use color_eyre::{
    eyre::{eyre, Result},
    Report,
};
use nom::{
    bytes::complete::tag,
    character::complete::{anychar, char},
    multi::separated_list1,
    sequence::{delimited, separated_pair},
    IResult,
};

use std::{collections::HashMap, num::ParseIntError};

#[global_allocator]
static ALLOC: TracingAlloc = TracingAlloc::new();

#[derive(Debug, Clone, PartialEq, Eq)]
enum RuleValidation {
    Const(char),
    Seq(Vec<usize>),
    Either { left: Vec<usize>, right: Vec<usize> },
}

impl RuleValidation {
    fn parse(input: &str) -> Result<RuleValidation> {
        fn const_parser(input: &str) -> IResult<&str, char> {
            delimited(char('"'), anychar, char('"'))(input)
        }
        fn seq_parser(input: &str) -> IResult<&str, Vec<Result<usize, ParseIntError>>> {
            separated_list1(char(' '), unsigned_number)(input)
        }
        fn pair_parser(
            input: &str,
        ) -> IResult<
            &str,
            (
                Vec<Result<usize, ParseIntError>>,
                Vec<Result<usize, ParseIntError>>,
            ),
        > {
            separated_pair(seq_parser, tag(" | "), seq_parser)(input)
        }

        if let Ok(("", ch)) = const_parser(input) {
            Ok(RuleValidation::Const(ch))
        } else if let Ok(("", seq)) = seq_parser(input) {
            Ok(RuleValidation::Seq(
                seq.into_iter().collect::<Result<_, _>>()?,
            ))
        } else if let Ok(("", (left, right))) = pair_parser(input) {
            Ok(RuleValidation::Either {
                left: left.into_iter().collect::<Result<_, _>>()?,
                right: right.into_iter().collect::<Result<_, _>>()?,
            })
        } else {
            Err(eyre!("Invalid input: {}", input))
        }
    }

    fn test_seq<'a>(
        sequence: &[usize],
        rules: &HashMap<usize, RuleValidation>,
        input: &'a str,
        depth: usize,
    ) -> Result<&'a str, (&'a str, Report)> {
        let mut local_input = input;

        for (i, id) in sequence.iter().enumerate() {
            eprintln!("Testing Seq rule: {}", id);
            let rule = rules
                .get(id)
                .ok_or_else(|| (input, eyre!("Id not found in ruleset: {}", id)))?;

            // Need special handling here, so we can backtrack and try the right side if the left fails.
            local_input = match rule {
                RuleValidation::Either { left, right } => {
                    let rem_seq = &sequence[i + 1..];

                    eprintln!("(Seq) Either rule: {:?} | {:?}", left, right);
                    eprintln!("Left");

                    let left_res = RuleValidation::test_seq(left, rules, local_input, depth + 1)
                        .and_then(|rem| {
                            eprintln!("Continuing sequence: {:?} from {:?}", sequence, rem_seq);
                            RuleValidation::test_seq(rem_seq, rules, rem, depth + 1)
                        });

                    match left_res {
                        Ok(rem) => {
                            return Ok(rem);
                        }
                        Err(_) => {
                            eprintln!("Right");
                            return RuleValidation::test_seq(right, rules, local_input, depth + 1)
                                .and_then(|rem| {
                                    eprintln!(
                                        "Continuing sequence: {:?} from {:?}",
                                        sequence, rem_seq
                                    );
                                    RuleValidation::test_seq(rem_seq, rules, rem, depth + 1)
                                });
                        }
                    }
                }
                _ => rule.test_rule(rules, local_input, depth)?,
            };
        }

        Ok(local_input)
    }

    fn test_rule<'a>(
        &self,
        rules: &HashMap<usize, RuleValidation>,
        input: &'a str,
        depth: usize,
    ) -> Result<&'a str, (&'a str, Report)> {
        eprint!("Depth: {} - Testing: {} - ", depth, input);
        match self {
            RuleValidation::Const(c) => {
                eprint!("Const rule: {} - ", c);
                if input.starts_with(*c) {
                    eprintln!("OK");
                    Ok(&input[c.len_utf8()..])
                } else {
                    eprintln!("Err");
                    Err((
                        input,
                        eyre!("Pattern `{}` not found in input: {}", c, input),
                    ))
                }
            }
            RuleValidation::Seq(seq) => {
                eprintln!("Seq rule: {:?}", seq);
                RuleValidation::test_seq(seq, rules, input, depth + 1)
            }
            RuleValidation::Either { left, right } => {
                eprintln!("Either rule: {:?} | {:?}", left, right);
                eprintln!("Left");
                RuleValidation::test_seq(left, rules, input, depth + 1).or_else(|_| {
                    eprintln!("Right");
                    RuleValidation::test_seq(right, rules, input, depth + 1)
                })
            }
        }
    }
}

fn parse_input(input: &str) -> Result<(HashMap<usize, RuleValidation>, Vec<&str>)> {
    let (rules, data) = split_pair(input, "\n\n")?;

    let rules = rules
        .trim()
        .lines()
        .map(str::trim)
        .map(|l| -> Result<(usize, RuleValidation)> {
            let (id, rule) = split_pair(l, ": ")?;
            Ok((id.parse()?, RuleValidation::parse(rule)?))
        })
        .collect::<Result<_, _>>()?;

    let data = data.trim().lines().map(str::trim).collect();

    Ok((rules, data))
}

fn get_valid_count(rules: &HashMap<usize, RuleValidation>, data: &[&str]) -> Result<usize> {
    let first_rule = &rules[&0];
    let count = data
        .iter()
        .filter(|l| {
            eprintln!("\n---");
            match first_rule.test_rule(rules, l, 0) {
                Ok("") => {
                    eprintln!("OK");
                    true
                }
                Ok(rem) => {
                    eprintln!("OK, with remainder: `{}`", rem);
                    false
                }
                Err(_) => {
                    eprintln!("Failure");
                    false
                }
            }
        })
        .count();

    Ok(count)
}

fn print_rules(rules: &HashMap<usize, RuleValidation>) {
    for (id, rule) in rules {
        eprint!("{}: ", id);

        match rule {
            RuleValidation::Const(c) => eprintln!("\"{}\"", c),
            RuleValidation::Seq(seq) => eprintln!("{:?}", seq),
            RuleValidation::Either { left, right } => eprintln!("{:?} | {:?}", left, right),
        }
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let input = aoc_lib::input(2020, 19).open()?;
    let (rules, data) = parse_input(&input)?;

    aoc_lib::run(
        &ALLOC,
        "Day 19: Monster Messages",
        (&rules, &*data),
        &|(rules, data)| get_valid_count(rules, data),
        &|_| Ok("Not Implemented"),
    )
}

#[cfg(test)]
mod tests_2019 {
    use super::*;
    use maplit::hashmap;

    #[test]
    fn rule_segment_parse_test() {
        let tests = [
            ("\"a\"", RuleValidation::Const('a')),
            ("12", RuleValidation::Seq(vec![12])),
            ("12 41", RuleValidation::Seq(vec![12, 41])),
            (
                "12 13 | 14 15",
                RuleValidation::Either {
                    left: vec![12, 13],
                    right: vec![14, 15],
                },
            ),
        ];

        for (test, expected) in &tests {
            let actual = RuleValidation::parse(test).unwrap();
            assert_eq!(&actual, expected);
        }
    }

    #[test]
    fn parse_test() {
        let input = aoc_lib::input(2020, 19).example(0, 1).open().unwrap();
        let expected_rules = hashmap! {
            0 => RuleValidation::Seq(vec![4, 1, 5]),
            1 => RuleValidation::Either{left: vec![2, 3], right: vec![3, 2]},
            2 => RuleValidation::Either{left: vec![4, 4], right: vec![5, 5]},
            3 => RuleValidation::Either{left: vec![4, 5], right: vec![5, 4]},
            4 => RuleValidation::Const('a'),
            5 => RuleValidation::Const('b'),
        };
        let expected_data = vec!["ababbb", "bababa", "abbbab", "aaabbb", "aaaabbb"];

        let (actual_rules, actual_data) = parse_input(&input).unwrap();

        assert_eq!(expected_rules, actual_rules);
        assert_eq!(expected_data, actual_data);
    }

    #[test]
    fn rule_test() {
        let rules = hashmap! {
            0 => RuleValidation::Const('a'),
            1 => RuleValidation::Const('b'),
            2 => RuleValidation::Seq(vec![0, 1]),
            3 => RuleValidation::Either{left: vec![2, 1], right: vec![1, 2]},
        };

        let res = rules[&0].test_rule(&rules, "a", 0).unwrap();
        assert_eq!(res, "");

        let res = rules[&2].test_rule(&rules, "ab", 0).unwrap();
        assert_eq!(res, "");

        let res = rules[&3].test_rule(&rules, "abb", 0).unwrap();
        assert_eq!(res, "");

        let res = rules[&3].test_rule(&rules, "bab", 0).unwrap();
        assert_eq!(res, "");
    }

    #[test]
    fn part1_example() {
        let input = aoc_lib::input(2020, 19).example(0, 1).open().unwrap();
        let (rules, data) = parse_input(&input).unwrap();

        let expected = 2;
        let actual = get_valid_count(&rules, &data).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn part2_example1() {
        let input = aoc_lib::input(2020, 19).example(2, 1).open().unwrap();
        let (rules, data) = parse_input(&input).unwrap();

        let expected = 3;
        let actual = get_valid_count(&rules, &data).unwrap();

        assert_eq!(expected, actual);
    }

    // #[test]
    fn part2_example2() {
        let input = aoc_lib::input(2020, 19).example(2, 1).open().unwrap();
        let (mut rules, data) = parse_input(&input).unwrap();

        rules.insert(
            8,
            RuleValidation::Either {
                left: vec![42],
                right: vec![42, 8],
            },
        );
        rules.insert(
            11,
            RuleValidation::Either {
                left: vec![42, 31],
                right: vec![42, 11, 31],
            },
        );

        let expected = 12;
        let actual = get_valid_count(&rules, &data).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn part2_example3() {
        let input = aoc_lib::input(2020, 19).example(2, 1).open().unwrap();
        let (mut rules, _) = parse_input(&input).unwrap();

        rules.insert(
            8,
            RuleValidation::Either {
                left: vec![42],
                right: vec![42, 8],
            },
        );
        rules.insert(
            11,
            RuleValidation::Either {
                left: vec![42, 31],
                right: vec![42, 11, 31],
            },
        );

        print_rules(&rules);

        let test = "bbbbbbbaaaabbbbaaabbabaaa";
        let rem = rules[&0].test_rule(&rules, test, 0).unwrap();

        assert_eq!("", rem);
    }

    #[test]
    fn part2_example4() {
        let input = aoc_lib::input(2020, 19).example(2, 2).open().unwrap();
        let (rules, data) = parse_input(&input).unwrap();

        let expected = 1;
        let actual = get_valid_count(&rules, &data).unwrap();

        assert_eq!(expected, actual);
    }
}
