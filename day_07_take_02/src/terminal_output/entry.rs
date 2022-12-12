use super::path::{Utf8PathBuf, parse_path};
use nom::{IResult, combinator::map, sequence::{separated_pair, preceded}, bytes::complete::tag, branch::alt};

#[derive(Debug)]
pub enum Entry {
    Dir(Utf8PathBuf),
    File(u64, Utf8PathBuf),
}

pub fn parse_entry(i: &str) -> IResult<&str, Entry> {
  let parse_file = map(
      separated_pair(nom::character::complete::u64, tag(" "), parse_path),
      |(size, path)| Entry::File(size, path),
  );
  let parse_dir = map(preceded(tag("dir "), parse_path), Entry::Dir);

  alt((parse_file, parse_dir))(i)
}
