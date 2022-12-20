#[test]
fn test_total_trees_from_sample() {
    let input = include_str!("../input/day_08_sample.txt");
    let count = day_08::count_visible_trees(input);

    assert_eq!(count, 21);
}