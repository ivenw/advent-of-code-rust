use std::fs;

extern crate day_14;

use day_14::input_parser::{parse_input, Coors, Tile};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let result = function(input);

    println!("{}", result);
}

fn function(input: String) -> u32 {
    let (_, mut map) = parse_input(&input).unwrap();

    let sand_spawn = Coors { x: 500, y: 0 };

    let mut sand_coors = sand_spawn;

    loop {
        if sand_coors.y + 1 >= map.len() as i32 {
            break;
        } else if map[(sand_coors.y + 1) as usize][sand_coors.x as usize] == Tile::Empty {
            sand_coors.y += 1;
        } else if map[(sand_coors.y + 1) as usize][(sand_coors.x - 1) as usize] == Tile::Empty {
            sand_coors = Coors {
                x: sand_coors.x - 1,
                y: sand_coors.y + 1,
            };
        } else if map[(sand_coors.y + 1) as usize][(sand_coors.x + 1) as usize] == Tile::Empty {
            sand_coors = Coors {
                x: sand_coors.x + 1,
                y: sand_coors.y + 1,
            };
        } else {
            map[sand_coors.y as usize][sand_coors.x as usize] = Tile::Sand;
            sand_coors = sand_spawn;
        }
    }

    map.iter()
        .flat_map(|row| {
            row.iter().filter_map(|tile| match tile {
                Tile::Sand => Some(1),
                _ => None,
            })
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

        assert_eq!(result, 24);
    }
}
