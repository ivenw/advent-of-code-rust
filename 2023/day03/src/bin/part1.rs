use std::collections::{HashSet, VecDeque};

fn main() {
    const INPUT: &str = include_str!("../../input.txt");

    let result = solve(INPUT);

    println!("{}", result)
}

#[derive(Debug)]
struct Number {
    value: u32,
    min_coord: Coord,
    max_coord: Coord,
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
    let mut numbers = vec![];
    let mut symbols: HashSet<Coord> = HashSet::new();
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            if c.is_ascii_digit() {
                let symbol = Char {
                    value: c,
                    coord: Coord { x, y },
                };
                buffer.push_back(symbol);
            } else {
                if c != '.' {
                    let coord = Coord { x, y };
                    symbols.insert(coord);
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

                    numbers.push(Number {
                        value,
                        min_coord,
                        max_coord,
                    });
                    buffer.clear();
                }
            }
        })
    });

    numbers
        .iter()
        .filter(|number| {
            let neighbors = generate_neighbors(number);
            !neighbors.is_disjoint(&symbols)
        })
        .map(|number| number.value)
        .sum()
}

fn generate_neighbors(number: &Number) -> HashSet<Coord> {
    let min_x = (number.min_coord.x as i32 - 1).max(0) as usize;
    let max_x = (number.max_coord.x as i32 + 1) as usize;
    let min_y = (number.min_coord.y as i32 - 1).max(0) as usize;
    let max_y = (number.max_coord.y as i32 + 1) as usize;

    (min_x..=max_x)
        .flat_map(|x| (min_y..=max_y).map(move |y| Coord { x, y }))
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
        assert_eq!(solve(INPUT), 4361);
    }
}
