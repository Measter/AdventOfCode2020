use aoc_lib::{Bench, BenchResult, Day, ParseResult, UserError};
use color_eyre::{
    eyre::{eyre, Result},
    Report,
};
use itertools::Itertools;

pub const DAY: Day = Day {
    day: 18,
    name: "Operation Order",
    part_1: run_part1,
    part_2: Some(run_part2),
    other: &[("Parse", run_parse)],
};

fn run_part1(input: &str, b: Bench) -> BenchResult {
    let input: Vec<_> = input
        .lines()
        .map(|l| Operator::parse(l, &part1_precedence))
        .collect::<Result<_, _>>()
        .map_err(UserError)?;

    b.bench(|| {
        input
            .iter()
            .map(|e| Operator::evaluate(e))
            .try_fold(0, |acc, res| Ok::<_, Report>(acc + res?))
            .map_err(UserError)
    })
}

fn run_part2(input: &str, b: Bench) -> BenchResult {
    let input: Vec<_> = input
        .lines()
        .map(|l| Operator::parse(l, &part2_precedence))
        .collect::<Result<_, _>>()
        .map_err(UserError)?;

    b.bench(|| {
        input
            .iter()
            .map(|e| Operator::evaluate(e))
            .try_fold(0, |acc, res| Ok::<_, Report>(acc + res?))
            .map_err(UserError)
    })
}

fn run_parse(input: &str, b: Bench) -> BenchResult {
    b.bench(|| {
        let data: Vec<_> = input
            .lines()
            .map(|l| Operator::parse(l, &part2_precedence))
            .collect::<Result<_, _>>()?;
        Ok::<_, Report>(ParseResult(data))
    })
}

// A weird one, but simplifies some code later.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Function {
    Add,
    Multiply,
}

impl Function {
    fn apply(self, a: u64, b: u64) -> u64 {
        match self {
            Function::Add => a + b,
            Function::Multiply => a * b,
        }
    }
}

fn part1_precedence(_: char) -> u8 {
    0
}

