use nom::{
    bytes::complete::{tag, take_while1},
    combinator::{map, opt},
    sequence::tuple,
    IResult,
};

use std::{num::ParseIntError, ops::Neg, str::FromStr};

pub fn signed_number<F>(input: &str) -> IResult<&str, Result<F, ParseIntError>>
where
    F: FromStr<Err = ParseIntError> + Neg<Output = F>,
{
    map(
        tuple((
            map(opt(tag("-")), |o: Option<&str>| o.is_some()),
            take_while1(|c: char| c.is_ascii_digit()),
        )),
        |(is_neg, num)| num.parse::<F>().map(|n| if is_neg { -n } else { n }),
    )(input)
}

pub fn unsigned_number<F>(input: &str) -> IResult<&str, Result<F, ParseIntError>>
where
    F: FromStr<Err = ParseIntError>,
{
    map(take_while1(|c: char| c.is_ascii_digit()), |num: &str| {
        num.parse::<F>()
    })(input)
}
