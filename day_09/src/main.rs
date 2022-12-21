use day_09::head_move::HeadMove;

fn main() {
    let input = include_str!("../input/day_09_sample.txt");
    let moves = HeadMove::parse_moves(input);
    
    println!("{moves:#?}");
}
