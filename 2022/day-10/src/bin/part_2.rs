use std::fs;

extern crate day_10;

use day_10::input_parser::{parse_input, Instruction};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let result = function(input);

    println!("{}", result);
}

fn function(input: String) -> String {
    let (_, instructions) = parse_input(&input).unwrap();

    let mut instruction = Instruction::Noop;
    let mut duration = instruction.cycles();

    let mut instructions = instructions.into_iter();
    let mut screen = vec!['.'; 240];
    let scanline = (-2..=41).collect::<Vec<i32>>();
    let sprite = scanline.windows(3).collect::<Vec<_>>();

    let mut register_x: i32 = 1;

    for (cycle, pixel) in screen.iter_mut().enumerate() {
        duration -= 1;

        // If current instruction is done
        if duration == 0 {
            // Add its value to the register
            instruction.addx(&mut register_x);

            // Get the next instruction. If there is none, stop looping
            instruction = instructions.next().unwrap_or(Instruction::Noop);

            // Set the duration of the next instruction
            duration = instruction.cycles();
        }

        let sprite_position = (register_x + 1).unsigned_abs() as usize;

        // dbg!(register_x, sprite_position, cycle % 40, cycle);

        if sprite[sprite_position].contains(&((cycle % 40) as i32)) {
            *pixel = '#';
            dbg!("set pixel");
        }
    }

    print_screen(&screen)
}

fn print_screen(screen: &[char]) -> String {
    let output = screen
        .chunks(40)
        .map(|line| line.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n");

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function() {
        // read test input file into a string
        let input = fs::read_to_string("test_input.txt").unwrap();

        let result = function(input);

        assert_eq!(
            result,
            "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
        );
    }

    #[test]
    fn test_function_2() {
        // read test input file into a string
        let input = fs::read_to_string("test_input_2.txt").unwrap();

        let result = function(input);

        assert_eq!(
            result,
            "###...##..###..#..#.####.#..#.####...##.
#..#.#..#.#..#.#.#..#....#.#..#.......#.
#..#.#..#.#..#.##...###..##...###.....#.
###..####.###..#.#..#....#.#..#.......#.
#....#..#.#....#.#..#....#.#..#....#..#.
#....#..#.#....#..#.#....#..#.####..##.."
        );
    }
}