fn part2_precedence(ch: char) -> u8 {
    match ch {
        '+' => 2,
        '*' => 1,
        _ => 0,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operator {
    Number(u64),
    Function(Function),
}

impl Operator {
    fn parse(line: &str, precedence: &dyn Fn(char) -> u8) -> Result<Vec<Operator>> {
        let mut output = Vec::new();
        let mut op_stack = Vec::new();

        let mut chars = line
            .char_indices()
            .filter(|&(_, c)| !c.is_whitespace())
            .peekable();
        while let Some((idx, ch)) = chars.next() {
            match ch {
                ')' => {
                    while let Some(op) = op_stack.pop() {
                        match op {
                            '(' => break,
                            '+' => output.push(Operator::Function(Function::Add)),
                            '*' => output.push(Operator::Function(Function::Multiply)),
                            _ => unreachable!(),
                        }
                    }
                }
                '(' => op_stack.push(ch),
                '+' | '*' => {
                    if matches!(op_stack.last(), Some(op) if *op != '(' && precedence(ch) <= precedence(*op))
                    {
                        match op_stack.pop().unwrap() {
                            '+' => output.push(Operator::Function(Function::Add)),
                            '*' => output.push(Operator::Function(Function::Multiply)),
                            _ => unreachable!(),
                        }
                    }

                    op_stack.push(ch);
                }
                _ if ch.is_ascii_digit() => {
                    let last_idx = chars
                        .peeking_take_while(|(_, c)| c.is_ascii_digit())
                        .last()
                        .map(|(idx, _)| idx)
                        .unwrap_or(idx);

                    let number = line[idx..last_idx + 1].parse().unwrap();
                    output.push(Operator::Number(number));
                }
                _ => return Err(eyre!("Invalid character: {}", ch)),
            }
        }

        op_stack.reverse();
        output.extend(op_stack.into_iter().map(|m| match m {
            '+' => Operator::Function(Function::Add),
            '*' => Operator::Function(Function::Multiply),
            _ => unreachable!(),
        }));

        Ok(output)
    }

    fn evaluate(mut expr: &[Operator]) -> Result<u64> {
        let mut stack = Vec::new();

        loop {
            expr = match expr {
                [] => break,
                [Operator::Number(a), Operator::Number(b), Operator::Function(f), rest @ ..] => {
                    stack.push(f.apply(*a, *b));
                    rest
                }
                [Operator::Number(a), rest @ ..] => {
                    stack.push(*a);
                    rest
                }
                [Operator::Function(f), rest @ ..] => {
                    let (a, b) = stack
                        .pop()
                        .zip(stack.pop())
                        .ok_or_else(|| eyre!("Not enough values on stack"))?;

                    stack.push(f.apply(a, b));
                    rest
                }
            }
        }

        if stack.len() != 1 {
            Err(eyre!("Stack not empty"))
        } else {
            Ok(stack.pop().unwrap())
        }
    }
}

#[cfg(test)]
mod tests_2018 {
    use aoc_lib::{parsers::split_pair, Example};

    use super::*;

    #[test]
    fn parse_test1() {
        let input = "31 + 2";
        let expected = vec![
            Operator::Number(31),
            Operator::Number(2),
            Operator::Function(Function::Add),
        ];
        let actual = Operator::parse(input, &part1_precedence).unwrap();
        assert_eq!(expected, actual);

        let input = "31 + 2 * 5";
        let expected = vec![
            Operator::Number(31),
            Operator::Number(2),
            Operator::Function(Function::Add),
            Operator::Number(5),
            Operator::Function(Function::Multiply),
        ];
        let actual = Operator::parse(input, &part1_precedence).unwrap();
        assert_eq!(expected, actual);

        let input = "3 + (2 * 5)";
        let expected = vec![
            Operator::Number(3),
            Operator::Number(2),
            Operator::Number(5),
            Operator::Function(Function::Multiply),
            Operator::Function(Function::Add),
        ];
        let actual = Operator::parse(input, &part1_precedence).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn part1_example() {
        let input = aoc_lib::input(18)
            .example(Example::Part1, 1)
            .open()
            .unwrap();

        for line in input.lines() {
            let (expr, res) = split_pair(line, ";").unwrap();
            let input = Operator::parse(expr, &part1_precedence).unwrap();

            let expected: u64 = res.parse().unwrap();
            let actual = Operator::evaluate(&input).unwrap();
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn parse_test2() {
        let input = "31 + 2";
        let expected = vec![
            Operator::Number(31),
            Operator::Number(2),
            Operator::Function(Function::Add),
        ];
        let actual = Operator::parse(input, &part2_precedence).unwrap();
        assert_eq!(expected, actual);

        let input = "5 * 2 + 31";
        let expected = vec![
            Operator::Number(5),
            Operator::Number(2),
            Operator::Number(31),
            Operator::Function(Function::Add),
            Operator::Function(Function::Multiply),
        ];
        let actual = Operator::parse(input, &part2_precedence).unwrap();
        assert_eq!(expected, actual);

        let input = "3 + (2 * 5)";
        let expected = vec![
            Operator::Number(3),
            Operator::Number(2),
            Operator::Number(5),
            Operator::Function(Function::Multiply),
            Operator::Function(Function::Add),
        ];
        let actual = Operator::parse(input, &part2_precedence).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn part2_example() {
        let input = aoc_lib::input(18)
            .example(Example::Part2, 1)
            .open()
            .unwrap();

        for line in input.lines() {
            let (expr, res) = split_pair(line, ";").unwrap();
            let input = Operator::parse(expr, &part2_precedence).unwrap();

            let expected: u64 = res.parse().unwrap();
            let actual = Operator::evaluate(&input).unwrap();
            assert_eq!(expected, actual);
        }
    }
}
