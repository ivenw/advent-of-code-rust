use itertools::Itertools;
use std::{collections::HashSet, fs};

extern crate day_15;

use day_15::input_parser::{parse_input, Coors};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let result = function(input, 2_000_000);

    println!("{}", result);
}

fn function(input: String, y_search: i32) -> u32 {
    let (_, sensors) = parse_input(&input).unwrap();

    sensors
        .iter()
        .flat_map(|sensor| {
            let mut distances = get_coors_within_distance(
                &sensor.pos,
                manhattan_distance(&sensor.pos, &sensor.beacon_pos),
                y_search,
            )
            .into_iter()
            .collect::<HashSet<_>>();

            distances.remove(&sensor.beacon_pos);
            distances
        })
        .collect::<HashSet<Coors>>()
        .len() as u32
}

// get all coors within a certain manhattan distance of a given coors
fn get_coors_within_distance(coors: &Coors, distance: u32, y_search: i32) -> Vec<Coors> {
    ((coors.x - distance as i32)..=(coors.x + distance as i32))
        .cartesian_product([y_search])
        .filter_map(|(x, y)| {
            if manhattan_distance(&Coors { x, y }, coors) <= distance {
                Some(Coors { x, y })
            } else {
                None
            }
        })
        .collect()
}

fn manhattan_distance(a: &Coors, b: &Coors) -> u32 {
    ((a.x - b.x).abs() + (a.y - b.y).abs()) as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function() {
        // read test input file into a string
        let input = fs::read_to_string("test_input.txt").unwrap();

        let result = function(input, 10);

        assert_eq!(result, 26);
    }
}
