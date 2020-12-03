use color_eyre::eyre::{eyre, Result};

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

    fn count_trees(&self, delta_x: usize, delta_y: usize) -> u32 {
        let mut x = delta_x;
        let mut y = delta_y;
        let width = self.tiles.len() / self.height;

        let mut trees = 0;

        while y < self.height {
            trees += self.tiles[y * width + x] as u32;

            y += delta_y;
            x += delta_x;
            if x >= width {
                x -= width;
            }
        }

        trees
    }
}

fn part2(map: &Map) -> u32 {
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .copied()
        .map(|(dx, dy)| map.count_trees(dx, dy))
        .product()
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let input = std::fs::read_to_string("inputs/aoc_2003.txt")?;
    let map = Map::parse(&input)?;

    let start = std::time::Instant::now();

    let part1 = map.count_trees(3, 1);
    let part2 = part2(&map);

    let elapsed = start.elapsed();

    println!("Part 1 output: {}", part1);
    println!("Part 2 output: {}", part2);

    println!("Elapsed: {}us", elapsed.as_micros());

    Ok(())
}

#[cfg(test)]
mod tests_2003 {
    use super::*;

    #[test]
    fn part1_example() {
        let input = "..##.......
        #...#...#..
        .#....#..#.
        ..#.#...#.#
        .#...##..#.
        ..#.##.....
        .#.#.#....#
        .#........#
        #.##...#...
        #...##....#
        .#..#...#.#";

        let map = Map::parse(input).unwrap();

        assert_eq!(7, map.count_trees(3, 1));
    }

    #[test]
    fn part2_example() {
        let input = "..##.......
        #...#...#..
        .#....#..#.
        ..#.#...#.#
        .#...##..#.
        ..#.##.....
        .#.#.#....#
        .#........#
        #.##...#...
        #...##....#
        .#..#...#.#";

        let slopes = [
            ((1, 1), 2),
            ((3, 1), 7),
            ((5, 1), 3),
            ((7, 1), 4),
            ((1, 2), 2),
        ];

        let map = Map::parse(input).unwrap();

        let mut product = 1;

        for (i, ((dx, dy), expected)) in slopes.iter().enumerate() {
            let trees = map.count_trees(*dx, *dy);
            product *= trees;

            assert_eq!(trees, *expected, "{}", i);
        }

        assert_eq!(product, 336);
    }
}
