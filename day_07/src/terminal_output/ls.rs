use nom::{IResult, combinator::map, bytes::complete::tag};

#[derive(Debug)]
pub struct Ls;

pub fn parse_ls(i: &str) -> IResult<&str, Ls> {
    map(tag("ls"), |_| Ls)(i)
}
