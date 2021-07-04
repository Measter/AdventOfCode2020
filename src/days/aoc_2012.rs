use aoc_lib::{day, Bench, BenchResult, UserError};
use color_eyre::eyre::{eyre, Result, WrapErr};

day! {
    day 12: "Rain Risk"
    1: run_part1
    2: run_part2
}

fn run_part1(input: &str, b: Bench) -> BenchResult {
    let instructions: Vec<_> = input
        .lines()
        .map(str::trim)
        .map(Instruction::parse)
        .collect::<Result<_, _>>()
        .map_err(UserError)?;

    b.bench(|| part1(&instructions))
}

fn run_part2(input: &str, b: Bench) -> BenchResult {
    let instructions: Vec<_> = input
        .lines()
        .map(str::trim)
        .map(Instruction::parse)
        .collect::<Result<_, _>>()
        .map_err(UserError)?;

    b.bench(|| part2(&instructions))
}

#[derive(Debug, Copy, Clone)]
enum Instruction {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Left(i32),
    Right(i32),
    Forward(i32),
    Nop,
}

impl Instruction {
    fn parse(line: &str) -> Result<Instruction> {
        let (idx, ch) = line
            .char_indices()
            .next()
            .ok_or_else(|| eyre!("Invalid instruction: {}", line))?;

        let val = line[idx + ch.len_utf8()..]
            .parse()
            .with_context(|| eyre!("Invalid instruction: {}", line))?;

        use Instruction::*;
        Ok(match ch {
            'N' => North(val),
            'S' => South(val),
            'E' => East(val),
            'W' => West(val),
            'L' => Left(-val),
            'R' => Right(val),
            'F' => Forward(val),
            _ => return Err(eyre!("Invalid instruction: {}", line)),
        })
    }

    fn forward_to_absolute(self, direction: i32) -> Result<Instruction> {
        use Instruction::*;
        Ok(match (self, direction) {
            (Forward(len), 0) => North(len),
            (Forward(len), 90) => East(len),
            (Forward(len), 180) => South(len),
            (Forward(len), 270) => West(len),
            _ => return Err(eyre!("Invalid direction: {}", direction)),
        })
    }
}

fn add_direction(start: i32, rot_val: i32) -> i32 {
    let res = start + rot_val;
    if res >= 360 {
        res - 360
    } else if res < 0 {
        res + 360
    } else {
        res
    }
}

fn part1(instructions: &[Instruction]) -> Result<i32> {
    let instructions = instructions.iter().scan(90, |cur_dir, instr| {
        let next = match instr {
            Instruction::Left(val) | Instruction::Right(val) => {
                *cur_dir = add_direction(*cur_dir, *val);
                Instruction::Nop
            }
            Instruction::Forward(_) => instr.forward_to_absolute(*cur_dir).unwrap(),
            _ => *instr,
        };

        Some(next)
    });

    let mut x = 0;
    let mut y = 0;

    for instr in instructions {
        match instr {
            Instruction::North(val) => y += val,
            Instruction::South(val) => y -= val,
            Instruction::East(val) => x += val,
            Instruction::West(val) => x -= val,
            Instruction::Nop => {}
            _ => return Err(eyre!("Unhandled instruction: {:?}", instr)),
        }
    }

    Ok(x.abs() + y.abs())
}

fn part2(instructions: &[Instruction]) -> Result<i32> {
    let mut waypoint_x = 10;
    let mut waypoint_y = 1;
    let mut ship_x = 0;
    let mut ship_y = 0;

    for instr in instructions {
        match instr {
            Instruction::North(val) => waypoint_y += val,
            Instruction::South(val) => waypoint_y -= val,
            Instruction::East(val) => waypoint_x += val,
            Instruction::West(val) => waypoint_x -= val,
            Instruction::Left(val) => {
                let times = -val / 90;
                for _ in 0..times {
                    std::mem::swap(&mut waypoint_x, &mut waypoint_y);
                    waypoint_x *= -1;
                }
            }
            Instruction::Right(val) => {
                let times = val / 90;
                for _ in 0..times {
                    std::mem::swap(&mut waypoint_x, &mut waypoint_y);
                    waypoint_y *= -1;
                }
            }
            Instruction::Forward(val) => {
                ship_x += waypoint_x * *val;
                ship_y += waypoint_y * *val;
            }
            Instruction::Nop => {}
        }
    }

    Ok(ship_x.abs() + ship_y.abs())
}

#[cfg(test)]
mod tests_2012 {
    use aoc_lib::Example;

    use super::*;

    #[test]
    fn part1_example() {
        let input = aoc_lib::input(2020, 12)
            .example(Example::Part1, 1)
            .open()
            .unwrap();
        let instructions: Vec<_> = input
            .lines()
            .map(str::trim)
            .map(Instruction::parse)
            .collect::<Result<_>>()
            .unwrap();

        let expected = 25;
        let actual = part1(&instructions).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn part2_example() {
        let input = aoc_lib::input(2020, 12)
            .example(Example::Part1, 1)
            .open()
            .unwrap();
        let instructions: Vec<_> = input
            .lines()
            .map(str::trim)
            .map(Instruction::parse)
            .collect::<Result<_>>()
            .unwrap();

        let expected = 286;
        let actual = part2(&instructions).unwrap();

        assert_eq!(expected, actual);
    }
}
