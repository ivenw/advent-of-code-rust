fn main() {
    const INPUT: &str = include_str!("../../input.txt");

    let result = solve(INPUT);

    println!("{}", result)
}

fn solve(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (first_number, last_number) = (
                line.chars()
                    .find(|b| b.is_numeric())
                    .unwrap()
                    .to_digit(10)
                    .unwrap(),
                line.chars()
                    .rev()
                    .find(|b| b.is_ascii_digit())
                    .unwrap()
                    .to_digit(10)
                    .unwrap(),
            );

            first_number * 10 + last_number
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    #[test]
    fn test_solve() {
        assert_eq!(solve(INPUT), 142);
    }
}
