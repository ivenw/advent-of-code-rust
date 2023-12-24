fn main() {
    const INPUT: &str = include_str!("../../input.txt");

    let result = solve(INPUT);

    println!("{}", result)
}

fn solve(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (first_number, last_number) = (process_forward(line), process_backward(line));

            first_number * 10 + last_number
        })
        .sum()
}

fn process_forward(line: &str) -> u32 {
    let mut index = 0;
    let mut line_iter = std::iter::from_fn(|| {
        let reduced_line = &line[index..];

        if reduced_line.starts_with("one") {
            Some('1')
        } else if reduced_line.starts_with("two") {
            Some('2')
        } else if reduced_line.starts_with("three") {
            Some('3')
        } else if reduced_line.starts_with("four") {
            Some('4')
        } else if reduced_line.starts_with("five") {
            Some('5')
        } else if reduced_line.starts_with("six") {
            Some('6')
        } else if reduced_line.starts_with("seven") {
            Some('7')
        } else if reduced_line.starts_with("eight") {
            Some('8')
        } else if reduced_line.starts_with("nine") {
            Some('9')
        } else {
            index += 1;
            reduced_line.chars().next()
        }
    });

    line_iter
        .find(|b| b.is_numeric())
        .unwrap()
        .to_digit(10)
        .unwrap()
}

fn process_backward(line: &str) -> u32 {
    let mut index = line.len();
    let mut line_iter = std::iter::from_fn(|| {
        let reduced_line = &line[..index];

        if reduced_line.ends_with("one") {
            Some('1')
        } else if reduced_line.ends_with("two") {
            Some('2')
        } else if reduced_line.ends_with("three") {
            Some('3')
        } else if reduced_line.ends_with("four") {
            Some('4')
        } else if reduced_line.ends_with("five") {
            Some('5')
        } else if reduced_line.ends_with("six") {
            Some('6')
        } else if reduced_line.ends_with("seven") {
            Some('7')
        } else if reduced_line.ends_with("eight") {
            Some('8')
        } else if reduced_line.ends_with("nine") {
            Some('9')
        } else {
            index -= 1;
            reduced_line.chars().next_back()
        }
    });

    line_iter
        .find(|b| b.is_numeric())
        .unwrap()
        .to_digit(10)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn test_solve() {
        assert_eq!(solve(INPUT), 281);
    }
}
