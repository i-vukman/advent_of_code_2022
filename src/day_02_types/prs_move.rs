#[derive(PartialEq)]
pub enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Move {
    pub fn get_winning_counter_move(&self) -> Move {
        match self {
            Move::Paper => Move::Scissors,
            Move::Rock => Move::Paper,
            Move::Scissors => Move::Rock,
        }
    }

    pub fn get_draw_counter_move(&self) -> Move {
        match self {
          Move::Paper => Move::Paper,
          Move::Rock => Move::Rock,
          Move::Scissors => Move::Scissors,
        }
    }

    pub fn get_losing_counter_move(&self) -> Move {
        match self {
            Move::Paper => Move::Rock,
            Move::Rock => Move::Scissors,
            Move::Scissors => Move::Paper
        }
    }
}
