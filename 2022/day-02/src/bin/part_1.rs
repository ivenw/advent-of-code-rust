use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let result = function(input);

    println!("{}", result);
}

fn function(input: String) -> u32 {
    // Given a input string with the first column between A and C, and the second column
    // between X and Z. A, X are worth 1 point, B, Y are worth 2 points, and C, Z are worth
    // 3 points.
    // A, X beat C, Z
    // B, Y beat A, X
    // C, Z beat B, Y
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

            let mut points = match second {
                'A' | 'X' => 1,
                'B' | 'Y' => 2,
                'C' | 'Z' => 3,
                _ => panic!("Invalid second character"),
            };

            if first == 'C' && second == 'X' {
                points += 6;
            } else if first == 'A' && second == 'Y' {
                points += 6;
            } else if first == 'B' && second == 'Z' {
                points += 6;
            } else if first == 'A' && second == 'X' {
                points += 3;
            } else if first == 'B' && second == 'Y' {
                points += 3;
            } else if first == 'C' && second == 'Z' {
                points += 3;
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

        dbg!(result);

        assert_eq!(result, 15);
    }
}
