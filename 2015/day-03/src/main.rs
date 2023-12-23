use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");
    let num_visited_houses = deliver_presents(input);
    let num_visited_houses_w_robo_santa = deliver_presents_w_robo_santa(input);

    println!("Houses visited: {}", num_visited_houses);
    println!(
        "Houses visited with robo santa: {}",
        num_visited_houses_w_robo_santa
    );
}

fn deliver_presents(input: &str) -> usize {
    let mut visited_houses = HashSet::new();
    visited_houses.insert(Coor::new(0, 0));
    let mut santa = Coor::new(0, 0);

    for c in input.chars() {
        match c {
            '^' => santa.y += 1,
            'v' => santa.y -= 1,
            '>' => santa.x += 1,
            '<' => santa.x -= 1,
            _ => (),
        }
        visited_houses.insert(santa.clone());
    }
    visited_houses.len()
}

fn deliver_presents_w_robo_santa(input: &str) -> usize {
    let mut visited_houses = HashSet::new();
    visited_houses.insert(Coor::new(0, 0));
    let mut santa = Coor::new(0, 0);
    let mut robo_santa = Coor::new(0, 0);

    for (i, c) in input.chars().enumerate() {
        let current_santa = if i % 2 == 0 {
            &mut santa
        } else {
            &mut robo_santa
        };
        match c {
            '^' => current_santa.y += 1,
            'v' => current_santa.y -= 1,
            '>' => current_santa.x += 1,
            '<' => current_santa.x -= 1,
            _ => (),
        }
        visited_houses.insert(current_santa.clone());
    }
    visited_houses.len()
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Coor {
    x: i32,
    y: i32,
}
impl Coor {
    fn new(x: i32, y: i32) -> Coor {
        Coor { x, y }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deliver_presents() {
        assert_eq!(deliver_presents(">"), 2);
        assert_eq!(deliver_presents("^>v<"), 4);
        assert_eq!(deliver_presents("^v^v^v^v^v"), 2);
    }

    #[test]
    fn test_deliver_presents_w_robo_santa() {
        assert_eq!(deliver_presents_w_robo_santa("^v"), 3);
        assert_eq!(deliver_presents_w_robo_santa("^>v<"), 3);
        assert_eq!(deliver_presents_w_robo_santa("^v^v^v^v^v"), 11);
    }
}
