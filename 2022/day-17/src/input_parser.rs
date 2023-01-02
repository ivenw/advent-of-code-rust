use nom::{branch::alt, character::complete, multi::many1, IResult, Parser};

#[derive(Debug)]
pub enum Direction {
    Down,
    Left,
    Right,
}

pub fn parse_input(input: &str) -> IResult<&str, Vec<Direction>> {
    many1(alt((
        complete::char('<').map(|_| Direction::Left),
        complete::char('>').map(|_| Direction::Right),
    )))(input)
}
