use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{alpha1, newline},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

#[derive(Debug)]
pub enum Command<'a> {
    Cd(Cd<'a>),
    Ls(Vec<FsObject<'a>>),
}

#[derive(Debug)]
pub enum Cd<'a> {
    Root,
    Up,
    Down(&'a str),
}

#[derive(Debug)]
pub enum FsObject<'a> {
    Dir(&'a str),
    File { size: u32, name: &'a str },
}

pub fn parse_input(input: &str) -> IResult<&str, Vec<Command>> {
    let (input, commands) = separated_list1(newline, parse_command)(input)?;
    Ok((input, commands))
}

fn parse_command(input: &str) -> IResult<&str, Command> {
    let (input, command) = alt((parse_cd, parse_ls))(input)?;
    Ok((input, command))
}

fn parse_cd(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("$ cd ")(input)?;
    let (input, path) = take_until("\n")(input)?;
    let result = match path {
        "/" => Command::Cd(Cd::Root),
        ".." => Command::Cd(Cd::Up),
        name => Command::Cd(Cd::Down(name)),
    };
    Ok((input, result))
}

fn parse_ls(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("$ ls")(input)?;
    let (input, _) = newline(input)?;
    let (input, file_or_dir) = separated_list1(newline, alt((parse_file, parse_dir)))(input)?;
    Ok((input, Command::Ls(file_or_dir)))
}

fn parse_dir(input: &str) -> IResult<&str, FsObject> {
    let (input, _) = tag("dir ")(input)?;
    let (input, name) = alpha1(input)?;
    Ok((input, FsObject::Dir(name)))
}

fn parse_file(input: &str) -> IResult<&str, FsObject> {
    let (input, (size, name)) =
        separated_pair(nom::character::complete::u32, tag(" "), take_until("\n"))(input)?;
    let result = FsObject::File { size, name };
    Ok((input, result))
}
