use std::num::ParseIntError;

use aoc_lib::{day, parsers::split_pair, Bench, BenchResult, NoError, UserError};
use color_eyre::eyre::{eyre, Result};
use nom::bytes::complete::take_while;

day! {
    day 4: "Passport Processing"
    1: run_part1
    2: run_part2
}

fn run_part1(input: &str, b: Bench) -> BenchResult {
    let passports = Passport::parse_passports(input).map_err(UserError)?;

    b.bench(|| Ok::<_, NoError>(passports.iter().filter(|p| p.is_valid_part1()).count()))
}

fn run_part2(input: &str, b: Bench) -> BenchResult {
    let passports = Passport::parse_passports(input).map_err(UserError)?;

    b.bench(|| Ok::<_, NoError>(passports.iter().filter(|p| p.is_valid_part2()).count()))
}

#[derive(Debug, PartialEq, Default)]
struct Passport<'a> {
    birth_year: Option<u16>,
    issue_year: Option<u16>,
    expiration_year: Option<u16>,
    height: Option<&'a str>,
    hair_color: Option<&'a str>,
    eye_color: Option<&'a str>,
    passport_id: Option<&'a str>,
    country_id: Option<u16>,
}

impl<'a> Passport<'a> {
    fn parse_passports(input: &'a str) -> Result<Vec<Passport<'a>>> {
        let mut passports = Vec::new();
        let raw_passports = input.split("\n\n");

        for raw_passport in raw_passports {
            let passport_fields = raw_passport
                .lines()
                .map(str::trim)
                .flat_map(str::split_whitespace);

            let mut passport = Passport::default();

            for field in passport_fields {
                let (field, value) = split_pair(field, ":")?;

                match field {
                    "byr" => passport.birth_year = Some(value.parse()?),
                    "iyr" => passport.issue_year = Some(value.parse()?),
                    "eyr" => passport.expiration_year = Some(value.parse()?),
                    "hgt" => passport.height = Some(value),
                    "hcl" => passport.hair_color = Some(value),
                    "ecl" => passport.eye_color = Some(value),
                    "pid" => passport.passport_id = Some(value),
                    "cid" => passport.country_id = Some(value.parse()?),
                    _ => return Err(eyre!("Unknown field: {}", field)),
                }
            }

            passports.push(passport);
        }

        Ok(passports)
    }

    fn is_valid_part1(&self) -> bool {
        matches!(
            self,
            Passport {
                birth_year: Some(_),
                issue_year: Some(_),
                expiration_year: Some(_),
                height: Some(_),
                hair_color: Some(_),
                eye_color: Some(_),
                passport_id: Some(_),
                country_id: _,
            }
        )
    }

    fn is_height_valid(hgt: &str) -> bool {
        let v = take_while::<_, _, ()>(|c: char| c.is_ascii_digit())(hgt)
            .map(|(unit, val)| Ok::<_, ParseIntError>((unit, val.parse::<u8>()?)));

        matches!(v, Ok(Ok(("cm", 150..=193))) | Ok(Ok(("in", 59..=76))))
    }

    fn is_hair_color_valid(hcl: &str) -> bool {
        if !hcl.starts_with('#') {
            return false;
        }

        hcl[1..].chars().all(|c| c.is_ascii_hexdigit()) && hcl.len() == 7
    }

    fn is_passport_id_valid(pid: &str) -> bool {
        pid.chars().all(|c| c.is_ascii_digit()) && pid.len() == 9
    }

    fn is_valid_part2(&self) -> bool {
        matches!(self, Passport {
                birth_year: Some(1920..=2002),
                issue_year: Some(2010..=2020),
                expiration_year: Some(2020..=2030),
                height: Some(hgt),
                hair_color: Some(hcl),
                eye_color: Some("amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth"),
                passport_id: Some(pid),
                country_id: _,
            }
            if Passport::is_height_valid(hgt) && Passport::is_hair_color_valid(hcl) && Passport::is_passport_id_valid(pid)
        )
    }
}

#[cfg(test)]
mod tests_2004 {
    use aoc_lib::Example;

    use super::*;

    #[test]
    fn parse_test() {
        let input = aoc_lib::input(2020, 4)
            .example(Example::Part1, 1)
            .open()
            .unwrap();

        let expected = [
            Passport {
                eye_color: Some("gry"),
                passport_id: Some("860033327"),
                expiration_year: Some(2020),
                hair_color: Some("#fffffd"),
                birth_year: Some(1937),
                issue_year: Some(2017),
                country_id: Some(147),
                height: Some("183cm"),
            },
            Passport {
                issue_year: Some(2013),
                eye_color: Some("amb"),
                country_id: Some(350),
                expiration_year: Some(2023),
                passport_id: Some("028048884"),
                hair_color: Some("#cfa07d"),
                birth_year: Some(1929),
                ..Default::default()
            },
            Passport {
                hair_color: Some("#ae17e1"),
                issue_year: Some(2013),
                expiration_year: Some(2024),
                eye_color: Some("brn"),
                passport_id: Some("760753108"),
                birth_year: Some(1931),
                height: Some("179cm"),
                ..Default::default()
            },
            Passport {
                hair_color: Some("#cfa07d"),
                expiration_year: Some(2025),
                passport_id: Some("166559648"),
                issue_year: Some(2011),
                eye_color: Some("brn"),
                height: Some("59in"),
                ..Default::default()
            },
        ];

        let actual = Passport::parse_passports(&input).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn part1_example() {
        let input = aoc_lib::input(2020, 4)
            .example(Example::Part1, 1)
            .open()
            .unwrap();

        let passports = Passport::parse_passports(&input).unwrap();

        let expected = [true, false, true, false];

        for (i, (passport, expected)) in passports.iter().zip(&expected).enumerate() {
            assert_eq!(passport.is_valid_part1(), *expected, "{}", i);
        }
    }
}
