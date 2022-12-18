use std::{collections::BTreeMap, fs};

extern crate day_07;

use day_07::dir_size_calc::calculate_dir_sizes;
use day_07::input_parser::parse_input;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let result = function(input);

    println!("{}", result);
}

fn function(input: String) -> u32 {
    let (_, commands) = parse_input(&input).unwrap();

    let (_, dir_sizes) = commands
        .iter()
        .fold((vec![], BTreeMap::new()), calculate_dir_sizes);

    dir_sizes
        .iter()
        .filter(|(_, &size)| size < 100_000)
        .map(|(_, size)| size)
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function() {
        // read test input file into a string
        let input = fs::read_to_string("test_input.txt").unwrap();

        let result = function(input);

        assert_eq!(result, 95437);
    }
}
