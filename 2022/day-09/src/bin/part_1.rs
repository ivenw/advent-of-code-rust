use std::{cmp::max, collections::HashSet, fs};

extern crate day_09;

use day_09::input_parser::{parse_input, Direction};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let result = function(input);

    println!("{}", result);
}

fn function(input: String) -> u32 {
    let (_, directions) = parse_input(&input).unwrap();

    let mut head_position = Position { x: 0, y: 0 };
    let mut tail_position = Position { x: 0, y: 0 };
    let mut tail_movement = HashSet::from([tail_position.clone()]);

    // print_rope(&head_position, &tail_position, 10);

    for direction in directions {
        move_head(&mut head_position, direction);

        // println!("Head moved to {},{}", head_position.x, head_position.y);
        // print_rope(&head_position, &tail_position, 10);

        move_tail(&mut tail_position, &head_position);

        tail_movement.insert(tail_position.clone());

        // print_rope(&head_position, &tail_position, 10);
    }

    tail_movement.len() as u32
}

fn move_head(head_position: &mut Position, direction: Direction) {
    match direction {
        Direction::Left => head_position.x -= 1,
        Direction::Right => head_position.x += 1,
        Direction::Up => head_position.y += 1,
        Direction::Down => head_position.y -= 1,
    }
}

fn move_tail(tail_position: &mut Position, head_position: &Position) {
    let difference = Position {
        x: head_position.x - tail_position.x,
        y: head_position.y - tail_position.y,
    };

    if chebyshev_distance(head_position, tail_position) > 1 {
        tail_position.x += difference.x.signum();
        tail_position.y += difference.y.signum();
    }
}

fn chebyshev_distance(a: &Position, b: &Position) -> i32 {
    max((a.x - b.x).abs(), (a.y - b.y).abs())
}

#[allow(dead_code)]
fn print_rope(head_position: &Position, tail_position: &Position, rope_length: usize) {
    let mut rope = vec![vec!['-'; rope_length]; rope_length];

    rope[tail_position.y as usize][tail_position.x as usize] = 'T';
    rope[head_position.y as usize][head_position.x as usize] = 'H';

    rope.iter().rev().for_each(|row| {
        row.iter().for_each(|cell| print!("{} ", cell));
        println!();
    });

    println!("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function() {
        // read test input file into a string
        let input = fs::read_to_string("test_input.txt").unwrap();

        let result = function(input);

        assert_eq!(result, 13);
    }
}
