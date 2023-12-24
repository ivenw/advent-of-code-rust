use std::cmp::Ordering;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1, line_ending},
    multi::separated_list1,
    sequence::{delimited, separated_pair},
    IResult,
};

#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
}
#[derive(Debug, PartialEq)]
struct Round {
    red: u32,
    green: u32,
    blue: u32,
}
impl PartialOrd for Round {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.red > other.red || self.green > other.green || self.blue > other.blue {
            Some(Ordering::Greater)
        } else if self.red < other.red && self.green < other.green && self.blue < other.blue {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Equal)
        }
    }
}
#[derive(Debug)]
struct Cube {
    count: u32,
    color: Color,
}
#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

fn main() {
    const INPUT: &str = include_str!("../../input.txt");

    let result = solve(INPUT);

    println!("{}", result)
}

fn solve(input: &str) -> u32 {
    const RULE: Round = Round {
        red: 12,
        green: 13,
        blue: 14,
    };

    let games = parse(input);

    games
        .iter()
        .filter(|game| game.rounds.iter().all(|round| round <= &RULE))
        .map(|game| game.id)
        .sum()
}

fn parse(input: &str) -> Vec<Game> {
    let (_, games) = separated_list1(line_ending, game)(input).unwrap();
    games
}
fn game(input: &str) -> IResult<&str, Game> {
    let (input, id) = delimited(tag("Game "), digit1, tag(": "))(input)?;
    let (input, rounds) = separated_list1(tag("; "), round)(input)?;
    let id = id.parse().unwrap();
    Ok((input, Game { id, rounds }))
}
fn round(input: &str) -> IResult<&str, Round> {
    let (input, cubes) = separated_list1(tag(", "), cube)(input)?;

    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;
    for cube in cubes {
        match cube.color {
            Color::Red => red += cube.count,
            Color::Green => green += cube.count,
            Color::Blue => blue += cube.count,
        }
    }
    Ok((input, Round { red, green, blue }))
}
fn cube(input: &str) -> IResult<&str, Cube> {
    let (input, (count, color)) = separated_pair(digit1, tag(" "), alpha1)(input)?;
    let count = count.parse().unwrap();
    let color = match color {
        "red" => Color::Red,
        "green" => Color::Green,
        "blue" => Color::Blue,
        _ => unreachable!(),
    };
    Ok((input, Cube { count, color }))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";

    #[test]
    fn test_solve() {
        assert_eq!(solve(INPUT), 8);
    }
}
