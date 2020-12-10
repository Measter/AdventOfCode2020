use aoc_lib::TracingAlloc;
use color_eyre::eyre::{eyre, Report, Result};

#[global_allocator]
static ALLOC: TracingAlloc = TracingAlloc::new();

fn get_seat_row(input: &str) -> Result<(u16, u16)> {
    let (row, seat) = input.split_at(7);

    let row = row
        .chars()
        .map(|c| match c {
            'F' => Ok(0),
            'B' => Ok(1),
            _ => Err(eyre!("Unexpected character: {}", c)),
        })
        .try_fold(0, |acc, digit| Ok::<_, Report>(acc * 2 + digit?))?;

    let seat = seat
        .chars()
        .map(|c| match c {
            'L' => Ok(0),
            'R' => Ok(1),
            _ => Err(eyre!("Unexpected character: {}", c)),
        })
        .try_fold(0, |acc, digit| Ok::<_, Report>(acc * 2 + digit?))?;

    Ok((row, seat))
}

fn seat_id((row, seat): (u16, u16)) -> u16 {
    row * 8 + seat
}

fn part1(input: &str) -> Result<u16> {
    let mut max = 0;

    let ids = input
        .lines()
        .map(str::trim)
        .map(|l| get_seat_row(l).map(seat_id));

    for id in ids {
        max = max.max(id?);
    }

    Ok(max)
}

fn part2(input: &str) -> Result<u16> {
    let mut seats: Vec<_> = input
        .lines()
        .map(str::trim)
        .map(|l| get_seat_row(l).map(seat_id))
        .collect::<Result<_>>()?;

    seats.sort_unstable();

    seats
        .windows(2)
        .filter(|pair| pair[1] - pair[0] != 1)
        .map(|pair| pair[0] + 1)
        .next()
        .ok_or_else(|| eyre!("Seat not found"))
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let input = aoc_lib::input(2020, 5).open()?;

    aoc_lib::run(&ALLOC, "Day 5: Binary Boarding", &*input, &part1, &part2)
}

#[cfg(test)]
mod tests_2005 {
    use super::*;

    #[test]
    fn decode_test() {
        let input = "FBFBBFFRLR";
        let expected = (44, 5);

        assert_eq!(get_seat_row(input).unwrap(), expected);
    }

    #[test]
    fn part1_example() {
        let tests = [
            ("BFFFBBFRRR", (70, 7), 567),
            ("FFFBBBFRRR", (14, 7), 119),
            ("BBFFBBFRLL", (102, 4), 820),
        ];

        for (i, (test, expected_seat, expected_id)) in tests.iter().enumerate() {
            let seat_row = get_seat_row(test).unwrap();
            assert_eq!(seat_row, *expected_seat, "{}", i);
            assert_eq!(seat_id(seat_row), *expected_id, "{}", i);
        }
    }
}
