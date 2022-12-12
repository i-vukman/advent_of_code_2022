pub use camino::Utf8PathBuf;
use nom::{IResult, combinator::map, bytes::complete::take_while1};

pub fn parse_path(i: &str) -> IResult<&str, Utf8PathBuf> {
    map(
        take_while1(|c: char| "abcdefghijklmnopqrstuvwxyz./".contains(c)),
        Into::into,
    )(i)
}
