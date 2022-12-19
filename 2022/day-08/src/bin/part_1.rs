use std::{fs, iter::zip};

extern crate day_08;

use day_08::input_parser::parse_input;
use ndarray::{
    iter::{Lanes, LanesMut},
    Array2,
};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let result = function(input);

    println!("{}", result);
}

fn function(input: String) -> u32 {
    let trees = parse_input(&input);
    let mut visibility = Array2::from_elem(trees.dim(), 0);
    // visibility.map(RefCell::new);

    // Fill in the boarder with 1s
    visibility.column_mut(0).fill(1);
    visibility.column_mut(visibility.ncols() - 1).fill(1);
    visibility.row_mut(0).fill(1);
    visibility.row_mut(visibility.nrows() - 1).fill(1);

    for (tree_col, mut vis_col) in zip(trees.columns(), visibility.columns_mut()) {
        let mut max_tree_height_so_far = 0;

        for (tree, vis) in zip(tree_col.iter(), vis_col.iter_mut()) {
            if *tree > max_tree_height_so_far {
                *vis = 1;
            }
            max_tree_height_so_far = max_tree_height_so_far.max(*tree);
        }

        max_tree_height_so_far = 0;

        for (tree, vis) in zip(tree_col.iter().rev(), vis_col.iter_mut().rev()) {
            if *tree > max_tree_height_so_far {
                *vis = 1;
            }
            max_tree_height_so_far = max_tree_height_so_far.max(*tree);
        }
    }

    for (tree_col, mut vis_col) in zip(trees.rows(), visibility.rows_mut()) {
        let mut max_tree_height_so_far = 0;

        for (tree, vis) in zip(tree_col.iter(), vis_col.iter_mut()) {
            if *tree > max_tree_height_so_far {
                *vis = 1;
            }
            max_tree_height_so_far = max_tree_height_so_far.max(*tree);
        }

        max_tree_height_so_far = 0;

        for (tree, vis) in zip(tree_col.iter().rev(), vis_col.iter_mut().rev()) {
            if *tree > max_tree_height_so_far {
                *vis = 1;
            }
            max_tree_height_so_far = max_tree_height_so_far.max(*tree);
        }
    }

    visibility.sum()
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
