use std::fs;

enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let result = function(input);

    println!("{}", result);
}

fn function(input: String) -> u32 {
    // Given a input string with the first column between A and C, and the second column
    // between X and Z. A is worth 1 point, B, is worth 2 points, and C, is worth
    // 3 points.
    // A beats C
    // B beats A
    // C beats B
    // X means you need to loose, Y means you need to draw, and Z means you need to win.
    // Losing gives 0 points, a draw gives 3 additional points, and a win gives 6 additional
    // points.
    // Return the total number of points for the input.
    input
        .lines()
        .map(|line| {
            let mut chars = line.chars();
            let first = chars.next().unwrap();
            let _ = chars.next().unwrap();
            let second = chars.next().unwrap();

            let mut points = 0;

            if first == 'A' && second == 'X' {
                points += 3;
            } else if first == 'A' && second == 'Y' {
                points += 1;
            } else if first == 'A' && second == 'Z' {
                points += 2;
            } else if first == 'B' && second == 'X' {
                points += 1;
            } else if first == 'B' && second == 'Y' {
                points += 2;
            } else if first == 'B' && second == 'Z' {
                points += 3;
            } else if first == 'C' && second == 'X' {
                points += 2;
            } else if first == 'C' && second == 'Y' {
                points += 3;
            } else if first == 'C' && second == 'Z' {
                points += 1;
            } else {
                panic!("Invalid second character");
            }

            if second == 'X' {
                points += 0;
            } else if second == 'Y' {
                points += 3;
            } else if second == 'Z' {
                points += 6;
            } else {
                panic!("Invalid second character");
            }

            points
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

        assert_eq!(result, 12);
    }
}
