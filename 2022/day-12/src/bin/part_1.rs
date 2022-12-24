use std::{collections::BTreeMap, fs};

extern crate day_12;

use day_12::input_parser::parse_input;
use petgraph::{algo::dijkstra, prelude::DiGraphMap};

type Position = (i32, i32);
type Grid<T> = BTreeMap<Position, T>;
type Edge<K, T> = ((K, T), (K, T));

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let result = function(input);

    println!("{}", result);
}

fn function(input: String) -> u32 {
    let mut grid = parse_input(&input);

    let start = get_key_with_value(&grid, 'S').unwrap();
    let end = get_key_with_value(&grid, 'E').unwrap();

    grid.entry(start.0).and_modify(|v| *v = 'a');
    grid.entry(end.0).and_modify(|v| *v = 'z');

    let edges: Vec<Edge<Position, char>> = grid
        .iter()
        .flat_map(|(k, v)| {
            neighbors(&grid, k)
                .iter()
                .filter_map(|(n_k, n_v)| {
                    if *v as u8 + 1 >= *n_v as u8 {
                        Some(((*k, *v), (*n_k, *n_v)))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect();

    let graph: DiGraphMap<(Position, char), ()> = DiGraphMap::from_edges(&edges);

    let start = (start.0, 'a');
    let end = (end.0, 'z');

    dijkstra(&graph, start, Some(end), |_| 1)[&end]
}

fn get_key_with_value<K, T>(grid: &BTreeMap<K, T>, value: T) -> Option<(K, T)>
where
    K: Copy,
    T: PartialEq + Copy,
{
    grid.iter()
        .find_map(|(k, v)| if *v == value { Some((*k, *v)) } else { None })
}

fn neighbors(grid: &Grid<char>, position: &Position) -> Vec<(Position, char)> {
    [
        (position.0 - 1, position.1),
        (position.0 + 1, position.1),
        (position.0, position.1 - 1),
        (position.0, position.1 + 1),
    ]
    .iter()
    .filter_map(|neighbor| grid.get_key_value(neighbor))
    .map(|(k, v)| (*k, *v))
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function() {
        // read test input file into a string
        let input = fs::read_to_string("test_input.txt").unwrap();

        let result = function(input);

        assert_eq!(result, 31);
    }
}
