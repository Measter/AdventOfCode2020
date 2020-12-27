use std::{
    collections::{HashSet, VecDeque},
    num::ParseIntError,
};

use aoc_lib::{parsers::split_pair, TracingAlloc};
use color_eyre::eyre::Result;

#[global_allocator]
static ALLOC: TracingAlloc = TracingAlloc::new();

#[derive(Debug, Clone, Copy, PartialEq)]
enum Winner {
    Player1,
    Player2,
}

fn parse_input(input: &str) -> Result<(VecDeque<u32>, VecDeque<u32>)> {
    let (player1_input, player2_input) = split_pair(
        input.trim_start().trim_start_matches("Player 1:"),
        "Player 2:",
    )?;

    let player1_deck = player1_input
        .trim()
        .lines()
        .map(str::trim)
        .map(str::parse)
        .collect::<Result<_, ParseIntError>>()?;

    let player2_deck = player2_input
        .trim()
        .lines()
        .map(str::trim)
        .map(str::parse)
        .collect::<Result<_, ParseIntError>>()?;

    Ok((player1_deck, player2_deck))
}

fn play_part1(mut p1_deck: VecDeque<u32>, mut p2_deck: VecDeque<u32>) -> Result<u32> {
    let mut counter: u32 = 100_000;

    let mut winner = loop {
        counter = counter.saturating_sub(1);
        if p1_deck.is_empty() {
            break p2_deck;
        } else if p2_deck.is_empty() {
            break p1_deck;
        } else if counter == 0 {
            panic!("Maybe infinite loop?")
        }

        let p1_card = p1_deck.pop_front().unwrap();
        let p2_card = p2_deck.pop_front().unwrap();

        if p1_card > p2_card {
            p1_deck.push_back(p1_card);
            p1_deck.push_back(p2_card);
        } else {
            p2_deck.push_back(p2_card);
            p2_deck.push_back(p1_card);
        }
    };

    let score = winner
        .drain(..)
        .rev()
        .zip(1..)
        .map(|(mul, card)| mul * card)
        .sum();

    Ok(score)
}

fn play_part2(mut p1_deck: VecDeque<u32>, mut p2_deck: VecDeque<u32>) -> Result<(Winner, u32)> {
    let mut seen_decks: HashSet<(u32, u32)> = HashSet::new();
    let mut round: u32 = 1;

    let (winner, mut winner_deck) = loop {
        if seen_decks.insert((
            p1_deck
                .iter()
                .rev()
                .copied()
                .zip(1..)
                .map(|(a, b)| a * b)
                .sum(),
            p2_deck
                .iter()
                .rev()
                .copied()
                .zip(1..)
                .map(|(a, b)| a * b)
                .sum(),
        )) == false
        {
            // This configuration has already been seen.
            break (Winner::Player1, p1_deck);
        }

        round += 1;
        if p1_deck.is_empty() {
            break (Winner::Player2, p2_deck);
        } else if p2_deck.is_empty() {
            break (Winner::Player1, p1_deck);
        } else if round == 20_000 {
            panic!("Maybe infinite loop?")
        }

        let p1_card = p1_deck.pop_front().unwrap();
        let p2_card = p2_deck.pop_front().unwrap();

        let winner = if p1_card as usize <= p1_deck.len() && p2_card as usize <= p2_deck.len() {
            let p1_new_deck = p1_deck.iter().copied().take(p1_card as usize).collect();
            let p2_new_deck = p2_deck.iter().copied().take(p2_card as usize).collect();

            play_part2(p1_new_deck, p2_new_deck)?.0
        } else if p1_card >= p2_card {
            Winner::Player1
        } else {
            Winner::Player2
        };

        if winner == Winner::Player1 {
            p1_deck.push_back(p1_card);
            p1_deck.push_back(p2_card);
        } else {
            p2_deck.push_back(p2_card);
            p2_deck.push_back(p1_card);
        }
    };

    let score = winner_deck
        .drain(..)
        .rev()
        .zip(1..)
        .map(|(mul, card)| mul * card)
        .sum();

    Ok((winner, score))
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let input = aoc_lib::input(2020, 22).open()?;
    let ((p1_deck, p2_deck), parse_bench) =
        aoc_lib::bench(&ALLOC, "Parse", &|| parse_input(&input))?;
    let (p1_res, p1_bench) = aoc_lib::bench(&ALLOC, "Part 1", &|| {
        play_part1(p1_deck.clone(), p2_deck.clone())
    })?;
    let (p2_res, p2_bench) = aoc_lib::bench(&ALLOC, "Part 2", &|| {
        play_part2(p1_deck.clone(), p2_deck.clone())
    })?;

    aoc_lib::display_results(
        "Day 22: Crab Combat",
        &[
            (&"", parse_bench),
            (&p1_res, p1_bench),
            (&p2_res.1, p2_bench),
        ],
    )
}

#[cfg(test)]
mod tests_2022 {
    use aoc_lib::Example;

    use super::*;

    #[test]
    fn part1_example() {
        let input = aoc_lib::input(2020, 22)
            .example(Example::Part1, 1)
            .open()
            .unwrap();
        let (player1, player2) = parse_input(&input).unwrap();

        let expected = 306;
        let actual = play_part1(player1, player2).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn part2_example_infinite_loop_test() {
        let input = aoc_lib::input(2020, 22)
            .example(Example::Part2, 1)
            .open()
            .unwrap();
        let (player1, player2) = parse_input(&input).unwrap();

        // Just checks that we don't go into an infinite loop.
        play_part2(player1, player2).unwrap();
    }

    #[test]
    fn part2_example2() {
        let input = aoc_lib::input(2020, 22)
            .example(Example::Part2, 2)
            .open()
            .unwrap();
        let (player1, player2) = parse_input(&input).unwrap();

        let expected = (Winner::Player2, 291);
        let actual = play_part2(player1, player2).unwrap();

        assert_eq!(expected, actual);
    }
}
