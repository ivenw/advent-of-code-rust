use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    sequence::preceded,
    IResult,
};

#[derive(Debug)]
pub enum Instruction {
    Noop,
    Addx(i32),
}

impl Instruction {
    pub fn cycles(&self) -> i32 {
        match self {
            Instruction::Noop => 1,
            Instruction::Addx(_) => 2,
        }
    }

    pub fn addx(&self, register: &mut i32) {
        if let Instruction::Addx(value) = self {
            *register += *value
        }
    }
}

pub fn parse_input(input: &str) -> IResult<&str, Vec<Instruction>> {
    let (input, instructions) = separated_list1(newline, instruction)(input)?;

    Ok((input, instructions))
}

fn instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, instruction) = alt((noop, addx))(input)?;

    Ok((input, instruction))
}

fn noop(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("noop")(input)?;

    Ok((input, Instruction::Noop))
}

fn addx(input: &str) -> IResult<&str, Instruction> {
    let (input, value) = preceded(tag("addx "), complete::i32)(input)?;

    Ok((input, Instruction::Addx(value)))
}
