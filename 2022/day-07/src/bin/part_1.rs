use std::{collections::BTreeMap, fs};

extern crate day_07;
use day_07::input_parser::{parse_input, Cd, Command, FsObject};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let result = function(input);

    println!("{}", result);
}

fn function(input: String) -> u32 {
    let (_, commands) = parse_input(&input).unwrap();

    let mut file_tree = BTreeMap::new();

    let mut current_dir_name = "/";

    for command in commands {
        match command {
            Command::Cd(Cd::Root) => {
                file_tree.insert("/", BTreeMap::new());
            }
            Command::Cd(Cd::Up) => (),
            Command::Cd(Cd::Down(name)) => (),
            Command::Ls(list) => {
                for fs_object in list {
                    match fs_object {
                        FsObject::Dir(name) => {
                            file_tree.insert(name, BTreeMap::new());
                        }
                        FsObject::File { size, name } => (),
                    }
                }
            }
        }
    }

    dbg!(file_tree);
    0
}

// #[derive(Debug)]
// struct Dir<'a> {
//     children: BTreeMap<FsObject>,
// }

// #[derive(Debug)]
// struct File<'a> {
//     name: &'a str,
//     size: u32,
// }

// #[derive(Debug)]
// enum FsObject {
//     Dir(Dir),
//     File(File),
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function() {
        // read test input file into a string
        let input = fs::read_to_string("test_input.txt").unwrap();

        let result = function(input);

        assert_eq!(result, 95437);
    }
}
