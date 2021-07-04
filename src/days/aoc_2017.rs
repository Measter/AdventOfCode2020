use std::{collections::HashSet, hash::Hash, ops::RangeInclusive};

use aoc_lib::{day, Bench, BenchResult, NoError, UserError};
use color_eyre::eyre::Result;
use itertools::iproduct;

day! {
    day 17: "Conway Cubes"
    1: run_part1
    2: run_part2
}

fn run_part1(input: &str, b: Bench) -> BenchResult {
    let state = parse::<[i8; 3]>(&input).map_err(UserError)?;
    let game = GameField::new(state, get_neighbours_3d);

    b.bench(|| {
        let mut state = game.clone();
        for _ in 0..6 {
            state.step();
        }

        Ok::<_, NoError>(state.count_active())
    })
}

fn run_part2(input: &str, b: Bench) -> BenchResult {
    let state = parse::<[i8; 4]>(&input).map_err(UserError)?;
    let game = GameField::new(state, get_neighbours_4d);

    b.bench(|| {
        let mut state = game.clone();
        for _ in 0..6 {
            state.step();
        }

        Ok::<_, NoError>(state.count_active())
    })
}

fn parse<T>(input: &str) -> Result<HashSet<T>>
where
    T: Default + AsMut<[i8]> + Eq + Hash,
{
    let mut state = HashSet::new();

    for (line, y) in input.lines().zip(0..) {
        for (_, x) in line.chars().zip(0..).filter(|(c, _)| *c == '#') {
            let mut cell = T::default();
            let cell_ref = cell.as_mut();
            cell_ref[0] = x;
            cell_ref[1] = y;
            state.insert(cell);
        }
    }

    Ok(state)
}

const NEIGHBOUR_RANGE: RangeInclusive<i8> = -1..=1;

fn get_neighbours_3d(cell: [i8; 3]) -> impl Iterator<Item = [i8; 3]> {
    iproduct!(NEIGHBOUR_RANGE, NEIGHBOUR_RANGE, NEIGHBOUR_RANGE)
        .filter(|&coords| coords != Default::default())
        .map(move |nc| [nc.0 + cell[0], nc.1 + cell[1], nc.2 + cell[2]])
}

fn get_neighbours_4d(cell: [i8; 4]) -> impl Iterator<Item = [i8; 4]> {
    iproduct!(
        NEIGHBOUR_RANGE,
        NEIGHBOUR_RANGE,
        NEIGHBOUR_RANGE,
        NEIGHBOUR_RANGE
    )
    .filter(|&coords| coords != Default::default())
    .map(move |nc| {
        [
            nc.0 + cell[0],
            nc.1 + cell[1],
            nc.2 + cell[2],
            nc.3 + cell[3],
        ]
    })
}

#[derive(Debug, Clone)]
struct GameField<T, F> {
    state: HashSet<T>,
    buf: HashSet<T>,
    check_buf: HashSet<T>,
    neighbour_func: F,
}

impl<T, F, I> GameField<T, F>
where
    T: Copy + Eq + Hash,
    F: Fn(T) -> I,
    I: Iterator<Item = T>,
{
    fn new(state: HashSet<T>, neighbour_func: F) -> Self {
        Self {
            state,
            buf: HashSet::new(),
            check_buf: HashSet::new(),
            neighbour_func,
        }
    }

    fn step(&mut self) {
        // So we don't have multiple mutable borrows through self.
        let Self {
            buf,
            check_buf,
            state,
            ..
        } = self;

        buf.clear();
        check_buf.clear();
        check_buf.extend(&*state);

        for node in &*state {
            check_buf.extend((self.neighbour_func)(*node));
        }

        for &node in &*check_buf {
            let living_neighours = (self.neighbour_func)(node)
                .filter(|neighbour| state.contains(neighbour))
                .take(4)
                .count();

            let alive = state.contains(&node);

            if matches!((alive, living_neighours), (true, 2..=3) | (false, 3)) {
                buf.insert(node);
            }
        }

        std::mem::swap(&mut self.buf, &mut self.state);
    }

    fn count_active(&self) -> usize {
        self.state.len()
    }
}

#[cfg(test)]
mod tests_2017 {
    use aoc_lib::Example;

    use super::*;

    #[test]
    fn part1_example() {
        let input = aoc_lib::input(2020, 17)
            .example(Example::Part1, 1)
            .open()
            .unwrap();
        let state = parse::<[i8; 3]>(&input).unwrap();
        let mut game = GameField::new(state, get_neighbours_3d);

        for _ in 0..6 {
            game.step();
        }

        let expected = 112;
        let actual = game.count_active();

        assert_eq!(expected, actual);
    }

    #[test]
    fn part2_example() {
        let input = aoc_lib::input(2020, 17)
            .example(Example::Part1, 1)
            .open()
            .unwrap();
        let state = parse::<[i8; 4]>(&input).unwrap();
        let mut game = GameField::new(state, get_neighbours_4d);

        for _ in 0..6 {
            game.step();
        }

        let expected = 848;
        let actual = game.count_active();

        assert_eq!(expected, actual);
    }
}
