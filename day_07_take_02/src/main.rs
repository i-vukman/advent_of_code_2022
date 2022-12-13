use day_07_take_02::{self, terminal_output::line::Line, file_system::file_system_node::{FileSystemTree}};

fn main() -> color_eyre::Result<()> {
    let input = include_str!("../input/day_07_sample.txt");
    let lines = Line::parse_lines(input);
    let root = FileSystemTree::build_from_lines(lines)?;

    println!("{:#?}", root);

    Ok(())
}
