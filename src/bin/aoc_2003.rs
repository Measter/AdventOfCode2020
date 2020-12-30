#![allow(clippy::unnecessary_wraps)]

use aoc_lib::TracingAlloc;
use color_eyre::eyre::{eyre, Result};

#[global_allocator]
static ALLOC: TracingAlloc = TracingAlloc::new();
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Tile {
    Open = 0,
    Tree = 1,
}

impl Tile {
    fn parse(ch: char) -> Result<Tile> {
        match ch {
            '.' => Ok(Tile::Open),
            '#' => Ok(Tile::Tree),
            _ => Err(eyre!("Unknown tile: {}", ch)),
        }
    }
}

struct Map {
    tiles: Vec<Tile>,
    height: usize,
}

impl Map {
    fn parse(input: &str) -> Result<Map> {
        let mut lines = 0;

        let tiles: Vec<_> = input
            .lines()
            .map(str::trim)
            .flat_map(|line| {
                lines += 1;
                line.chars()
            })
            .map(Tile::parse)
            .collect::<Result<_>>()?;

        if tiles.len() % lines != 0 {
            Err(eyre!("Map not square"))
        } else {
            Ok(Map {
                tiles,
                height: lines,
            })
        }
    }

    fn count_trees(&self, delta_x: usize, delta_y: usize) -> Result<u32> {
        let width = self.tiles.len() / self.height;

        Ok((1..)
            .map(|y| y * delta_y)
            .take_while(|&y| y < self.height)
            .zip((1..).map(|x| (x * delta_x) % width))
            .map(|(y, x)| self.tiles[y * width + x] as u32)
            .sum())
    }
}

fn part2(map: &Map) -> Result<u32> {
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .copied()
        .map(|(dx, dy)| map.count_trees(dx, dy))
        .try_fold(1, |acc, item| item.map(|i| acc * i))
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let input = aoc_lib::input(2020, 3).open()?;
    let (map, parse_bench) = aoc_lib::bench(&ALLOC, "Parse", &|| Map::parse(&input))?;
    let (p1_res, p1_bench) = aoc_lib::bench(&ALLOC, "Part 1", &|| map.count_trees(3, 1))?;
    let (p2_res, p2_bench) = aoc_lib::bench(&ALLOC, "Part 2", &|| part2(&map))?;

    aoc_lib::display_results(
        "Day 3: Toboggan Trajectory",
        &[(&"", parse_bench), (&p1_res, p1_bench), (&p2_res, p2_bench)],
    );

    Ok(())
}

#[cfg(test)]
mod tests_2003 {
    use aoc_lib::Example;

    use super::*;

    #[test]
    fn part1_example() {
        let input = aoc_lib::input(2020, 3)
            .example(Example::Part1, 1)
            .open()
            .unwrap();
        let map = Map::parse(&input).unwrap();

        assert_eq!(7, map.count_trees(3, 1).unwrap());
    }

    #[test]
    fn part2_example() {
        let input = aoc_lib::input(2020, 3)
            .example(Example::Part1, 1)
            .open()
            .unwrap();

        let slopes = [
            ((1, 1), 2),
            ((3, 1), 7),
            ((5, 1), 3),
            ((7, 1), 4),
            ((1, 2), 2),
        ];

        let map = Map::parse(&input).unwrap();

        let mut product = 1;

        for (i, ((dx, dy), expected)) in slopes.iter().enumerate() {
            let trees = map.count_trees(*dx, *dy).unwrap();
            product *= trees;

            assert_eq!(trees, *expected, "{}", i);
        }

        assert_eq!(product, 336);
    }
}
