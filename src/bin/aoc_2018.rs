use aoc_lib::TracingAlloc;
use color_eyre::eyre::{eyre, Result};
use itertools::Itertools;

#[global_allocator]
static ALLOC: TracingAlloc = TracingAlloc::new();

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

    fn precedence(&self) -> u8 {
        match self {
            Function::Add => 2,
            Function::Multiply => 1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operator {
    Number(u64),
    Function(Function),
    OpenParens,
    CloseParens,
}

impl Operator {
    fn is_parens(&self) -> bool {
        matches!(self, Operator::OpenParens | Operator::CloseParens)
    }

    fn precedence(&self) -> u8 {
        match self {
            Operator::Function(f) => f.precedence(),
            _ => 0,
        }
    }

    fn value(self) -> Result<u64> {
        match self {
            Operator::Number(a) => Ok(a),
            _ => Err(eyre!("Expected Number")),
        }
    }

    fn parse(line: &str, no_prec: bool) -> Result<Vec<Operator>> {
        let mut output = Vec::new();
        let mut op_stack = Vec::new();

        let mut chars = line.char_indices().peekable();
        while let Some((idx, ch)) = chars.next() {
            match ch {
                ')' => {
                    while let Some(op) = op_stack.pop() {
                        if op == Operator::OpenParens {
                            break;
                        }

                        output.push(op);
                    }
                }
                '(' => op_stack.push(Operator::OpenParens),
                '+' => {
                    if matches!(op_stack.last(), Some(op) if !op.is_parens() && (no_prec || Function::Add.precedence() <= op.precedence()))
                    {
                        output.push(op_stack.pop().unwrap());
                    }

                    op_stack.push(Operator::Function(Function::Add));
                }
                '*' => {
                    if matches!(op_stack.last(), Some(op) if !op.is_parens() && (no_prec || Function::Multiply.precedence() <= op.precedence()))
                    {
                        output.push(op_stack.pop().unwrap());
                    }

                    op_stack.push(Operator::Function(Function::Multiply));
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
                _ if ch.is_whitespace() => {}
                _ => return Err(eyre!("Invalid character: {}", ch)),
            }
        }

        op_stack.reverse();
        output.extend(op_stack);

        Ok(output)
    }

    fn evaluate(mut expr: &[Operator]) -> Result<u64> {
        let mut stack = Vec::new();

        loop {
            expr = match expr {
                [] => break,
                [Operator::Number(a), Operator::Number(b), Operator::Function(f), rest @ ..] => {
                    stack.push(Operator::Number(f.apply(*a, *b)));
                    rest
                }
                [Operator::Number(a), rest @ ..] => {
                    stack.push(Operator::Number(*a));
                    rest
                }
                [Operator::Function(f), rest @ ..] => {
                    let (a, b) = stack
                        .pop()
                        .zip(stack.pop())
                        .ok_or_else(|| eyre!("Not enough values on stack"))?;

                    stack.push(Operator::Number(f.apply(a.value()?, b.value()?)));
                    rest
                }

                [Operator::OpenParens, ..] | [Operator::CloseParens, ..] => {
                    return Err(eyre!("Found invalid op"))
                }
            }
        }

        if stack.len() != 1 {
            Err(eyre!("Stack not empty"))
        } else {
            Ok(stack.pop().map(Operator::value).transpose()?.unwrap())
        }
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let input = aoc_lib::input(2020, 18).open()?;

    aoc_lib::run(
        &ALLOC,
        "Day 18: Operation Order",
        &*input,
        &|input| {
            let res = input
                .lines()
                .map(|l| Operator::parse(l, true).and_then(|e| Operator::evaluate(&e)))
                .try_fold(0, |acc, res| -> Result<u64> { Ok(acc + res?) })?;
            Ok(res)
        },
        &|input| {
            let res = input
                .lines()
                .map(|l| Operator::parse(l, false).and_then(|e| Operator::evaluate(&e)))
                .try_fold(0, |acc, res| -> Result<u64> { Ok(acc + res?) })?;
            Ok(res)
        },
    )
}

#[cfg(test)]
mod tests_2018 {
    use aoc_lib::parsers::split_pair;

    use super::*;

    #[test]
    fn parse_test1() {
        let input = "31 + 2";
        let expected = vec![
            Operator::Number(31),
            Operator::Number(2),
            Operator::Function(Function::Add),
        ];
        let actual = Operator::parse(input, true).unwrap();
        assert_eq!(expected, actual);

        let input = "31 + 2 * 5";
        let expected = vec![
            Operator::Number(31),
            Operator::Number(2),
            Operator::Function(Function::Add),
            Operator::Number(5),
            Operator::Function(Function::Multiply),
        ];
        let actual = Operator::parse(input, true).unwrap();
        assert_eq!(expected, actual);

        let input = "3 + (2 * 5)";
        let expected = vec![
            Operator::Number(3),
            Operator::Number(2),
            Operator::Number(5),
            Operator::Function(Function::Multiply),
            Operator::Function(Function::Add),
        ];
        let actual = Operator::parse(input, true).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn part1_example() {
        let input = aoc_lib::input(2020, 18).example(1, 1).open().unwrap();

        for line in input.lines() {
            let (expr, res) = split_pair(line, ";").unwrap();
            let input = Operator::parse(expr, true).unwrap();

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
        let actual = Operator::parse(input, false).unwrap();
        assert_eq!(expected, actual);

        let input = "5 * 2 + 31";
        let expected = vec![
            Operator::Number(5),
            Operator::Number(2),
            Operator::Number(31),
            Operator::Function(Function::Add),
            Operator::Function(Function::Multiply),
        ];
        let actual = Operator::parse(input, false).unwrap();
        assert_eq!(expected, actual);

        let input = "3 + (2 * 5)";
        let expected = vec![
            Operator::Number(3),
            Operator::Number(2),
            Operator::Number(5),
            Operator::Function(Function::Multiply),
            Operator::Function(Function::Add),
        ];
        let actual = Operator::parse(input, false).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn part2_example() {
        let input = aoc_lib::input(2020, 18).example(2, 1).open().unwrap();

        for line in input.lines() {
            let (expr, res) = split_pair(line, ";").unwrap();
            let input = Operator::parse(expr, false).unwrap();

            let expected: u64 = res.parse().unwrap();
            let actual = Operator::evaluate(&input).unwrap();
            assert_eq!(expected, actual);
        }
    }
}
