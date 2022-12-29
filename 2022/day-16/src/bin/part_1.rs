use std::fs;

extern crate day_16;

use day_16::input_parser::{parse_input, Valve};
use petgraph::{
    algo::bellman_ford,
    prelude::{DiGraph, DiGraphMap},
    visit::{Bfs, Dfs, EdgeRef},
    Direction,
};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let result = function(input);

    println!("{}", result);
}

fn function(input: String) -> u32 {
    let (_, mut valves) = parse_input(&input).unwrap();

    let edges = valves
        .iter()
        .flat_map(|valve| {
            // dbg!(valve);
            valve
                .connected_valves
                .iter()
                .map(|v| (valve, valves.iter().find(|v2| v2.label == *v).unwrap(), 1.0))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // let test = valves[0]
    //     .connected_valves
    //     .iter()
    //     .map(|v| valves.iter().find(|v2| v2.label == *v).unwrap())
    //     .collect::<Vec<_>>()
    //     .iter()
    //     .max_by_key(|v| v.flow_rate)
    //     .unwrap();

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

        assert_eq!(result, 1651);
    }
}
