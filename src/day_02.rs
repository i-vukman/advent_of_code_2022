pub mod day_02 {
    use std::str::FromStr;

    #[derive(PartialEq)]
    enum Move {
        Rock = 1,
        Paper = 2,
        Scissors = 3,
    }

    trait EncryptedMove {
        fn decrypt(&self) -> Move;
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

    impl EncryptedMove for EncryptedOpponentMove {
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

    impl EncryptedMove for EncryptedPlayerMove {
        fn decrypt(&self) -> Move {
            match self {
                EncryptedPlayerMove::X => Move::Rock,
                EncryptedPlayerMove::Y => Move::Paper,
                EncryptedPlayerMove::Z => Move::Scissors,
            }
        }
    }

    pub fn get_total_score(encrypted_input: String) -> u32 {
        encrypted_input
            .lines()
            .map(|line| line.split(' ').collect::<Vec<_>>())
            .map(|move_pair| (EncryptedOpponentMove::from_str(move_pair[0]).unwrap(), 
                            EncryptedPlayerMove::from_str(move_pair[1]).unwrap()))
            .map(|move_pair| (move_pair.0.decrypt(), move_pair.1.decrypt()))
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

    use super::day_02::get_total_score;

    #[test]
    fn test_from_input() {
        let input = fs::read_to_string("input/day_02.txt").unwrap();

        assert_eq!(get_total_score(input), 10595);
    }
}
