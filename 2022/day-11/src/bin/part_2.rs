use itertools::Itertools;
use std::fs;

extern crate day_11;

use day_11::input_parser::parse_input;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let result = function(input);

    println!("{}", result);
}

fn function(input: String) -> u64 {
    let (_, mut monkeys) = parse_input(&input).unwrap();

    let lcm = monkeys.iter().map(|monkey| monkey.test).product::<u64>();

    for _ in 0..10000 {
        for monkey_number in 0..monkeys.len() {
            for _ in 0..monkeys[monkey_number].items.len() {
                let monkey = monkeys.get_mut(monkey_number).unwrap();
                let item = monkey.inspect_item_part_2(&lcm);
                let target_monkey = monkey.throw_item(item);
                monkeys[target_monkey].items.push_back(item);
            }
        }
    }

    monkeys
        .iter()
        .map(|monkey| monkey.times_inspected)
        .collect::<Vec<_>>()
        .iter()
        .sorted()
        .rev()
        .take(2)
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function() {
        // read test input file into a string
        let input = fs::read_to_string("test_input.txt").unwrap();

        let result = function(input);

        assert_eq!(result, 2713310158);
    }
}
