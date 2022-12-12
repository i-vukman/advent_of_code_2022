use super::prs_move::Move;

pub trait EncryptedPlayerMove {
    fn decrypt(&self) -> Move;
}
