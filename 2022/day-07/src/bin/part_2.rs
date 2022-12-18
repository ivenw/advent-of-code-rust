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

    let total_space = 70_000_000;
    let space_needed = 30_000_000;
    let used_space = dir_sizes.get(&vec![""]).unwrap();
    let free_space = total_space - used_space;
    let need_to_free = space_needed - free_space;

    let mut sizes = Vec::from_iter(dir_sizes.values());
    sizes.sort();

    let mut out = 0;
    for &size in sizes {
        if size >= need_to_free {
            out = size;
            break;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function() {
        // read test input file into a string
        let input = fs::read_to_string("test_input.txt").unwrap();

        let result = function(input);

        assert_eq!(result, 24933642);
    }
}
