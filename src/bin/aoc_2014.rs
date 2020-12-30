use aoc_lib::{parsers::split_pair, TracingAlloc};
use color_eyre::eyre::{eyre, Result};

use std::collections::HashMap;

#[global_allocator]
static ALLOC: TracingAlloc = TracingAlloc::new();

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    UpdateMask { mask: u64, replace: u64 },
    UpdateMem { address: u64, value: u64 },
}

impl Instruction {
    fn parse(line: &str) -> Result<Instruction> {
        let (left, value) = split_pair(line, " = ")?;
        let mut splits = left.splitn(2, '[');

        let instr = splits.next();
        let addr = splits
            .next()
            .map(|addr| addr.trim_end_matches(']'))
            .ok_or_else(|| eyre!("Mem instruction missing address"))
            .and_then(|addr| Ok(addr.parse()?));

        let instr = match (instr, addr) {
            (Some("mask"), _) => {
                let mut mask = 0;
                let mut replace = 0;
                for ch in value.trim().chars() {
                    let (mask_bit, replace_bit) = match ch {
                        'X' => (1, 0),
                        '1' => (0, 1),
                        '0' => (0, 0),
                        _ => return Err(eyre!("Invalid character in mask: {}", ch)),
                    };

                    mask = (mask << 1) | mask_bit;
                    replace = (replace << 1) | replace_bit;
                }

                Instruction::UpdateMask { mask, replace }
            }
            (Some("mem"), addr) => Instruction::UpdateMem {
                address: addr?,
                value: value.parse()?,
            },
            _ => return Err(eyre!("Unknown instruction: {}", line)),
        };

        Ok(instr)
    }
}

fn part1(instructions: &[Instruction]) -> Result<u64> {
    let mut memory = vec![0; u16::MAX as usize];
    let mut mask = u64::MAX;
    let mut replace = 0;

    for &instr in instructions {
        match instr {
            Instruction::UpdateMask {
                mask: new_mask,
                replace: new_replace,
            } => {
                mask = new_mask;
                replace = new_replace;
            }
            Instruction::UpdateMem { address, value } => {
                memory[address as usize] = (value & mask) | replace;
            }
        }
    }

    Ok(memory.iter().sum())
}

fn part2(instructions: &[Instruction]) -> Result<u64> {
    let mut memory = HashMap::new();

    let mut or_mask = 0;
    let mut floating_bits = 0;
    let mut num_addresses = 0;

    for &instr in instructions {
        match instr {
            Instruction::UpdateMask { mask, replace } => {
                or_mask = replace;
                floating_bits = mask;
                num_addresses = 2u64.pow(mask.count_ones());
            }
            Instruction::UpdateMem { mut address, value } => {
                // eprintln!("        Value: {}", value);
                // eprintln!("      Address: {0:036b} {0}", address);
                // eprintln!("Floating Bits: {:036b}", floating_bits);
                // eprintln!("      Or Mask: {:036b}", or_mask);
                address &= !floating_bits;
                address |= or_mask;
                // eprintln!("  Masked Addr: {:036b}", address);

                // eprintln!("    Num Addrs: {}", num_addresses);
                for mut i in 0..num_addresses {
                    let mut replace_bit = 1;
                    let mut address = address;

                    while i != 0 {
                        // eprintln!("  Replace Bit: {:036b}", replace_bit);

                        // Look for the next bit to replace
                        while floating_bits & replace_bit == 0 {
                            replace_bit <<= 1;
                        }

                        let new_bit = if i & 1 != 0 { replace_bit } else { 0 };
                        address |= new_bit;
                        i >>= 1;
                        replace_bit <<= 1
                    }

                    // eprintln!("         Addr: {0:036b} {0}", address);

                    memory.insert(address, value);
                }

                // eprintln!()
            }
        }
    }

    Ok(memory.values().sum())
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let input = aoc_lib::input(2020, 14).open()?;
    let (instructions, parse_bench) = aoc_lib::bench(&ALLOC, "Parse", &|| {
        input
            .lines()
            .map(str::trim)
            .map(Instruction::parse)
            .collect::<Result<Vec<_>>>()
    })?;
    let (p1_res, p1_bench) = aoc_lib::bench(&ALLOC, "Part 1", &|| part1(&instructions))?;
    let (p2_res, p2_bench) = aoc_lib::bench(&ALLOC, "Part 2", &|| part2(&instructions))?;

    aoc_lib::display_results(
        "Day 14: Docking Data",
        &[(&"", parse_bench), (&p1_res, p1_bench), (&p2_res, p2_bench)],
    );

    Ok(())
}

#[cfg(test)]
mod tests_2014 {
    use aoc_lib::Example;

    use super::*;

    #[test]
    fn parse_test() {
        let input = aoc_lib::input(2020, 14)
            .example(Example::Part1, 1)
            .open()
            .unwrap();
        let actual: Vec<_> = input
            .lines()
            .map(str::trim)
            .map(Instruction::parse)
            .collect::<Result<_>>()
            .unwrap();

        let expected = vec![
            Instruction::UpdateMask {
                mask: 0b111111111111111111111111111110111101,
                replace: 0b000000000000000000000000000001000000,
            },
            Instruction::UpdateMem {
                address: 8,
                value: 11,
            },
            Instruction::UpdateMem {
                address: 7,
                value: 101,
            },
            Instruction::UpdateMem {
                address: 8,
                value: 0,
            },
        ];

        assert_eq!(expected, actual);
    }

    #[test]
    fn part1_example() {
        let input = aoc_lib::input(2020, 14)
            .example(Example::Part1, 1)
            .open()
            .unwrap();
        let instructions: Vec<_> = input
            .lines()
            .map(str::trim)
            .map(Instruction::parse)
            .collect::<Result<_>>()
            .unwrap();

        let expected = 165;
        let actual = part1(&instructions).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn part2_example() {
        let input = aoc_lib::input(2020, 14)
            .example(Example::Part2, 1)
            .open()
            .unwrap();
        let instructions: Vec<_> = input
            .lines()
            .map(str::trim)
            .map(Instruction::parse)
            .collect::<Result<_>>()
            .unwrap();

        let expected = 208;
        let actual = part2(&instructions).unwrap();

        assert_eq!(expected, actual);
    }
}
