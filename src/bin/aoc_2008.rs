use std::collections::HashSet;

use aoc_lib::{parsers::split_pair, TracingAlloc};
use color_eyre::eyre::{eyre, Result};

#[global_allocator]
static ALLOC: TracingAlloc = TracingAlloc::new();

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

fn main() -> Result<()> {
    color_eyre::install()?;

    let input = std::fs::read_to_string("inputs/aoc_2008.txt")?;
    let instrs: Vec<_> = Instruction::parse(&input)?;

    aoc_lib::run(
        &ALLOC,
        "Day 8: Handheld Halting",
        &*instrs,
        &|i| {
            let mut computer = Computer::default();
            let mut seen_pc = HashSet::new();
            part1(i, &mut computer, &mut seen_pc)
        },
        &part2,
    )
}

#[cfg(test)]
mod tests_2008 {
    use super::*;

    #[test]
    fn parse_test() {
        let input = "acc -1
        acc +1
        jmp -1
        jmp +1
        nop -1
        nop +1";

        use Instruction::*;
        let expected = vec![Acc(-1), Acc(1), Jmp(-1), Jmp(1), Nop(-1), Nop(1)];

        let actual = Instruction::parse(&input).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn part1_example() {
        let input = "nop +0
        acc +1
        jmp +4
        acc +3
        jmp -3
        acc -99
        acc +1
        jmp -4
        acc +6";

        let instrs = Instruction::parse(&input).unwrap();
        let mut computer = Computer::default();
        let mut seen_pc = HashSet::new();

        let expected = 5;
        let actual = part1(&instrs, &mut computer, &mut seen_pc).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn part2_example() {
        let input = "nop +0
        acc +1
        jmp +4
        acc +3
        jmp -3
        acc -99
        acc +1
        jmp -4
        acc +6";

        let instrs = Instruction::parse(&input).unwrap();

        let expected = 8;
        let actual = part2(&instrs).unwrap();

        assert_eq!(expected, actual);
    }
}
