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
enum SuccessType<'a> {
    Finish,
    Remainder(&'a str),
    Branch(Vec<&'a str>),
}

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
    ) -> Result<SuccessType<'a>, (&'a str, Report)> {
        let (id, rem_seq) = match sequence.split_first() {
            None if input.is_empty() => {
                // eprintln!("Seq Finished - Depth: {}", depth);
                return Ok(SuccessType::Finish);
            }
            None => {
                // eprintln!("Seq Finished - Depth: {} - Rem: {}", depth, input);
                return Ok(SuccessType::Remainder(input));
            }
            Some(val) => val,
        };

        // eprintln!(
        //     "Depth: {} - Testing Seq rule: {} - Testing: {}",
        //     depth, id, input
        // );
        let rule = rules
            .get(id)
            .ok_or_else(|| (input, eyre!("Id not found in ruleset: {}", id)))?;

        // Need special handling here, so we can backtrack and try the right side if the left fails.
        match rule {
            RuleValidation::Either { left, right } => {
                let calc_side = |side: &[usize]| {
                    RuleValidation::test_seq(side, rules, input, depth + 1).and_then(
                        |rem| -> Result<SuccessType, (&str, Report)> {
                            // eprintln!("Continuing sequence: {:?} from 1: {:?}", sequence, rem_seq);
                            match rem {
                                SuccessType::Finish if rem_seq.is_empty() => {
                                    Ok(SuccessType::Finish)
                                }
                                SuccessType::Finish => Err((input, eyre!("Incomplete sequence"))),
                                SuccessType::Remainder(rem) => {
                                    RuleValidation::test_seq(rem_seq, rules, rem, depth)
                                }
                                SuccessType::Branch(branches) => {
                                    let mut success_branches = Vec::new();
                                    let mut last_err = None;

                                    for branch in branches {
                                        match RuleValidation::test_seq(
                                            rem_seq, rules, branch, depth,
                                        ) {
                                            Ok(SuccessType::Finish) => {
                                                return Ok(SuccessType::Finish)
                                            }
                                            Ok(SuccessType::Remainder(rem)) => {
                                                success_branches.push(rem)
                                            }
                                            Ok(SuccessType::Branch(seq)) => {
                                                success_branches.extend(seq)
                                            }
                                            Err(e) => last_err = Some(e),
                                        }
                                    }

                                    match (success_branches.as_slice(), last_err) {
                                        ([], Some(e)) => Err(e),
                                        ([], None) => panic!("Should have an error here"),
                                        ([rem], _) => Ok(SuccessType::Remainder(*rem)),
                                        (_, _) => Ok(SuccessType::Branch(success_branches)),
                                    }
                                }
                            }
                        },
                    )
                };

                // eprintln!("(Seq) Either rule: {:?} | {:?}", left, right);
                // eprintln!("Left - Depth: {} - Seq {:?}", depth, left);
                let left_res = calc_side(left);
                // eprintln!("Right - Depth: {} - Seq {:?}", depth, right);
                let right_res = calc_side(right);

                match (left_res, right_res) {
                    (Ok(SuccessType::Finish), _) | (_, Ok(SuccessType::Finish)) => {
                        Ok(SuccessType::Finish)
                    }
                    (Ok(SuccessType::Remainder(l_rem)), Ok(SuccessType::Remainder(r_rem))) => {
                        Ok(SuccessType::Branch(vec![l_rem, r_rem]))
                    }
                    (Ok(SuccessType::Branch(mut l_branch)), Ok(SuccessType::Branch(r_branch))) => {
                        l_branch.extend(r_branch);
                        Ok(SuccessType::Branch(l_branch))
                    }
                    (Ok(SuccessType::Remainder(rem)), Ok(SuccessType::Branch(mut branch)))
                    | (Ok(SuccessType::Branch(mut branch)), Ok(SuccessType::Remainder(rem))) => {
                        branch.push(rem);
                        Ok(SuccessType::Branch(branch))
                    }
                    (Err(_), Err(e)) => Err(e),
                    (Ok(succ), Err(_)) | (Err(_), Ok(succ)) => Ok(succ),
                }
            }
            _ => rule.test_rule(rules, input, depth + 1).and_then(
                |rem| -> Result<SuccessType, (&str, Report)> {
                    // eprintln!("Continuing sequences: {:?} from 1: {:?}", sequence, rem_seq);
                    match rem {
                        SuccessType::Finish if rem_seq.is_empty() => Ok(SuccessType::Finish),
                        SuccessType::Finish => Err((input, eyre!("Incomplete sequence"))),
                        SuccessType::Remainder(rem) => {
                            RuleValidation::test_seq(rem_seq, rules, rem, depth)
                        }
                        SuccessType::Branch(branches) => {
                            let mut success_branches = Vec::new();
                            let mut last_err = None;

                            for branch in branches {
                                match RuleValidation::test_seq(rem_seq, rules, branch, depth) {
                                    Ok(SuccessType::Finish) => return Ok(SuccessType::Finish),
                                    Ok(SuccessType::Remainder(rem)) => success_branches.push(rem),
                                    Ok(SuccessType::Branch(seq)) => success_branches.extend(seq),
                                    Err(e) => last_err = Some(e),
                                }
                            }

                            match (success_branches.as_slice(), last_err) {
                                ([], Some(e)) => Err(e),
                                ([], None) => panic!("Should have an error here"),
                                ([rem], _) => Ok(SuccessType::Remainder(*rem)),
                                (_, _) => Ok(SuccessType::Branch(success_branches)),
                            }
                        }
                    }
                },
            ),
        }
    }

    fn test_rule<'a>(
        &self,
        rules: &HashMap<usize, RuleValidation>,
        input: &'a str,
        depth: usize,
    ) -> Result<SuccessType<'a>, (&'a str, Report)> {
        // eprint!("Depth: {} - Testing: {} - ", depth, input);
        match self {
            RuleValidation::Const(c) => {
                // eprint!("Const rule: {} - ", c);
                if input.starts_with(*c) {
                    // eprintln!("OK");
                    let rem = &input[c.len_utf8()..];
                    if rem.is_empty() {
                        Ok(SuccessType::Finish)
                    } else {
                        Ok(SuccessType::Remainder(rem))
                    }
                } else {
                    // eprintln!("Err");
                    Err((
                        input,
                        eyre!("Pattern `{}` not found in input: {}", c, input),
                    ))
                }
            }
            RuleValidation::Seq(seq) => {
                // eprintln!("Seq rule: {:?}", seq);
                RuleValidation::test_seq(seq, rules, input, depth + 1)
            }
            RuleValidation::Either { left, right } => {
                // eprintln!("Either rule: {:?} | {:?}", left, right);
                let left_res = RuleValidation::test_seq(left, rules, input, depth + 1);
                let right_res = RuleValidation::test_seq(right, rules, input, depth + 1);

                match (left_res, right_res) {
                    (Ok(SuccessType::Finish), _) | (_, Ok(SuccessType::Finish)) => {
                        Ok(SuccessType::Finish)
                    }
                    (Ok(SuccessType::Remainder(l_rem)), Ok(SuccessType::Remainder(r_rem))) => {
                        Ok(SuccessType::Branch(vec![l_rem, r_rem]))
                    }
                    (Ok(SuccessType::Branch(mut l_branch)), Ok(SuccessType::Branch(r_branch))) => {
                        l_branch.extend(r_branch);
                        Ok(SuccessType::Branch(l_branch))
                    }
                    (Ok(SuccessType::Remainder(rem)), Ok(SuccessType::Branch(mut branch)))
                    | (Ok(SuccessType::Branch(mut branch)), Ok(SuccessType::Remainder(rem))) => {
                        branch.push(rem);
                        Ok(SuccessType::Branch(branch))
                    }
                    (Err(_), Err(e)) => Err(e),
                    (Ok(succ), Err(_)) | (Err(_), Ok(succ)) => Ok(succ),
                }
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
            // eprintln!("\n---");
            match first_rule.test_rule(rules, l, 0) {
                Ok(SuccessType::Finish) => {
                    // eprintln!("OK");
                    true
                }
                Ok(_) => {
                    // eprintln!("OK, with remainder(s): `{:?}`", rem);
                    false
                }
                Err(_) => {
                    // eprintln!("Failure");
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
        &|(rules, data)| {
            let mut rules = rules.clone();
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
            get_valid_count(&rules, data)
        },
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
        assert_eq!(res, SuccessType::Finish);

        let res = rules[&2].test_rule(&rules, "ab", 0).unwrap();
        assert_eq!(res, SuccessType::Finish);

        let res = rules[&3].test_rule(&rules, "abb", 0).unwrap();
        assert_eq!(res, SuccessType::Finish);

        let res = rules[&3].test_rule(&rules, "bab", 0).unwrap();
        assert_eq!(res, SuccessType::Finish);
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

    #[test]
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

        let test = "babbbbaabbbbbabbbbbbaabaaabaaa";
        let rem = rules[&0].test_rule(&rules, test, 0).unwrap();

        assert_eq!(rem, SuccessType::Finish);
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
