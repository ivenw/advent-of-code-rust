use std::fs;

extern crate day_08;

use day_08::input_parser::parse_input;
use ndarray::{s, Array2};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let result = function(input);

    println!("{}", result);
}

fn function(input: String) -> u32 {
    let trees = parse_input(&input);
    let mut scenic_scores = Array2::from_elem(trees.dim(), 0);

    for (idx, tree) in trees.indexed_iter() {
        let mut scores = [0, 0, 0, 0];

        let row = trees.row(idx.0);
        let column = trees.column(idx.1);

        let left = row.slice(s![..idx.1; -1]);
        let right = row.slice(s![(idx.1 + 1)..]);
        let up = column.slice(s![..idx.0; -1]);
        let down = column.slice(s![(idx.0 + 1)..]);

        for (i, direction) in [left, right, up, down].iter().enumerate() {
            if direction.is_empty() {
                continue;
            }

            for tree_ahead in direction {
                scores[i] += 1;
                if *tree_ahead >= *tree {
                    break;
                }
            }
        }

        scenic_scores[idx] = scores.iter().product();
    }

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
