use std::{fs, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Move::Rock),
            "B" => Ok(Move::Paper),
            "C" => Ok(Move::Scissors),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Outcome {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

impl FromStr for Outcome {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Outcome::Loss),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => unreachable!(),
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let result = function(input);

    println!("{}", result);
}

fn function(input: String) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut chars = line.chars();

            let their_move = chars.next().unwrap().to_string().parse::<Move>().unwrap();

            let my_move = chars
                .nth(1)
                .unwrap()
                .to_string()
                .parse::<Outcome>()
                .unwrap();

            let mut points = my_move as u32;

            points += match my_move {
                Outcome::Win => {
                    let our_move: Move = match their_move {
                        Move::Rock => Move::Paper,
                        Move::Paper => Move::Scissors,
                        Move::Scissors => Move::Rock,
                    };
                    our_move as u32
                }
                Outcome::Draw => {
                    let our_move = their_move;
                    our_move as u32
                }
                Outcome::Loss => {
                    let our_move = match their_move {
                        Move::Rock => Move::Scissors,
                        Move::Paper => Move::Rock,
                        Move::Scissors => Move::Paper,
                    };
                    our_move as u32
                }
            };

            points
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function() {
        // read test input file into a string
        let input = fs::read_to_string("test_input.txt").unwrap();

        let result = function(input);

        dbg!(result);

        assert_eq!(result, 12);
    }
}
