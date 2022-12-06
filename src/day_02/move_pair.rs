use super::prs_move::Move;

pub struct MovePair {
    player_move: Move,
    opponent_move: Move,
}

impl MovePair {
    pub fn new(player_move: Move, opponent_move: Move) -> MovePair {
        MovePair { player_move, opponent_move }
    }

    pub fn calculate_score(&self) -> u32 {
        match &self.player_move {
            Move::Rock => match &self.opponent_move {
                Move::Rock => 3 + Move::Rock as u32,
                Move::Paper => 0 + Move::Rock as u32,
                Move::Scissors => 6 + Move::Rock as u32,
            },
            Move::Paper => match &self.opponent_move {
                Move::Rock => 6 + Move::Paper as u32,
                Move::Scissors => 0 + Move::Paper as u32,
                Move::Paper => 3 + Move::Paper as u32,
            },
            Move::Scissors => match &self.opponent_move {
                Move::Rock => 0 + Move::Scissors as u32,
                Move::Paper => 6 + Move::Scissors as u32,
                Move::Scissors => 3 + Move::Scissors as u32,
            }
        }
    }
}
