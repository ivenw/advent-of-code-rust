use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, newline, space1},
    multi::{separated_list0, separated_list1},
    sequence::{delimited, preceded, tuple},
    IResult, Parser,
};

#[derive(Debug, PartialEq, Clone, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Valve<'a> {
    pub label: &'a str,
    pub flow_rate: u32,
    pub connected_valves: Vec<&'a str>,
}

pub fn parse_input(input: &str) -> IResult<&str, Vec<Valve>> {
    separated_list1(newline, valve)(input)
}

fn valve(input: &str) -> IResult<&str, Valve> {
    tuple((label, flow_rate, connected_valves))
        .map(|(label, flow_rate, connected_valves)| Valve {
            label,
            flow_rate,
            connected_valves,
        })
        .parse(input)
}

fn label(input: &str) -> IResult<&str, &str> {
    delimited(tag("Valve "), alpha1, space1)(input)
}

fn flow_rate(input: &str) -> IResult<&str, u32> {
    preceded(tag("has flow rate="), complete::u32)(input)
}

fn connected_valves(input: &str) -> IResult<&str, Vec<&str>> {
    preceded(
        alt((
            tag("; tunnels lead to valves "),
            tag("; tunnel leads to valve "),
        )),
        separated_list0(tag(", "), alpha1),
    )(input)
}
