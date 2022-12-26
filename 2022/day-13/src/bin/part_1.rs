use std::{cmp::Ordering, fs};

extern crate day_13;

use day_13::input_parser::parse_input;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let result = function(input);

    println!("{}", result);
}

fn function(input: String) -> u32 {
    let (_, packet_pairs) = parse_input(&input).unwrap();

    packet_pairs
        .iter()
        .enumerate()
        .filter_map(|(idx, (left, right))| match left.cmp(right) {
            Ordering::Less => Some(idx + 1),
            Ordering::Greater => None,
            _ => unreachable!(),
        })
        .sum::<usize>() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function() {
        // read test input file into a string
        let input = fs::read_to_string("test_input.txt").unwrap();

        let result = function(input);

        assert_eq!(result, 13);
    }
}
