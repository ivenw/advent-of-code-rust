use std::fs;

extern crate day_10;

use day_10::input_parser::{parse_input, Instruction};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let result = function(input);

    println!("{}", result);
}

fn function(input: String) -> i32 {
    let (_, instructions) = parse_input(&input).unwrap();

    let mut register_x = 1;
    let mut cycle = 0;
    let mut signal_strengths = Vec::<i32>::new();

    let mut instruction_duration = 1;
    let mut current_instruction = Instruction::Noop;

    let mut instructions = instructions.into_iter();

    let mut looping = true;
    while looping {
        instruction_duration -= 1;

        if cycle == 20 || (cycle > 20 && (cycle - 20) % 40 == 0) {
            signal_strengths.push(register_x * cycle);
        }

        // If current instruction is done
        if instruction_duration == 0 {
            // Add its value to the register
            current_instruction.addx(&mut register_x);

            // Get the next instruction. If there is none, stop looping
            current_instruction = instructions.next().unwrap_or_else(|| {
                looping = false;
                Instruction::Noop
            });

            // Set the duration of the next instruction
            instruction_duration = current_instruction.cycles();
        }

        // Advance cpu cycle
        cycle += 1;
    }
    signal_strengths.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function() {
        // read test input file into a string
        let input = fs::read_to_string("test_input.txt").unwrap();

        let result = function(input);

        assert_eq!(result, 13140);
    }
}
