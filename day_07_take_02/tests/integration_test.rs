#[test]
fn test_total_small_files() {
    let input = include_str!("../input/day_07.txt");
    let result = day_07_take_02::calculate_total_size_of_dirs_smaller_than(input, 100000);
    assert_eq!(result, 1648397);
}

#[test]
fn test_smallest_file_to_delete() {
    //todo!()
}