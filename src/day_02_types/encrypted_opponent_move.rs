use std::str::FromStr;

use super::prs_move::Move;

pub enum EncryptedOpponentMove {
  A,
  B,
  C,
}

impl FromStr for EncryptedOpponentMove {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
      match s {
          "A" => Ok(EncryptedOpponentMove::A),
          "B" => Ok(EncryptedOpponentMove::B),
          "C" => Ok(EncryptedOpponentMove::C),
          _ => Err(())
      }
  }
}

//TODO: define as function or strategy?
impl EncryptedOpponentMove {
  pub fn decrypt(&self) -> Move {
      match self {
        EncryptedOpponentMove::A => Move::Rock,
        EncryptedOpponentMove::B => Move::Paper,
        EncryptedOpponentMove::C => Move::Scissors,
      }
  }
}
