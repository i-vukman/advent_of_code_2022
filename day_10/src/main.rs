use day_10::instruction::Instruction;

fn main() {
    let input = include_str!("../input/sample.txt");
    let instructions = Instruction::parse_instructions(input); 
    println!("{instructions:#?}");
}
