use nom::{
    bytes::complete::tag,
    character::complete::{self, newline, one_of},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

#[derive(Debug, Clone)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

pub fn parse_input(input: &str) -> IResult<&str, Vec<Direction>> {
    let (input, directions) = separated_list1(newline, movement_instruction)(input)?;

    let result = directions.into_iter().flatten().collect();

    Ok((input, result))
}

fn movement_instruction(input: &str) -> IResult<&str, Vec<Direction>> {
    let (input, (direction, distance)) =
        separated_pair(one_of("LRUD"), tag(" "), complete::u32)(input)?;
    let distance = distance as usize;

    let result = match direction {
        'L' => vec![Direction::Left; distance],
        'R' => vec![Direction::Right; distance],
        'U' => vec![Direction::Up; distance],
        'D' => vec![Direction::Down; distance],
        _ => panic!("Invalid direction"),
    };

    Ok((input, result))
}
