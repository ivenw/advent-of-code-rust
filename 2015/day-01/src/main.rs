fn main() {
    let input = include_str!("../input.txt");
    let floor = go_to_floor(input);
    let first = first_in_basement(input);

    println!("{}", floor);
    println!("{}", first);
}

fn go_to_floor(instructions: &str) -> i32 {
    let mut floor = 0;
    for c in instructions.chars() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => (),
        }
    }
    floor
}

fn first_in_basement(instructions: &str) -> i32 {
    let mut floor = 0;
    for (i, c) in instructions.chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => (),
        }
        if floor == -1 {
            return i as i32 + 1;
        }
    }
    unreachable!()
}

mod tests {
    #[test]
    fn test_go_to_floor() {
        assert_eq!(super::go_to_floor("(())"), 0);
        assert_eq!(super::go_to_floor("()()"), 0);
        assert_eq!(super::go_to_floor("((("), 3);
        assert_eq!(super::go_to_floor("(()(()("), 3);
        assert_eq!(super::go_to_floor("))((((("), 3);
        assert_eq!(super::go_to_floor("())"), -1);
        assert_eq!(super::go_to_floor("))("), -1);
        assert_eq!(super::go_to_floor(")))"), -3);
        assert_eq!(super::go_to_floor(")())())"), -3);
    }

    #[test]
    fn test_first_in_basement() {
        assert_eq!(super::first_in_basement(")"), 1);
        assert_eq!(super::first_in_basement("()())"), 5);
    }
}
