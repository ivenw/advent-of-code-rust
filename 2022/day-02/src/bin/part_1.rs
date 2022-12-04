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
            "A" | "X" => Ok(Move::Rock),
            "B" | "Y" => Ok(Move::Paper),
            "C" | "Z" => Ok(Move::Scissors),
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
            let moves: Vec<Move> = line
                .split(' ')
                .map(|letter| letter.parse::<Move>().unwrap())
                .collect();

            let (their_move, my_move) = (moves[0], moves[1]);

            let mut points = my_move as u32;

            match (my_move, their_move) {
                (Move::Rock, Move::Scissors)
                | (Move::Paper, Move::Rock)
                | (Move::Scissors, Move::Paper) => points += 6,
                _ if my_move == their_move => points += 3,
                _ => points += 0,
            }

            dbg!(points);

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

        assert_eq!(result, 15);
    }
}
