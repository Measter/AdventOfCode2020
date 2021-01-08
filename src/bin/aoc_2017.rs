use std::collections::HashSet;

use aoc_lib::TracingAlloc;
use color_eyre::{eyre::Result, Report};
use itertools::Itertools;

#[global_allocator]
static ALLOC: TracingAlloc = TracingAlloc::new();

fn parse_3d(input: &str) -> Result<HashSet<[i8; 3]>> {
    let mut state = HashSet::new();

    for (line, y) in input.lines().zip(0..) {
        for (_, x) in line.chars().zip(0..).filter(|(c, _)| *c == '#') {
            state.insert([x, y, 0]);
        }
    }

    Ok(state)
}

fn get_neighbours_3d([x, y, z]: [i8; 3]) -> impl Iterator<Item = [i8; 3]> {
    let x_range = -1..=1;
    let y_range = -1..=1;
    let z_range = -1..=1;

    x_range
        .cartesian_product(y_range)
        .cartesian_product(z_range)
        .filter(|&((x, y), z)| [x, y, z] != [0, 0, 0])
        .map(move |((rx, ry), rz)| [x + rx, y + ry, z + rz])
}

fn parse_4d(input: &str) -> Result<HashSet<[i8; 4]>> {
    let mut state = HashSet::new();

    for (line, y) in input.lines().zip(0..) {
        for (_, x) in line.chars().zip(0..).filter(|(c, _)| *c == '#') {
            state.insert([x, y, 0, 0]);
        }
    }

    Ok(state)
}

fn get_neighbours_4d([x, y, z, w]: [i8; 4]) -> impl Iterator<Item = [i8; 4]> {
    let x_range = -1..=1;
    let y_range = -1..=1;
    let z_range = -1..=1;
    let w_range = -1..=1;

    x_range
        .cartesian_product(y_range)
        .cartesian_product(z_range)
        .cartesian_product(w_range)
        .filter(|&(((x, y), z), w)| [x, y, z, w] != [0, 0, 0, 0])
        .map(move |(((rx, ry), rz), rw)| [x + rx, y + ry, z + rz, w + rw])
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
    T: Copy + Eq + std::hash::Hash,
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
fn main() -> Result<()> {
    color_eyre::install()?;

    let input = aoc_lib::input(2020, 17).open()?;
    let (game_3d, parse3d_bench) = aoc_lib::bench::<_, Report>(&ALLOC, "Parse 3D", &|| {
        let state = parse_3d(&input)?;
        Ok(GameField::new(state, get_neighbours_3d))
    })?;
    let (game_4d, parse4d_bench) = aoc_lib::bench::<_, Report>(&ALLOC, "Parse 4D", &|| {
        let state = parse_4d(&input)?;
        Ok(GameField::new(state, get_neighbours_4d))
    })?;

    let (p1_res, p1_bench) = aoc_lib::bench::<_, ()>(&ALLOC, "Part 1", &|| {
        let mut state = game_3d.clone();
        for _ in 0..6 {
            state.step();
        }

        Ok(state.count_active())
    })?;
    let (p2_res, p2_bench) = aoc_lib::bench::<_, ()>(&ALLOC, "Part 2", &|| {
        let mut state = game_4d.clone();
        for _ in 0..6 {
            state.step();
        }

        Ok(state.count_active())
    })?;

    aoc_lib::display_results(
        "Day 17: Conway Cubes",
        &[
            (&"", parse3d_bench),
            (&"", parse4d_bench),
            (&p1_res, p1_bench),
            (&p2_res, p2_bench),
        ],
    );

    Ok(())
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
        let state = parse_3d(&input).unwrap();
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
        let state = parse_4d(&input).unwrap();
        let mut game = GameField::new(state, get_neighbours_4d);

        for _ in 0..6 {
            game.step();
        }

        let expected = 848;
        let actual = game.count_active();

        assert_eq!(expected, actual);
    }
}
