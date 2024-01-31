use std::collections::BTreeMap;

use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, space1},
    multi::separated_list1,
    sequence::{delimited, separated_pair, tuple},
    IResult,
};

fn main() {
    const INPUT: &str = include_str!("../../input.txt");

    let result = solve(INPUT);

    println!("{}", result)
}

#[derive(Debug)]
struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    my_numbers: Vec<u32>,
}

fn solve(input: &str) -> u32 {
    let cards = parse(input);

    let data: Vec<(u32, u32)> = cards.iter().map(|c| (c.id, evaluate_card(c))).collect();

    let store: BTreeMap<u32, u32> = cards.iter().map(|c| (c.id, 1)).collect();

    data.iter()
        .fold(store, |mut acc, (id, num)| {
            let to_add = *acc.get(id).unwrap();
            (*id + 1..(id + 1 + num)).for_each(|i| {
                acc.entry(i).and_modify(|e| *e += to_add);
            });
            acc
        })
        .values()
        .sum()
}

fn evaluate_card(card: &Card) -> u32 {
    card.my_numbers
        .iter()
        .filter(|&n| card.winning_numbers.contains(n))
        .count() as u32
}

fn parse(input: &str) -> Vec<Card> {
    let (_, cards) = separated_list1(line_ending, card)(input).unwrap();
    cards
}
fn card(input: &str) -> IResult<&str, Card> {
    let (input, id) = delimited(
        tuple((tag("Card"), space1)),
        nom::character::complete::u32,
        tuple((tag(":"), space1)),
    )(input)?;

    let (input, (winning_numbers, my_numbers)) =
        separated_pair(numbers, tuple((space1, tag("|"), space1)), numbers)(input)?;

    Ok((
        input,
        Card {
            id,
            winning_numbers,
            my_numbers,
        },
    ))
}
fn numbers(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(space1, nom::character::complete::u32)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";

    #[test]
    fn test_solve() {
        assert_eq!(solve(INPUT), 30);
    }
}
