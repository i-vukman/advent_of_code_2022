use day_10::{instruction::Instruction, build_crt_output};

fn main() {
    let input = include_str!("../input/sample.txt");
    let instructions = Instruction::parse_instructions(input); 
    let output = build_crt_output(&instructions);
    for line in output {
        println!("{:?}", line.iter().collect::<String>());
    }
}
