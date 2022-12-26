use std::fs;

extern crate day_13;

use day_13::{input_parser::parse_input, packet::Packet};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let result = function(input);

    println!("{}", result);
}

fn function(input: String) -> u32 {
    let (_, packet_pairs) = parse_input(&input).unwrap();

    let divider_packets = [
        Packet::List(vec![Packet::List(vec![Packet::Int(2)])]),
        Packet::List(vec![Packet::List(vec![Packet::Int(6)])]),
    ];

    let mut packets = packet_pairs
        .into_iter()
        .flat_map(|(left, right)| [left, right])
        .chain(divider_packets.iter().cloned())
        .collect::<Vec<_>>();

    packets.sort();

    [
        packets
            .iter()
            .position(|p| p == &divider_packets[0])
            .unwrap()
            + 1,
        packets
            .iter()
            .position(|p| p == &divider_packets[1])
            .unwrap()
            + 1,
    ]
    .iter()
    .product::<usize>() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function() {
        // read test input file into a string
        let input = fs::read_to_string("test_input.txt").unwrap();

        let result = function(input);

        assert_eq!(result, 1402);
    }
}
