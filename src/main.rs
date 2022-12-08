pub mod day_01;
pub mod day_03;
pub mod day_04;
pub mod day_05;

mod day_02 {
    pub mod solution;
    pub mod prs_move;
    pub mod encrypted_opponent_move;
    pub mod encrypted_player_move;
    pub mod move_pair;
    pub mod move_based_encrypted_player_move;
    pub mod outcume_based_encrypted_player_move;
}

fn main() {
    println!("All challenges are in their own module covered with unit test.");
}
