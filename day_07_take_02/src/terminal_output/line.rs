use nom::{
    branch::alt,
    combinator::{all_consuming, map},
    IResult, Finish,
};

use super::{command::Command, entry::Entry};

#[derive(Debug)]
pub enum Line {
    Command(Command),
    Entry(Entry),
}

pub fn parse_line(i: &str) -> IResult<&str, Line> {
    alt((
        map(super::command::parse_command, Line::Command),
        map(super::entry::parse_entry, Line::Entry),
    ))(i)
}

impl Line {
    pub fn parse_lines(input: &str) -> Vec<Line> {
        input
            .lines()
            .map(|l| all_consuming(parse_line)(l).finish().unwrap().1)
            .collect::<Vec<_>>()
    }
}
