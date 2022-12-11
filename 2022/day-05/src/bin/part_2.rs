use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, digit1, newline},
    multi::separated_list1,
    sequence::delimited,
    *,
};
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let result = function(input);

    println!("{}", result);
}

fn function(input: String) -> String {
    let (stacks, instructions) = input.split_once("\n\n").unwrap();
    let (_, crates_by_row) = parse_crates_by_row(stacks).unwrap();
    let (_, instructions) = parse_instructions(instructions).unwrap();

    let mut crates_by_column: Vec<Vec<&str>> = transpose(crates_by_row)
        .iter()
        .map(|row| row.iter().rev().filter_map(|c| c.to_owned()).collect())
        .collect();

    execute_instructions(&mut crates_by_column, instructions);

    // collect the last element from each stack into a string
    crates_by_column
        .iter()
        .map(|stack| stack.last().unwrap().to_owned())
        .collect()
}

fn execute_instructions(crates_by_column: &mut [Vec<&str>], instructions: Vec<Instruction>) {
    for instruction in instructions {
        let from_stack = instruction.from_stack - 1;
        let to_stack = instruction.to_stack - 1;
        let number_of_crates = instruction.number_of_crates;
        let stack_height = crates_by_column[from_stack].len();

        let mut crates_to_move = crates_by_column[from_stack]
            .drain((stack_height - number_of_crates)..)
            .collect::<Vec<&str>>();

        crates_by_column[to_stack].append(&mut crates_to_move);
    }
}

// Tranposes a 2D vector aka a vector of vectors
fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    (0..v[0].len())
        .map(|i| v.iter().map(|row| row[i].clone()).collect())
        .collect()
}

fn parse_crates_by_row(input: &str) -> IResult<&str, Vec<Vec<Option<&str>>>> {
    let (input, crates_horizontal) = separated_list1(newline, crate_row)(input)?;

    Ok((input, crates_horizontal))
}
fn crate_row(input: &str) -> IResult<&str, Vec<Option<&str>>> {
    let (input, result) = separated_list1(tag(" "), crate_item)(input)?;

    Ok((input, result))
}
fn crate_item(input: &str) -> IResult<&str, Option<&str>> {
    let (input, c) = alt((
        tag("   "),
        delimited(complete::char('['), alpha1, complete::char(']')),
    ))(input)?;

    let result = match c {
        "   " => None,
        value => Some(value),
    };
    Ok((input, result))
}

#[derive(Debug)]
struct Instruction {
    number_of_crates: usize,
    from_stack: usize,
    to_stack: usize,
}

fn parse_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    let (input, instructions) = separated_list1(newline, parse_instruction)(input)?;

    Ok((input, instructions))
}
fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("move ")(input)?;
    let (input, number_of_crates) = digit1(input)?;
    let (input, _) = tag(" from ")(input)?;
    let (input, from_stack) = digit1(input)?;
    let (input, _) = tag(" to ")(input)?;
    let (input, to_stack) = digit1(input)?;

    let instruction = Instruction {
        number_of_crates: number_of_crates.parse().unwrap(),
        from_stack: from_stack.parse().unwrap(),
        to_stack: to_stack.parse().unwrap(),
    };

    Ok((input, instruction))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function() {
        // read test input file into a string
        let input = fs::read_to_string("test_input.txt").unwrap();

        let result = function(input);

        assert_eq!(result, "MCD".to_owned());
    }
}
