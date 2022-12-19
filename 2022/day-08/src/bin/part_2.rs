use std::{fs, iter::zip};

extern crate day_08;

use day_08::input_parser::parse_input;
use ndarray::{
    iter::{Lanes, LanesMut},
    Array2,
};

fn main() {
    let input = fs::read_to_string("test_input.txt").unwrap();

    let result = function(input);

    println!("{}", result);
}

fn function(input: String) -> u32 {
    let trees = parse_input(&input);
    let mut scenic_scores = Array2::from_elem(trees.dim(), 0);

    scenic_scores.iter().max().unwrap().to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function() {
        // read test input file into a string
        let input = fs::read_to_string("test_input.txt").unwrap();

        let result = function(input);

        assert_eq!(result, 8);
    }
}
