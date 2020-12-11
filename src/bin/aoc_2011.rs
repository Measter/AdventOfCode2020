use aoc_lib::TracingAlloc;
use color_eyre::eyre::{eyre, Result};

#[global_allocator]
static ALLOC: TracingAlloc = TracingAlloc::new();

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

#[derive(Debug)]
struct WaitingArea {
    floor_space: Vec<Tile>,
    buf: Vec<Tile>,
    width: usize,
}

impl WaitingArea {
    fn parse(input: &str) -> Result<WaitingArea> {
        let width = input
            .find(['\n', '\r'].as_ref())
            .unwrap_or_else(|| input.len());

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
                buf: vec![Tile::Floor; floor_space.len()],
                floor_space,
                width,
            })
        }
    }

    fn get_neighbours(x: usize, y: usize, size: usize) -> [Option<usize>; 8] {
        let neighbour = |rel_x: isize, rel_y: isize| -> Option<usize> {
            let size = size as isize;
            let new_x = (x as isize) + rel_x;
            let new_y = (y as isize) + rel_y;

            if new_x < 0 || new_x >= size || new_y < 0 || new_y >= size {
                None
            } else {
                Some((new_y * size + new_x) as usize)
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
    }

    fn step(&mut self) {
        let buffer = &mut self.buf;
        let floor_space = &self.floor_space;

        let rows = floor_space
            .chunks_exact(self.width)
            .zip(buffer.chunks_exact_mut(self.width))
            .enumerate();

        for (y, (src_tiles, dst_tiles)) in rows {
            let tiles = src_tiles.iter().zip(dst_tiles).enumerate();

            for (x, (src_tile, dst_tile)) in tiles {
                let neighbours = WaitingArea::get_neighbours(x, y, self.width);

                let filled_count = neighbours
                    .iter()
                    .filter_map(|n| *n)
                    .filter(|n| floor_space[*n] == Tile::Occupied)
                    .count();

                *dst_tile = match (src_tile, filled_count) {
                    (Tile::Empty, 0) => Tile::Occupied,
                    (Tile::Occupied, 4..=usize::MAX) => Tile::Empty,
                    _ => *src_tile,
                };
            }
        }

        std::mem::swap(&mut self.floor_space, &mut self.buf);
    }

    fn run(&mut self) {
        while self.floor_space != self.buf {
            self.step()
        }
    }

    fn occupied_seats(&self) -> usize {
        self.floor_space
            .iter()
            .filter(|t| **t == Tile::Occupied)
            .count()
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let input = aoc_lib::input(2020, 11).open()?;

    aoc_lib::run(
        &ALLOC,
        "Day 11: Seating System",
        &*input,
        &|input| {
            let mut floor = WaitingArea::parse(input)?;
            floor.run();
            Ok(floor.occupied_seats())
        },
        &|_| Ok("Not Implemented"),
    )
}

#[cfg(test)]
mod tests_2011 {
    use super::*;

    #[test]
    fn step_test() {
        let start_input = aoc_lib::input(2020, 11).example(1, 1).open().unwrap();
        let end_input = aoc_lib::input(2020, 11).example(1, 2).open().unwrap();

        let mut floor = WaitingArea::parse(&start_input).unwrap();
        let WaitingArea {
            floor_space: expected,
            ..
        } = WaitingArea::parse(&end_input).unwrap();

        floor.step();
        floor.step();

        assert_eq!(floor.floor_space, expected);
    }

    #[test]
    fn part1_example_full_run() {
        let input = aoc_lib::input(2020, 11).example(1, 1).open().unwrap();
        let end_input = aoc_lib::input(2020, 11).example(1, 3).open().unwrap();

        let mut floor = WaitingArea::parse(&input).unwrap();
        let WaitingArea {
            floor_space: expected_tiles,
            ..
        } = WaitingArea::parse(&end_input).unwrap();

        floor.run();

        let expected_count = 37;
        let actual = floor.occupied_seats();
        assert_eq!(expected_count, actual);
        assert_eq!(expected_tiles, floor.floor_space);
    }
}
