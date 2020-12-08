use std::num::ParseIntError;

use aoc_lib::{parsers::split_pair, TracingAlloc};
use color_eyre::eyre::{eyre, Result};
use nom::bytes::complete::take_while;

#[global_allocator]
static ALLOC: TracingAlloc = TracingAlloc::new();

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
        matches!(self, Passport {
            birth_year: Some(_),
            issue_year: Some(_),
            expiration_year: Some(_),
            height: Some(_),
            hair_color: Some(_),
            eye_color: Some(_),
            passport_id: Some(_),
            country_id: _,
        })
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

    fn is_eye_color_valid(ecl: &str) -> bool {
        matches!(ecl, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth")
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
                eye_color: Some(ecl),
                passport_id: Some(pid),
                country_id: _,
            }
            if Passport::is_height_valid(hgt) && Passport::is_hair_color_valid(hcl) && Passport::is_eye_color_valid(ecl) && Passport::is_passport_id_valid(pid)
        )
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let input = std::fs::read_to_string("inputs/aoc_2004.txt")?;
    let passports = Passport::parse_passports(&input)?;

    aoc_lib::run(
        &ALLOC,
        "Day 4: Passport Processing",
        &passports,
        &|passports| Ok(passports.iter().filter(|p| p.is_valid_part1()).count()),
        &|passports| Ok(passports.iter().filter(|p| p.is_valid_part2()).count()),
    )
}

#[cfg(test)]
mod tests_2004 {
    use super::*;

    #[test]
    fn parse_test() {
        let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
        byr:1937 iyr:2017 cid:147 hgt:183cm

        iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
        hcl:#cfa07d byr:1929

        hcl:#ae17e1 iyr:2013
        eyr:2024
        ecl:brn pid:760753108 byr:1931
        hgt:179cm

        hcl:#cfa07d eyr:2025 pid:166559648
        iyr:2011 ecl:brn hgt:59in";

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
        let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
        byr:1937 iyr:2017 cid:147 hgt:183cm

        iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
        hcl:#cfa07d byr:1929

        hcl:#ae17e1 iyr:2013
        eyr:2024
        ecl:brn pid:760753108 byr:1931
        hgt:179cm

        hcl:#cfa07d eyr:2025 pid:166559648
        iyr:2011 ecl:brn hgt:59in";

        let passports = Passport::parse_passports(input).unwrap();

        let expected = [true, false, true, false];

        for (i, (passport, expected)) in passports.iter().zip(&expected).enumerate() {
            assert_eq!(passport.is_valid_part1(), *expected, "{}", i);
        }
    }
}
