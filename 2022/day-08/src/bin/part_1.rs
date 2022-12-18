use std::fs;

extern crate day_08;

use day_08::input_parser::parse_input;

fn main() {
    let input = fs::read_to_string("test_input.txt").unwrap();

    let result = function(input);

    println!("{}", result);
}

fn function(input: String) -> u32 {
    let mut trees = parse_input(&input);

    for mut col in trees.columns_mut() {
        for (idx, tree) in col.iter_mut().enumerate() {
            if idx == 0 {
                tree.v_visible = true;
            }
        }
    }

    for mut col in trees.columns_mut() {
        for (idx, tree) in col.iter_mut().rev().enumerate() {
            if idx == 0 {
                tree.v_visible = true;
            }
        }
    }

    for mut row in trees.rows_mut() {
        for (idx, tree) in row.iter_mut().enumerate() {
            if idx == 0 {
                tree.h_visible = true;
            }
        }
    }

    for mut row in trees.rows_mut() {
        for (idx, tree) in row.iter_mut().rev().enumerate() {
            if idx == 0 {
                tree.h_visible = true;
            }
        }
    }

    trees.iter().fold(0, |acc, tree| {
        if tree.h_visible || tree.v_visible {
            acc + 1
        } else {
            acc
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function() {
        // read test input file into a string
        let input = fs::read_to_string("test_input.txt").unwrap();

        let result = function(input);

        assert_eq!(result, 21);
    }
}
