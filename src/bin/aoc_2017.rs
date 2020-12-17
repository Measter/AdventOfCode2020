use std::{collections::HashSet, ops::RangeInclusive};

use aoc_lib::TracingAlloc;
use color_eyre::eyre::Result;
use itertools::Itertools;

#[global_allocator]
static ALLOC: TracingAlloc = TracingAlloc::new();

type Field = HashSet<(i32, i32, i32, i32)>;

#[derive(Debug, Clone, Copy, Default)]
struct Bounds {
    lower: i32,
    upper: i32,
}

impl Bounds {
    fn expand(&mut self) {
        self.lower -= 1;
        self.upper += 1;
    }

    fn range(self) -> RangeInclusive<i32> {
        self.lower..=self.upper
    }
}

#[derive(Debug, Clone)]
struct GameField {
    state: Field,
    buf: Field,
    x_bounds: Bounds,
    y_bounds: Bounds,
    z_bounds: Bounds,
    w_bounds: Bounds,
}

impl GameField {
    fn parse(input: &str) -> Result<GameField> {
        let mut x_bounds = Bounds::default();
        let mut y_bounds = Bounds::default();

        let mut state = Field::new();

        for (line, y) in input.lines().zip(0..) {
            y_bounds.upper = y;

            for (_, x) in line.chars().zip(0..).filter(|(c, _)| *c == '#') {
                x_bounds.upper = x;
                state.insert((x, y, 0, 0));
            }
        }

        Ok(GameField {
            state,
            buf: Field::new(),
            x_bounds,
            y_bounds,
            z_bounds: Bounds::default(),
            w_bounds: Bounds::default(),
        })
    }

    fn count_neighbours_3d(&self, x: i32, y: i32, z: i32) -> usize {
        let x_range = -1..=1;
        let y_range = -1..=1;
        let z_range = -1..=1;

        x_range
            .cartesian_product(y_range)
            .cartesian_product(z_range)
            .map(|((rx, ry), rz)| (x + rx, y + ry, z + rz))
            .filter(|&(rx, ry, rz)| !(rx == x && ry == y && rz == z))
            .flat_map(|(x, y, z)| self.state.get(&(x, y, z, 0)))
            .count()
    }

    fn step_3d(&mut self) {
        self.buf.clear();

        self.x_bounds.expand();
        self.y_bounds.expand();
        self.z_bounds.expand();

        let coords = self
            .x_bounds
            .range()
            .cartesian_product(self.y_bounds.range())
            .cartesian_product(self.z_bounds.range());

        let mut new_x_bounds = Bounds::default();
        let mut new_y_bounds = Bounds::default();
        let mut new_z_bounds = Bounds::default();

        for ((x, y), z) in coords {
            let neighbours = self.count_neighbours_3d(x, y, z);

            let cube = self.state.contains(&(x, y, z, 0));

            match (cube, neighbours) {
                (true, 2..=3) | (false, 3) => {
                    self.buf.insert((x, y, z, 0));
                    new_x_bounds.lower = new_x_bounds.lower.min(x);
                    new_x_bounds.upper = new_x_bounds.upper.max(x);
                    new_y_bounds.lower = new_y_bounds.lower.min(y);
                    new_y_bounds.upper = new_y_bounds.upper.max(y);
                    new_z_bounds.lower = new_z_bounds.lower.min(z);
                    new_z_bounds.upper = new_z_bounds.upper.max(z);
                }
                _ => {}
            };
        }

        self.x_bounds = new_x_bounds;
        self.y_bounds = new_y_bounds;
        self.z_bounds = new_z_bounds;

        std::mem::swap(&mut self.buf, &mut self.state);
    }

    fn count_neighbours_4d(&self, x: i32, y: i32, z: i32, w: i32) -> usize {
        let x_range = -1..=1;
        let y_range = -1..=1;
        let z_range = -1..=1;
        let w_range = -1..=1;

        x_range
            .cartesian_product(y_range)
            .cartesian_product(z_range)
            .cartesian_product(w_range)
            .map(|(((rx, ry), rz), rw)| (x + rx, y + ry, z + rz, w + rw))
            .filter(|&(rx, ry, rz, rw)| !(rx == x && ry == y && rz == z && rw == w))
            .flat_map(|(x, y, z, w)| self.state.get(&(x, y, z, w)))
            .count()
    }

    fn step_4d(&mut self) {
        self.buf.clear();

        self.x_bounds.expand();
        self.y_bounds.expand();
        self.z_bounds.expand();
        self.w_bounds.expand();

        let coords = self
            .x_bounds
            .range()
            .cartesian_product(self.y_bounds.range())
            .cartesian_product(self.z_bounds.range())
            .cartesian_product(self.w_bounds.range());

        let mut new_x_bounds = Bounds::default();
        let mut new_y_bounds = Bounds::default();
        let mut new_z_bounds = Bounds::default();
        let mut new_w_bounds = Bounds::default();

        for (((x, y), z), w) in coords {
            let neighbours = self.count_neighbours_4d(x, y, z, w);

            let cube = self.state.contains(&(x, y, z, w));

            match (cube, neighbours) {
                (true, 2..=3) | (false, 3) => {
                    self.buf.insert((x, y, z, w));
                    new_x_bounds.lower = new_x_bounds.lower.min(x);
                    new_x_bounds.upper = new_x_bounds.upper.max(x);
                    new_y_bounds.lower = new_y_bounds.lower.min(y);
                    new_y_bounds.upper = new_y_bounds.upper.max(y);
                    new_z_bounds.lower = new_z_bounds.lower.min(z);
                    new_z_bounds.upper = new_z_bounds.upper.max(z);
                    new_w_bounds.lower = new_w_bounds.lower.min(w);
                    new_w_bounds.upper = new_w_bounds.upper.max(w);
                }
                _ => {}
            };
        }

        self.x_bounds = new_x_bounds;
        self.y_bounds = new_y_bounds;
        self.z_bounds = new_z_bounds;
        self.w_bounds = new_w_bounds;

        std::mem::swap(&mut self.buf, &mut self.state);
    }

    fn count_active(&self) -> usize {
        self.state.len()
    }
}
fn main() -> Result<()> {
    color_eyre::install()?;

    let input = aoc_lib::input(2020, 17).open()?;
    let state = GameField::parse(&input).unwrap();

    aoc_lib::run(
        &ALLOC,
        "Day 17: Conway Cubes",
        &state,
        &|state| {
            let mut state = state.clone();
            for _ in 0..6 {
                state.step_3d();
            }

            Ok(state.count_active())
        },
        &|state| {
            let mut state = state.clone();
            for _ in 0..6 {
                state.step_4d();
            }

            Ok(state.count_active())
        },
    )
}

#[cfg(test)]
mod tests_2017 {
    use super::*;

    #[test]
    fn part1_example() {
        let input = aoc_lib::input(2020, 17).example(1, 1).open().unwrap();
        let mut state = GameField::parse(&input).unwrap();

        for _ in 0..6 {
            state.step_3d();
        }

        let expected = 112;
        let actual = state.count_active();

        assert_eq!(expected, actual);
    }

    #[test]
    fn part2_example() {
        let input = aoc_lib::input(2020, 17).example(1, 1).open().unwrap();
        let mut state = GameField::parse(&input).unwrap();

        for _ in 0..6 {
            state.step_4d();
        }

        let expected = 848;
        let actual = state.count_active();

        assert_eq!(expected, actual);
    }
}
