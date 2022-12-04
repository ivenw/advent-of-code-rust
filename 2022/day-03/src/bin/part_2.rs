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
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|group| {
            let (first_elf, second_elf, third_elf) = (group[0], group[1], group[2]);

            let (first_elf_unique_items, second_elf_unique_items, third_elf_unique_items) = (
                first_elf.chars().collect::<HashSet<char>>(),
                second_elf.chars().collect::<HashSet<char>>(),
                third_elf.chars().collect::<HashSet<char>>(),
            );

            let duplicate_item = first_elf_unique_items
                .intersection(&second_elf_unique_items)
                .filter(|item| third_elf_unique_items.contains(item))
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

        assert_eq!(result, 70);
    }
}
