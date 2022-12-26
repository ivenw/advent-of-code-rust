use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Parser,
};

pub type Map = Vec<Vec<Tile>>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Tile {
    Empty,
    Rock,
    Sand,
}

pub type Path = Vec<Coors>;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Coors {
    pub x: i32,
    pub y: i32,
}

pub fn parse_input(input: &str) -> IResult<&str, Map> {
    let (_, rock_paths) = separated_list1(newline, path)(input)?;

    let (x_max, y_max) = rock_paths
        .iter()
        .flat_map(|path| path.iter())
        .fold((0, 0), |(x_max, y_max), coors| {
            (x_max.max(coors.x), y_max.max(coors.y))
        });

    let mut map = vec![vec![Tile::Empty; (x_max + 500) as usize]; (y_max + 1) as usize];

    rock_paths.iter().for_each(|path| {
        path.iter().tuple_windows().for_each(|(start, stop)| {
            let x_min = start.x.min(stop.x);
            let x_max = start.x.max(stop.x);
            let x_range = x_min..=x_max;

            let y_min = start.y.min(stop.y);
            let y_max = start.y.max(stop.y);
            let y_range = y_min..=y_max;

            x_range.cartesian_product(y_range).for_each(|(x, y)| {
                map[y as usize][x as usize] = Tile::Rock;
            });
        })
    });
    Ok((input, map))
}

fn path(input: &str) -> IResult<&str, Path> {
    separated_list1(tag(" -> "), coors)(input)
}

fn coors(input: &str) -> IResult<&str, Coors> {
    separated_pair(complete::i32, tag(","), complete::i32)
        .map(|(x, y)| Coors { x, y })
        .parse(input)
}
