use super::{encrypted_player_move::EncryptedPlayerMove, encrypted_opponent_move::EncryptedOpponentMove};

pub enum OutcomeBasedEncryptedPlayerMove {
    X(EncryptedOpponentMove),
    Y(EncryptedOpponentMove),
    Z(EncryptedOpponentMove),
}

impl EncryptedPlayerMove for OutcomeBasedEncryptedPlayerMove {
    fn decrypt(&self) -> super::prs_move::Move {
        match self {
            OutcomeBasedEncryptedPlayerMove::X(opponent_move) => opponent_move.decrypt().get_losing_counter_move(),
            OutcomeBasedEncryptedPlayerMove::Y(opponent_move) => opponent_move.decrypt().get_draw_counter_move(),
            OutcomeBasedEncryptedPlayerMove::Z(opponent_move) => opponent_move.decrypt().get_winning_counter_move(),
        }
    }
}

impl OutcomeBasedEncryptedPlayerMove {
    pub fn new(value: &str, opponent_move: EncryptedOpponentMove) -> OutcomeBasedEncryptedPlayerMove {
        match value {
            "X" => OutcomeBasedEncryptedPlayerMove::X(opponent_move),
            "Y" => OutcomeBasedEncryptedPlayerMove::Y(opponent_move),
            "Z" => OutcomeBasedEncryptedPlayerMove::Z(opponent_move),
            _ => panic!("Can't parse {value} to OutcomeBasedEncryptedPlayerMove")
        }
    }
}
