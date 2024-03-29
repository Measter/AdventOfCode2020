use aoc_lib::{Bench, BenchResult, Day, NoError, ParseResult, UserError};
use color_eyre::{
    eyre::{eyre, Result},
    Report,
};

pub const DAY: Day = Day {
    day: 11,
    name: "Seating System",
    part_1: run_part1,
    part_2: Some(run_part2),
    other: &[("Parse", run_parse)],
};

fn run_part1(input: &str, b: Bench) -> BenchResult {
    let floor = WaitingArea::parse(input).map_err(UserError)?;

    b.bench(|| {
        let mut floor = floor.clone();
        floor.run(WaitingArea::count_neighbours_part1, 4);
        Ok::<_, NoError>(floor.occupied_seats())
    })
}

fn run_part2(input: &str, b: Bench) -> BenchResult {
    let floor = WaitingArea::parse(input).map_err(UserError)?;

    b.bench(|| {
        let mut floor = floor.clone();
        floor.run(WaitingArea::count_neighbours_part2, 5);
        Ok::<_, NoError>(floor.occupied_seats())
    })
}

fn run_parse(input: &str, b: Bench) -> BenchResult {
    b.bench(|| {
        let data = WaitingArea::parse(input)?;
        Ok::<_, Report>(ParseResult(data))
    })
}

type NeighbourFunc = fn(&[Tile], usize, usize, usize, usize) -> usize;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Tile {
    Floor,
    Empty,
    Occupied,
}

impl Tile {
    fn parse(ch: char) -> Result<Tile> {
        match ch {
            '.' => Ok(Tile::Floor),
            'L' => Ok(Tile::Empty),
            '#' => Ok(Tile::Occupied),
            _ => Err(eyre!("Unknown character: {}", ch)),
        }
    }
}

#[derive(Debug, Clone)]
struct WaitingArea {
    floor_space: Vec<Tile>,
    buf: Vec<Tile>,
    width: usize,
    height: usize,
}

impl WaitingArea {
    fn parse(input: &str) -> Result<WaitingArea> {
        let width = input.find(['\n', '\r'].as_ref()).unwrap_or(input.len());

        let floor_space: Vec<_> = input
            .lines()
            .map(str::trim)
            .flat_map(str::chars)
            .map(Tile::parse)
            .collect::<Result<_>>()?;

        if floor_space.len() % width != 0 {
            Err(eyre!("Input must be a rectangular grid"))
        } else {
            Ok(WaitingArea {
                height: floor_space.len() / width,
                buf: vec![Tile::Floor; floor_space.len()],
                floor_space,
                width,
            })
        }
    }

    fn count_neighbours_part1(
        floor: &[Tile],
        x: usize,
        y: usize,
        width: usize,
        height: usize,
    ) -> usize {
        let width = width as isize;
        let height = height as isize;

        let neighbour = |rel_x: isize, rel_y: isize| -> bool {
            let new_x = (x as isize) + rel_x;
            let new_y = (y as isize) + rel_y;

            let idx = (new_y * width + new_x) as usize;
            (0..width).contains(&new_x)
                && (0..height).contains(&new_y)
                && floor[idx] == Tile::Occupied
        };

        [
            neighbour(-1, -1),
            neighbour(0, -1),
            neighbour(1, -1),
            neighbour(-1, 0),
            neighbour(1, 0),
            neighbour(-1, 1),
            neighbour(0, 1),
            neighbour(1, 1),
        ]
        .iter()
        .filter(|b| **b)
        .count()
    }

    fn count_neighbours_part2(
        floor: &[Tile],
        x: usize,
        y: usize,
        width: usize,
        height: usize,
    ) -> usize {
        let width = width as isize;
        let height = height as isize;

        let neighbour = |rel_x: isize, rel_y: isize| -> bool {
            let mut new_x = (x as isize) + rel_x;
            let mut new_y = (y as isize) + rel_y;

            loop {
                let idx = (new_y * width + new_x) as usize;
                if !(0..width).contains(&new_x) || !(0..height).contains(&new_y) {
                    return false;
                } else if floor[idx] != Tile::Floor {
                    return floor[idx] == Tile::Occupied;
                }

                new_x += rel_x;
                new_y += rel_y;
            }
        };

        [
            neighbour(-1, -1),
            neighbour(0, -1),
            neighbour(1, -1),
            neighbour(-1, 0),
            neighbour(1, 0),
            neighbour(-1, 1),
            neighbour(0, 1),
            neighbour(1, 1),
        ]
        .iter()
        .filter(|b| **b)
        .count()
    }

