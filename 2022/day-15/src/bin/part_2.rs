use std::{collections::HashMap, fs};

extern crate day_15;

use day_15::input_parser::parse_input;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let result = function(input, 4_000_000);

    println!("{}", result);
}

fn function(input: String, max: i32) -> u64 {
    let (_, sensors) = parse_input(&input).unwrap();

    let ranges = sensors
        .iter()
        .map(|s| {
            let max_distance = s.range() as i32;
            let y_min = (s.pos.y - max_distance).max(0);
            let y_max = (s.pos.y + max_distance).min(max);

            (y_min..=y_max)
                .map(|y| {
                    let distance_to_line = (s.pos.y - y).abs();
                    let max_distance_on_line = max_distance - distance_to_line;
                    let start = (s.pos.x - max_distance_on_line).max(0);
                    let end = (s.pos.x + max_distance_on_line).min(max);

                    (y, start..=end)
                })
                .collect::<HashMap<_, _>>()
        })
        .collect::<Vec<_>>();

    (0..=max)
        .find_map(|y| {
            let mut x = 0;
            'mid: while x <= max {
                for s in &ranges {
                    if s.contains_key(&y) && s[&y].contains(&x) {
                        x = *s[&y].end() + 1;
                        continue 'mid;
                    }
                }
                return Some(x as u64 * 4_000_000 + y as u64);
            }
            None
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function() {
        // read test input file into a string
        let input = fs::read_to_string("test_input.txt").unwrap();

        let result = function(input, 20);

        assert_eq!(result, 56000011);
    }
}
