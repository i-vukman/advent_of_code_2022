use nom::{IResult, combinator::map, sequence::preceded, bytes::complete::tag};

use super::path::{Utf8PathBuf, parse_path};

#[derive(Debug)]
pub struct Cd(pub Utf8PathBuf);

pub fn parse_cd(i: &str) -> IResult<&str, Cd> {
  map(preceded(tag("cd "), parse_path), Cd)(i)
}