    fn step(&mut self, neighbour_fn: NeighbourFunc, max_filled: usize) {
        let buffer = &mut self.buf;
        let floor_space = &self.floor_space;

        let rows = floor_space
            .chunks_exact(self.width)
            .zip(buffer.chunks_exact_mut(self.width))
            .enumerate();

        for (y, (src_tiles, dst_tiles)) in rows {
            let tiles = src_tiles
                .iter()
                .zip(dst_tiles)
                .enumerate()
                .filter(|(_, (t, _))| **t != Tile::Floor);

            for (x, (src_tile, dst_tile)) in tiles {
                let filled_count = neighbour_fn(floor_space, x, y, self.width, self.height);

                *dst_tile = match (src_tile, filled_count) {
                    (Tile::Empty, 0) => Tile::Occupied,
                    (Tile::Occupied, _) if filled_count >= max_filled => Tile::Empty,
                    _ => *src_tile,
                };
            }
        }

        std::mem::swap(&mut self.floor_space, &mut self.buf);
    }

    fn run(&mut self, neighbour_fn: NeighbourFunc, max_seats: usize) {
        while self.floor_space != self.buf {
            self.step(neighbour_fn, max_seats);
        }
    }

    fn occupied_seats(&self) -> usize {
        self.floor_space
            .iter()
            .filter(|t| **t == Tile::Occupied)
            .count()
    }
}

#[cfg(test)]
mod tests_2011 {
    use aoc_lib::Example;

    use super::*;

    #[test]
    fn step_test() {
        let start_input = aoc_lib::input(11)
            .example(Example::Part1, 1)
            .open()
            .unwrap();
        let end_input = aoc_lib::input(11)
            .example(Example::Part1, 2)
            .open()
            .unwrap();

        let mut floor = WaitingArea::parse(&start_input).unwrap();
        let WaitingArea {
            floor_space: expected,
            ..
        } = WaitingArea::parse(&end_input).unwrap();

        floor.step(WaitingArea::count_neighbours_part1, 4);
        floor.step(WaitingArea::count_neighbours_part1, 4);

        assert_eq!(floor.floor_space, expected);
    }

    #[test]
    fn part1_example_full_run() {
        let input = aoc_lib::input(11)
            .example(Example::Part1, 1)
            .open()
            .unwrap();
        let end_input = aoc_lib::input(11)
            .example(Example::Part1, 3)
            .open()
            .unwrap();

        let mut floor = WaitingArea::parse(&input).unwrap();
        let WaitingArea {
            floor_space: expected_tiles,
            ..
        } = WaitingArea::parse(&end_input).unwrap();

        floor.run(WaitingArea::count_neighbours_part1, 4);

        let expected_count = 37;
        let actual = floor.occupied_seats();
        assert_eq!(expected_count, actual);
        assert_eq!(expected_tiles, floor.floor_space);
    }

    #[test]
    fn part2_example_full_run() {
        let input = aoc_lib::input(11)
            .example(Example::Part1, 1)
            .open()
            .unwrap();
        let end_input = aoc_lib::input(11)
            .example(Example::Part2, 1)
            .open()
            .unwrap();

        let mut floor = WaitingArea::parse(&input).unwrap();
        let WaitingArea {
            floor_space: expected_tiles,
            ..
        } = WaitingArea::parse(&end_input).unwrap();

        floor.run(WaitingArea::count_neighbours_part2, 5);

        let expected_count = 26;
        let actual = floor.occupied_seats();
        assert_eq!(expected_count, actual);
        assert_eq!(expected_tiles, floor.floor_space);
    }
}
