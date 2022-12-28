use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult, Parser,
};

pub struct Sensor {
    pub pos: Coors,
    pub beacon_pos: Coors,
}

impl Sensor {
    pub fn range(&self) -> u32 {
        manhattan_distance(&self.pos, &self.beacon_pos)
    }
}

fn manhattan_distance(a: &Coors, b: &Coors) -> u32 {
    ((a.x - b.x).abs() + (a.y - b.y).abs()) as u32
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Coors {
    pub x: i32,
    pub y: i32,
}

pub fn parse_input(input: &str) -> IResult<&str, Vec<Sensor>> {
    separated_list1(newline, sensor)(input)
}

fn sensor(input: &str) -> IResult<&str, Sensor> {
    separated_pair(
        preceded(tag("Sensor at "), coors),
        tag(": closest beacon is at "),
        coors,
    )
    .map(|(pos, beacon_pos)| Sensor { pos, beacon_pos })
    .parse(input)
}

fn coors(input: &str) -> IResult<&str, Coors> {
    separated_pair(
        preceded(tag("x="), complete::i32),
        tag(", "),
        preceded(tag("y="), complete::i32),
    )
    .map(|(x, y)| Coors { x, y })
    .parse(input)
}
