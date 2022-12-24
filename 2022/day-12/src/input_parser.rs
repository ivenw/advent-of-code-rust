use std::collections::BTreeMap;

pub fn parse_input(input: &str) -> BTreeMap<(i32, i32), char> {
    input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(move |(col, height)| ((row as i32, col as i32), height))
        })
        .collect()
}
