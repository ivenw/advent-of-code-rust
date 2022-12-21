use ::lending_iterator::prelude::*;
use std::{cmp::max, collections::HashSet, fs, thread::sleep, time::Duration};

extern crate day_09;

use day_09::input_parser::{parse_input, Direction};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
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

    let mut rope = vec![Position { x: 0, y: 0 }; 10];

    let mut tail_movement = HashSet::from([*rope.last().unwrap()]);

    // print_rope(&rope, &tail_movement);

    for direction in directions {
        move_head(&mut rope[0], direction);

        let mut rope_windows = rope.windows_mut::<2>();
        while let Some([ref mut leading_knot, ref mut lagging_knot]) = rope_windows.next() {
            move_tail(lagging_knot, leading_knot)
        }

        tail_movement.insert(*rope.last().unwrap());

        // print_rope(&rope, &tail_movement);
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
// super ugly and only for debuggint test cases
fn print_rope(rope: &[Position], tail_movement: &HashSet<Position>) {
    let grid_size = 31;
    let grid_middle = 15;
    let mut grid = vec![vec!['-'; grid_size]; grid_size];

    for pos in tail_movement {
        grid[(pos.y + grid_middle) as usize][(pos.x + grid_middle) as usize] = '#';
    }

    for (idx, position) in rope.iter().enumerate().rev() {
        match idx {
            0 => {
                grid[(position.y + grid_middle) as usize][(position.x + grid_middle) as usize] = 'H'
            }
            _ => {
                grid[(position.y + grid_middle) as usize][(position.x + grid_middle) as usize] =
                    idx.to_string().chars().next().unwrap()
            }
        }
    }

    grid.iter().rev().for_each(|row| {
        row.iter().for_each(|cell| print!("{} ", cell));
        println!();
    });

    println!("\n");
    sleep(Duration::from_millis(100));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function() {
        // read test input file into a string
        let input = fs::read_to_string("test_input.txt").unwrap();

        let result = function(input);

        assert_eq!(result, 1);
    }

    #[test]
    fn test_function_2() {
        // read test input file into a string
        let input = fs::read_to_string("test_input_2.txt").unwrap();

        let result = function(input);

        assert_eq!(result, 36);
    }
}
