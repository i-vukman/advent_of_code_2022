use day_07_take_02;

fn main() {
    let input = include_str!("../input/day_07_sample.txt");
    let lines = day_07_take_02::parse_lines(input);

    for line in lines {
        println!("{line:?}");
    }
}
