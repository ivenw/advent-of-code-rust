use std::fmt::{Display, Formatter};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::{many1, separated_list1},
    IResult, Parser,
};

const BLOCK_SHAPES: &str = "####

.#.
###
.#.

..#
..#
###

#
#
#
#

##
##";

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Rock {
    Rock,
    Gap,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct Coord(pub usize, pub usize);

impl Coord {
    pub fn add(&self, other: &Coord) -> Coord {
        Coord(self.0 + other.0, self.1 + other.1)
    }
    pub fn sub(&self, other: &Coord) -> Coord {
        Coord(self.0 - other.0, self.1 - other.1)
    }
}

pub struct RockFormation {
    pub rocks: Vec<Vec<Rock>>,
    pub offset: Vec<Coord>,
}

impl RockFormation {
    pub fn height(&self) -> usize {
        self.rocks.len()
    }

    pub fn width(&self) -> usize {
        self.rocks
            .iter()
            .map(|row| row.iter().filter(|block| **block == Rock::Rock).count())
            .max()
            .unwrap()
    }

    pub fn get_rock_positions(&self, formation_position: &Coord) -> Vec<Coord> {
        self.offset
            .iter()
            .map(|coord| {
                Coord(
                    formation_position.0 + coord.0,
                    formation_position.1 - coord.1,
                )
            })
            .collect()
    }
}

impl Display for RockFormation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in &self.rocks {
            for block in row {
                match block {
                    Rock::Rock => write!(f, "#")?,
                    Rock::Gap => write!(f, ".")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn rock_formations() -> Vec<RockFormation> {
    let (_, blocks) = separated_list1(tag("\n\n"), rock_formation)(BLOCK_SHAPES).unwrap();
    blocks
}

fn rock_formation(input: &str) -> IResult<&str, RockFormation> {
    let (input, rocks) = separated_list1(
        newline,
        many1(alt((
            complete::char('#').map(|_| Rock::Rock),
            complete::char('.').map(|_| Rock::Gap),
        ))),
    )(input)?;

    let offset = rocks
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(x, block)| match block {
                    Rock::Rock => Some(Coord(x, y)),
                    Rock::Gap => None,
                })
        })
        .collect();

    Ok((input, RockFormation { rocks, offset }))
}
