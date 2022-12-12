use nom::{IResult, bytes::complete::tag, branch::alt, combinator::map};

use super::{path::Utf8PathBuf, ls::{Ls, parse_ls}, cd::{Cd, parse_cd}};

#[derive(Debug)]
pub enum Command {
    Ls,
    Cd(Utf8PathBuf),
}

impl From<Ls> for Command {
  fn from(_: Ls) -> Self {
      Command::Ls
  }
}

impl From<Cd> for Command {
  fn from(cd: Cd) -> Self {
      Command::Cd(cd.0)
  }
}

pub fn parse_command(i: &str) -> IResult<&str, Command> {
  let (i, _) = tag("$ ")(i)?;
  alt((map(parse_ls, Into::into), map(parse_cd, Into::into)))(i)
}
