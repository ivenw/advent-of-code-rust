use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let result = function(input);

    println!("{}", result);
}

fn function(input: String) -> u32 {
    input
        .lines()
        .map(|line| {
            let (first_elf, second_elf) = line.split_once(',').unwrap();

            let (first_elf_range, second_elf_range) = (
                first_elf.split_once('-').unwrap(),
                second_elf.split_once('-').unwrap(),
            );

            let (first_elf_start, first_elf_end) = (
                first_elf_range.0.parse::<u32>().unwrap(),
                first_elf_range.1.parse::<u32>().unwrap(),
            );

            let (second_elf_start, second_elf_end) = (
                second_elf_range.0.parse::<u32>().unwrap(),
                second_elf_range.1.parse::<u32>().unwrap(),
            );

            if (first_elf_start >= second_elf_start && first_elf_end <= second_elf_end)
                | (first_elf_start <= second_elf_start && first_elf_end >= second_elf_end)
            {
                return 1;
            }

            0
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

        assert_eq!(result, 2);
    }
}
