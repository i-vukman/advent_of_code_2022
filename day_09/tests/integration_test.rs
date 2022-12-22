use day_09::head_move::HeadMove;

#[test]
fn test_unique_position_count_from_sample() {
    let input = include_str!("../input/day_09_sample.txt");
    let moves = HeadMove::parse_moves(input);
    let count = day_09::calculate_unique_position_count(&moves);

    assert_eq!(count, 13);
}

#[test]
fn test_unique_positions_from_input() {
    let input = include_str!("../input/day_09.txt");
    let moves = HeadMove::parse_moves(input);
    let count = day_09::calculate_unique_position_count(&moves);

    assert_eq!(count, 6044);
}
