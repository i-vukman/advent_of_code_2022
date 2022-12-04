pub mod day_02 {
    use std::str::FromStr;

    #[derive(PartialEq)]
    enum Move {
        Rock = 1,
        Paper = 2,
        Scissors = 3,
    }

    enum EncryptedOpponentMove {
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

    impl EncryptedOpponentMove {
        fn decrypt(&self) -> Move {
            match self {
              EncryptedOpponentMove::A => Move::Rock,
              EncryptedOpponentMove::B => Move::Paper,
              EncryptedOpponentMove::C => Move::Scissors,
            }
        }
    }

    enum EncryptedPlayerMove {
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

    pub fn get_total_score_1(encrypted_input: String) -> u32 {
        let decrypt_player_move = |_opponent_move: &EncryptedOpponentMove, player_move: &EncryptedPlayerMove| {
            match player_move {
                EncryptedPlayerMove::X => Move::Rock,
                EncryptedPlayerMove::Y => Move::Paper,
                EncryptedPlayerMove::Z => Move::Scissors,
            }
        };

        get_total_score(encrypted_input, decrypt_player_move)
    }

    pub fn get_total_score_2(encrypted_input: String) -> u32 {
        let decrypt_player_move = |opponent_move: &EncryptedOpponentMove, player_move: &EncryptedPlayerMove| {
            match player_move {
                EncryptedPlayerMove::X => get_losing_counter_move(&opponent_move.decrypt()),
                EncryptedPlayerMove::Y => get_draw_counter_move(&opponent_move.decrypt()),
                EncryptedPlayerMove::Z => get_winning_counter_move(&opponent_move.decrypt()),
            }
        };

        get_total_score(encrypted_input, decrypt_player_move)
    }

    fn get_winning_counter_move(m: &Move) -> Move {
        match m {
            Move::Paper => Move::Scissors,
            Move::Rock => Move::Paper,
            Move::Scissors => Move::Rock,
        }
    }

    fn get_draw_counter_move(m: &Move) -> Move {
        match m {
          Move::Paper => Move::Paper,
          Move::Rock => Move::Rock,
          Move::Scissors => Move::Scissors,
        }
    }

    fn get_losing_counter_move(m: &Move) -> Move {
        match m {
            Move::Paper => Move::Rock,
            Move::Rock => Move::Scissors,
            Move::Scissors => Move::Paper
        }
    }

    fn get_total_score<F>(encrypted_input: String, decrypt_player_move: F) -> u32  where 
        F: Fn(&EncryptedOpponentMove, &EncryptedPlayerMove) -> Move {
        encrypted_input
            .lines()
            .map(|line| line.split(' ').collect::<Vec<_>>())
            .map(|move_pair| (EncryptedOpponentMove::from_str(move_pair[0]).unwrap(), 
                            EncryptedPlayerMove::from_str(move_pair[1]).unwrap()))
            .map(|move_pair| (move_pair.0.decrypt(), decrypt_player_move(&move_pair.0, &move_pair.1)))
            .map(|move_pair| {
                let opponent_move = move_pair.0;
                let player_move = move_pair.1;

                match get_winning_move(&opponent_move, &player_move) {
                    Some(winning_move) => return if winning_move == player_move { 6 + player_move as u32 } else { 0 + player_move as u32 },
                    None => return 3 + player_move as u32
                }
        })
        .sum()
    }

    fn get_winning_move(first: &Move, second: &Move) -> Option<Move> {
        match first {
            Move::Paper => match second {
                Move::Paper => return None,
                Move::Rock => return Some(Move::Paper),
                Move::Scissors => return Some(Move::Scissors),
            },
            Move::Rock => match second {
                Move::Paper => return Some(Move::Paper),
                Move::Rock => return None,
                Move::Scissors => return Some(Move::Rock),
            },
            Move::Scissors => match second {
                Move::Paper => return Some(Move::Scissors),
                Move::Rock => return Some(Move::Rock),
                Move::Scissors => return None,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::day_02::{get_total_score_1, get_total_score_2};

    #[test]
    fn test_part_1_from_input() {
        let input = fs::read_to_string("input/day_02.txt").unwrap();

        assert_eq!(get_total_score_1(input), 10595);
    }

    #[test]
    fn test_part_2_from_input() {
        let input = fs::read_to_string("input/day_02.txt").unwrap();

        assert_eq!(get_total_score_2(input), 9541);
    }
}
