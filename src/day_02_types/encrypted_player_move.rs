use std::str::FromStr;

pub enum EncryptedPlayerMove {
    X,
    Y,
    Z,
}

impl FromStr for EncryptedPlayerMove {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(EncryptedPlayerMove::X),
            "Y" => Ok(EncryptedPlayerMove::Y),
            "Z" => Ok(EncryptedPlayerMove::Z),
            _ => Err(())
        }
    }
}
