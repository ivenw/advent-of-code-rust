use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::{separated_list0, separated_list1},
    sequence::{delimited, separated_pair},
    IResult,
};

use super::packet::Packet;

pub fn parse_input(input: &str) -> IResult<&str, Vec<(Packet, Packet)>> {
    let (input, result) = separated_list1(tag("\n\n"), packet_pair)(input)?;

    Ok((input, result))
}

fn packet_pair(input: &str) -> IResult<&str, (Packet, Packet)> {
    let (input, result) = separated_pair(data, newline, data)(input)?;

    Ok((input, result))
}

fn data(input: &str) -> IResult<&str, Packet> {
    let (input, result) = alt((list, int))(input)?;

    Ok((input, result))
}

fn list(input: &str) -> IResult<&str, Packet> {
    let (input, result) = delimited(tag("["), separated_list0(tag(","), data), tag("]"))(input)?;

    Ok((input, Packet::List(result)))
}

fn int(input: &str) -> IResult<&str, Packet> {
    let (input, result) = complete::u32(input)?;

    Ok((input, Packet::Int(result)))
}
