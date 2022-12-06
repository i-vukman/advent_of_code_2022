pub mod solution {
    use std::str::FromStr;

    use crate::day_02::{encrypted_opponent_move::EncryptedOpponentMove, encrypted_player_move::EncryptedPlayerMove, move_pair::MovePair, move_based_encrypted_player_move::MoveBasedEncryptedPlayerMove, outcume_based_encrypted_player_move::OutcomeBasedEncryptedPlayerMove};

    pub fn get_total_score_1(encrypted_input: String) -> u32 {
        get_total_score(encrypted_input, |value, _encrypted_opponent_move| MoveBasedEncryptedPlayerMove::new(value))
    }

    pub fn get_total_score_2(encrypted_input: String) -> u32 {
        get_total_score(encrypted_input, |value, encrypted_opponent_move| OutcomeBasedEncryptedPlayerMove::new(value, encrypted_opponent_move))
    }

    fn get_total_score<F, M>(encrypted_input: String, encrypted_player_move_factory: F) -> u32  
        where F: Fn(&str, EncryptedOpponentMove) -> M,
        M: EncryptedPlayerMove {
            
        encrypted_input
            .lines()
            .map(|line| line.split(' ').collect::<Vec<_>>())
            .map(|move_pair| (EncryptedOpponentMove::from_str(move_pair[0]).unwrap(), 
                              encrypted_player_move_factory(move_pair[1], EncryptedOpponentMove::from_str(move_pair[0]).unwrap())))
            .map(|move_pair| MovePair::new(move_pair.1.decrypt(), move_pair.0.decrypt()))
            .map(|move_pair| move_pair.calculate_score())
        .sum()
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    #[test]
    fn test_part_1_from_input() {
        let input = fs::read_to_string("input/day_02.txt").unwrap();

        assert_eq!(super::solution::get_total_score_1(input), 10595);
    }

    #[test]
    fn test_part_2_from_input() {
        let input = fs::read_to_string("input/day_02.txt").unwrap();

        assert_eq!(super::solution::get_total_score_2(input), 9541);
    }
}
