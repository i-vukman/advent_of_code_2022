use nom::{IResult, branch::alt, combinator::map};

use super::{command::{Command}, entry::Entry};

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
