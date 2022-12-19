use std::{fs, iter::zip};

extern crate day_08;

use day_08::input_parser::parse_input;
use ndarray::{
    iter::{Lanes, LanesMut},
    Array2, Dim,
};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let result = function(input);

    println!("{}", result);
}

fn function(input: String) -> u32 {
    let tree_heights = parse_input(&input);
    let mut tree_visibilities = Array2::from_elem(tree_heights.dim(), 0);
    // visibility.map(RefCell::new);

    // Fill in the boarder with 1s
    tree_visibilities.column_mut(0).fill(1);
    tree_visibilities
        .column_mut(tree_visibilities.ncols() - 1)
        .fill(1);
    tree_visibilities.row_mut(0).fill(1);
    tree_visibilities
        .row_mut(tree_visibilities.nrows() - 1)
        .fill(1);

    walk_tree_lanes(tree_heights.columns(), tree_visibilities.columns_mut());
    walk_tree_lanes(tree_heights.rows(), tree_visibilities.rows_mut());

    tree_visibilities.sum()
}

fn walk_tree_lanes(
    height_lanes: Lanes<u32, Dim<[usize; 1]>>,
    visibility_lanes: LanesMut<u32, Dim<[usize; 1]>>,
) {
    for (height_lane, mut visibility_lane) in zip(height_lanes, visibility_lanes) {
        compare_trees(height_lane.iter(), visibility_lane.iter_mut());

        compare_trees(height_lane.iter().rev(), visibility_lane.iter_mut().rev());
    }
}

fn compare_trees<'a>(
    height_lane: impl Iterator<Item = &'a u32>,
    visibility_lane: impl Iterator<Item = &'a mut u32>,
) {
    let mut max_tree_height_so_far = 0;

    for (height, visibility) in zip(height_lane, visibility_lane) {
        if *height > max_tree_height_so_far {
            *visibility = 1;
        }
        max_tree_height_so_far = max_tree_height_so_far.max(*height);
    }
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
