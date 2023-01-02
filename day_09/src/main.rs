use day_09::head_move::HeadMove;

fn main() {
    let input = include_str!("../input/day_09.txt");
    let moves = HeadMove::parse_moves(input);
    
    println!("{:#?}", day_09::calculate_unique_position_count(&moves, 2));
}
