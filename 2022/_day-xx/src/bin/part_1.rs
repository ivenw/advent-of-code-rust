use std::fs;

extern crate day_xx;

use day_xx::input_parser::parse_input;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let result = function(input);

    println!("{}", result);
}

fn function(input: String) -> u32 {
    let (_, result) = parse_input(&input).unwrap();

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function() {
        // read test input file into a string
        let input = fs::read_to_string("test_input.txt").unwrap();

        let result = function(input);

        assert_eq!(result, 0);
    }
}
