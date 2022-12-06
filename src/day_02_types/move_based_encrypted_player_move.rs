use super::{encrypted_player_move::EncryptedPlayerMove, prs_move::Move};

pub enum MoveBasedEncryptedPlayerMove {
    X,
    Y,
    Z,
}

impl EncryptedPlayerMove for MoveBasedEncryptedPlayerMove {
    fn decrypt(&self) -> Move {
        match self {
            MoveBasedEncryptedPlayerMove::X => Move::Rock,
            MoveBasedEncryptedPlayerMove::Y => Move::Paper,
            MoveBasedEncryptedPlayerMove::Z => Move::Scissors,
        }
    }
}

impl MoveBasedEncryptedPlayerMove {
    pub fn new(value: &str) -> MoveBasedEncryptedPlayerMove {
        match value {
            "X" => MoveBasedEncryptedPlayerMove::X,
            "Y" => MoveBasedEncryptedPlayerMove::Y,
            "Z" => MoveBasedEncryptedPlayerMove::Z,
            _ => panic!("Can't parse {value} to MoveBasedEncryptedPlayerMove"),
        }
    }
}
