use std::fs;

fn main() {
    let input = fs::read_to_string("input/day_01.txt").unwrap();
    println!("{}", day_01::sum_top_3(input));
}
