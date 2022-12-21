use day_08::parse_trees;

#[test]
fn test_total_trees_from_sample() {
    let input = include_str!("../input/day_08_sample.txt");
    let count = day_08::count_visible_trees(input);

    assert_eq!(count, 21);
}

#[test]
fn test_total_trees_from_input() {
    let input = include_str!("../input/day_08.txt");
    let count = day_08::count_visible_trees(input);

    assert_eq!(count, 1733);
}

#[test]
fn test_best_visibility_from_sample() {
    let input = include_str!("../input/day_08_sample.txt");
    let best = day_08::calculate_the_tree_with_best_visibility(&parse_trees(input));

    assert_eq!(8, best);
}

#[test]
fn test_best_visibility_from_input() {
    let input = include_str!("../input/day_08.txt");
    let best = day_08::calculate_the_tree_with_best_visibility(&parse_trees(input));

    assert_eq!(284648, best);
}
