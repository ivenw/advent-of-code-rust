use std::collections::BTreeMap;

use super::input_parser::{Cd, Command, FsObject};

pub fn calculate_dir_sizes<'a>(
    (mut path, mut dir_sizes): (Vec<&'a str>, BTreeMap<Vec<&'a str>, u32>),
    command: &'a Command,
) -> (Vec<&'a str>, BTreeMap<Vec<&'a str>, u32>) {
    match command {
        Command::Cd(_) => {
            update_path(&mut path, command);
        }
        Command::Ls(file_objects) => {
            let sum = calculate_dir_size(file_objects);

            add_dir_size(&path, &mut dir_sizes, sum);
        }
    }
    (path, dir_sizes)
}

fn update_path<'a>(path: &mut Vec<&'a str>, command: &'a Command) {
    match command {
        Command::Cd(Cd::Root) => {
            path.clear(); // Not needed in this challenge
            path.push("");
        }
        Command::Cd(Cd::Up) => {
            path.pop();
        }
        Command::Cd(Cd::Down(name)) => {
            path.push(name);
        }
        _ => {}
    }
}

fn calculate_dir_size(file_objects: &[FsObject]) -> u32 {
    file_objects
        .iter()
        .filter_map(|file_object| match file_object {
            FsObject::File(size) => Some(size),
            _ => None,
        })
        .sum::<u32>()
}

fn add_dir_size<'a>(path: &Vec<&'a str>, dir_sizes: &mut BTreeMap<Vec<&'a str>, u32>, sum: u32) {
    for i in 0..path.len() {
        // For each element in the `path` vector, insert or modify the entry in the `dir_sizes`
        // map with a key equal to the slice of the `path` vector from the start to the current
        // element (inclusive). The value of the entry is the sum of the current entry value
        // (if it exists) and the `sum` argument. If the key does not exist in the map,
        // insert a new entry with the given key and value.
        dir_sizes
            .entry(path[0..=i].to_vec())
            .and_modify(|entry| *entry += sum)
            .or_insert(sum);
    }
}
