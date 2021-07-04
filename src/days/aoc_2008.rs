use std::collections::HashSet;

use aoc_lib::{day, parsers::split_pair, Bench, BenchResult, UserError};
use color_eyre::eyre::{eyre, Result};

day! {
    day 8: "Handheld Halting"
    1: run_part1
    2: run_part2
}

fn run_part1(input: &str, b: Bench) -> BenchResult {
    let instrs = Instruction::parse(input).map_err(UserError)?;
    b.bench(|| {
        let mut computer = Computer::default();
        let mut seen_pc = HashSet::new();
        part1(&instrs, &mut computer, &mut seen_pc)
    })
}

fn run_part2(input: &str, b: Bench) -> BenchResult {
    let instrs = Instruction::parse(input).map_err(UserError)?;
    b.bench(|| part2(&instrs))
}

const VALID_VAL_START: &[char] = &['-', '+'];

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Instruction {
    Acc(i64),
    Jmp(isize),
    Nop(isize),
}

impl Instruction {
    fn parse(program: &str) -> Result<Vec<Instruction>> {
        let mut instrs = Vec::new();

        for line in program.lines().map(str::trim) {
            let (op, val) = split_pair(line, " ")?;

            let val = if val.starts_with(VALID_VAL_START) {
                let (sign, mag) = val.split_at(1);
                match sign {
                    "-" => val,
                    _ => mag,
                }
            } else {
                return Err(eyre!("Invalid op value: `{}`", line));
            };

            let instr = match op {
                "acc" => Instruction::Acc(val.parse()?),
                "jmp" => Instruction::Jmp(val.parse()?),
                "nop" => Instruction::Nop(val.parse()?),
                _ => return Err(eyre!("Invalid opcode: `{}`", line)),
            };

            instrs.push(instr);
        }

        Ok(instrs)
    }

    fn swap_nop_jmp(&mut self) {
        *self = match *self {
            Instruction::Nop(v) => Instruction::Jmp(v),
            Instruction::Jmp(v) => Instruction::Nop(v),
            a => a,
        };
    }
}

#[derive(Debug, Default)]
struct Computer {
    acc: i64,
    pc: usize,
}

impl Computer {
    fn step(&mut self, program: &[Instruction]) -> Result<()> {
        let next_op = *program
            .get(self.pc)
            .ok_or_else(|| eyre!("Invalid memory address: 0x{:?}", self.pc))?;

        match next_op {
            Instruction::Acc(val) => self.acc += val,
            Instruction::Jmp(val) => {
                self.pc = (self.pc as isize + val) as usize;
                return Ok(());
            }
            Instruction::Nop(_) => {}
        }

        self.pc += 1;

        Ok(())
    }
}

fn part1(
    instrs: &[Instruction],
    computer: &mut Computer,
    seen_pc: &mut HashSet<usize>,
) -> Result<i64> {
    seen_pc.clear();

    while seen_pc.insert(computer.pc) {
        computer.step(&instrs)?;
    }

    Ok(computer.acc)
}

fn part2(instrs: &[Instruction]) -> Result<i64> {
    let mut local_instrs = instrs.to_owned();
    let mut seen_pc = HashSet::new();

    let instrs_to_swap = instrs
        .iter()
        .enumerate()
        .filter(|(_, i)| matches!(i, Instruction::Nop(_) | Instruction::Jmp(_)))
        .map(|(i, _)| i);

    for idx in instrs_to_swap {
        local_instrs[idx].swap_nop_jmp();

        let mut computer = Computer::default();
        match part1(&local_instrs, &mut computer, &mut seen_pc) {
            Ok(_) => {
                // It got into an infinite loop, so this program is wrong.
            }
            Err(_) if computer.pc == local_instrs.len() => return Ok(computer.acc),
            Err(e) => return Err(e),
        }

        local_instrs[idx].swap_nop_jmp();
    }

    Err(eyre!("No instruction swap worked"))
}

#[cfg(test)]
mod tests_2008 {
    use aoc_lib::Example;

    use super::*;

    #[test]
    fn parse_test() {
        use Instruction::*;
        let expected = vec![Acc(-1), Acc(1), Jmp(-1), Jmp(1), Nop(-1), Nop(1)];

        let input = aoc_lib::input(2020, 8)
            .example(Example::Parse, 1)
            .open()
            .unwrap();
        let actual = Instruction::parse(&input).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn part1_example() {
        let input = aoc_lib::input(2020, 8)
            .example(Example::Part1, 1)
            .open()
            .unwrap();
        let instrs = Instruction::parse(&input).unwrap();

        let mut computer = Computer::default();
        let mut seen_pc = HashSet::new();

        let expected = 5;
        let actual = part1(&instrs, &mut computer, &mut seen_pc).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn part2_example() {
        let input = aoc_lib::input(2020, 8)
            .example(Example::Part1, 1)
            .open()
            .unwrap();
        let instrs = Instruction::parse(&input).unwrap();

        let expected = 8;
        let actual = part2(&instrs).unwrap();

        assert_eq!(expected, actual);
    }
}
