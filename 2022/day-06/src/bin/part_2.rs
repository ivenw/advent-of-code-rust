use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let result = function(input);

    println!("{}", result);
}

fn function(input: String) -> u32 {
    // iterate over input with window of 4 chars, collect each window into a hashset
    // if the hashset has 4 elements, return the position of the last element in the window
    let unique_chars_for_marker = 14;

    (input
        .chars()
        .collect::<Vec<char>>()
        .windows(unique_chars_for_marker)
        .map(|window| window.iter().collect::<std::collections::HashSet<&char>>())
        .position(|set| set.len() == unique_chars_for_marker)
        .unwrap()
        + unique_chars_for_marker) as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function() {
        // read test input file into a string
        let input = fs::read_to_string("test_input.txt").unwrap();

        let result = function(input);

        assert_eq!(result, 19);
    }
}
