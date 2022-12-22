use std::collections::VecDeque;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, digit1, newline},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair, terminated},
    IResult,
};

use super::monkey::{Monkey, Operation};

pub fn parse_input(input: &str) -> IResult<&str, Vec<Monkey>> {
    let (input, monkeys) = separated_list1(newline, monkey)(input)?;

    Ok((input, monkeys))
}

fn monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, _) = separated_pair(tag("Monkey "), digit1, tag(":\n"))(input)?;
    let (input, items) = items(input)?;
    let (input, operation) = operation(input)?;
    let (input, test) = test(input)?;
    let (input, target_monkey) = target_monkey(input)?;

    Ok((
        input,
        Monkey {
            items: VecDeque::from(items),
            operation,
            test,
            target_monkey,
            times_inspected: 0,
        },
    ))
}

fn items(input: &str) -> IResult<&str, Vec<u64>> {
    delimited(
        tag("  Starting items: "),
        separated_list1(tag(", "), complete::u64),
        newline,
    )(input)
}

fn operation(input: &str) -> IResult<&str, Operation> {
    let (input, operation) = delimited(
        tag("  Operation: new = "),
        alt((
            map(preceded(tag("old + "), complete::u64), Operation::Add),
            map(preceded(tag("old * "), complete::u64), Operation::Multiply),
            map(tag("old * old"), |_| Operation::Square),
        )),
        newline,
    )(input)?;

    Ok((input, operation))
}

fn test(input: &str) -> IResult<&str, u64> {
    let (input, test) = delimited(tag("  Test: divisible by "), complete::u64, newline)(input)?;

    Ok((input, test))
}

fn target_monkey(input: &str) -> IResult<&str, (u64, u64)> {
    let (input, target_monkey) = terminated(
        separated_pair(
            preceded(tag("    If true: throw to monkey "), complete::u64),
            newline,
            preceded(tag("    If false: throw to monkey "), complete::u64),
        ),
        newline,
    )(input)?;

    Ok((input, target_monkey))
}
