pub mod day_02 {
    use std::str::FromStr;

    use crate::day_02_types::{prs_move::Move, encrypted_opponent_move::EncryptedOpponentMove, encrypted_player_move::EncryptedPlayerMove, move_pair::MovePair};

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
                EncryptedPlayerMove::X => opponent_move.decrypt().get_losing_counter_move(),
                EncryptedPlayerMove::Y => opponent_move.decrypt().get_draw_counter_move(),
                EncryptedPlayerMove::Z => opponent_move.decrypt().get_winning_counter_move(),
            }
        };

        get_total_score(encrypted_input, decrypt_player_move)
    }

    fn get_total_score<F>(encrypted_input: String, decrypt_player_move: F) -> u32  
        where F: Fn(&EncryptedOpponentMove, &EncryptedPlayerMove) -> Move {
            
        encrypted_input
            .lines()
            .map(|line| line.split(' ').collect::<Vec<_>>())
            .map(|move_pair| (EncryptedOpponentMove::from_str(move_pair[0]).unwrap(), 
                              EncryptedPlayerMove::from_str(move_pair[1]).unwrap()))
            .map(|move_pair| MovePair::new(decrypt_player_move(&move_pair.0, &move_pair.1), move_pair.0.decrypt()))
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

        assert_eq!(super::day_02::get_total_score_1(input), 10595);
    }

    #[test]
    fn test_part_2_from_input() {
        let input = fs::read_to_string("input/day_02.txt").unwrap();

        assert_eq!(super::day_02::get_total_score_2(input), 9541);
    }
}
