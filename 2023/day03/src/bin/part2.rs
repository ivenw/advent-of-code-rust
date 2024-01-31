use std::collections::{HashSet, VecDeque};

fn main() {
    const INPUT: &str = include_str!("../../input.txt");

    let result = solve(INPUT);

    println!("{}", result)
}
struct Number {
    value: u32,
    neighbors: HashSet<Coord>,
}

#[derive(Eq, Hash, PartialEq)]
struct Char {
    value: char,
    coord: Coord,
}
#[derive(Eq, Hash, PartialEq, Debug, Clone, Copy)]
struct Coord {
    x: usize,
    y: usize,
}

fn solve(input: &str) -> u32 {
    let mut buffer: VecDeque<Char> = VecDeque::new();
    let mut numbers: Vec<Number> = vec![];
    let mut gear_coords = vec![];
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            if c.is_ascii_digit() {
                let symbol = Char {
                    value: c,
                    coord: Coord { x, y },
                };
                buffer.push_back(symbol);
            } else {
                if c == '*' {
                    let coord = Coord { x, y };
                    gear_coords.push(coord);
                }
                if !buffer.is_empty() {
                    let value = buffer
                        .iter()
                        .map(|s| s.value)
                        .collect::<String>()
                        .parse()
                        .unwrap();

                    let min_coord = buffer.pop_front().unwrap().coord;
                    let max_coord = if buffer.is_empty() {
                        min_coord
                    } else {
                        buffer.pop_back().unwrap().coord
                    };
                    let neighbors = generate_neighbors(min_coord, max_coord);
                    let number = Number { value, neighbors };
                    numbers.push(number);
                    buffer.clear();
                }
            }
        })
    });

    gear_coords
        .iter()
        .map(|g| {
            numbers.iter().filter_map(|n| {
                if n.neighbors.contains(g) {
                    Some(n.value)
                } else {
                    None
                }
            })
        })
        .filter_map(|n| if n.clone().count() > 1 { Some(n) } else { None })
        .map(|n| n.product::<u32>())
        .sum()
}

fn generate_neighbors(min_coord: Coord, max_coord: Coord) -> HashSet<Coord> {
    let min_x = (min_coord.x as i32 - 1).max(0) as usize;
    let max_x = (max_coord.x as i32 + 1) as usize;
    let min_y = (min_coord.y as i32 - 1).max(0) as usize;
    let max_y = (max_coord.y as i32 + 1) as usize;

    (min_x..=max_x)
        .flat_map(|x| {
            (min_y..=max_y).filter_map(move |y| {
                if (min_coord.x..=max_coord.x).contains(&x)
                    && (min_coord.y..=max_coord.y).contains(&y)
                {
                    return None;
                }
                Some(Coord { x, y })
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test_solve() {
        assert_eq!(solve(INPUT), 467835);
    }
}
