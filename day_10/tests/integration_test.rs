use day_10::instruction::Instruction;

#[test]
fn test_signal_strenght_from_sample() {
    let input = include_str!("../input/sample.txt");
    let instructions = Instruction::parse_instructions(input);
    let sum = day_10::sum_signal_strenghts(&instructions);

    assert_eq!(sum, 13140);
}

#[test]
fn test_signal_strength_from_input() {
    let input = include_str!("../input/day_10.txt");
    let instructions = Instruction::parse_instructions(input);
    let sum = day_10::sum_signal_strenghts(&instructions);

    assert_eq!(sum, 12980);
}