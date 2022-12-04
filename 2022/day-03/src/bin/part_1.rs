use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let result = function(input);

    println!("{}", result);
}

fn function(input: String) -> u32 {
    let value_map: HashMap<char, u32> =
        HashMap::from_iter(('a'..='z').chain('A'..='Z').zip(1..=56));

    input
        .lines()
        .map(|rucksack| {
            let rucksack_size = rucksack.len();

            let (compartment_1, compartment_2) = rucksack.split_at(rucksack_size / 2);

            let (compartment_1_unique_items, compartment_2_unique_items) = (
                compartment_1.chars().collect::<HashSet<_>>(),
                compartment_2.chars().collect::<HashSet<_>>(),
            );

            let duplicate_item = compartment_1_unique_items
                .intersection(&compartment_2_unique_items)
                .take(1)
                .next()
                .unwrap();

            value_map.get(duplicate_item).unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function() {
        // read test input file into a string
        let input = fs::read_to_string("test_input.txt").unwrap();

        let result = function(input);

        dbg!(result);

        assert_eq!(result, 157);
    }
}
