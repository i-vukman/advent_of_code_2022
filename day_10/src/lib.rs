use instruction::Instruction;

pub mod instruction;

const PART_1_CYCLES: [i64; 6] = [20, 60, 100, 140, 180, 220];

pub fn sum_signal_strenghts(instructions: &[Instruction]) -> i64 {
    let mut current_cycle: i64 = 0;
    let mut x = 1;
    let mut sum = 0;
    for instruction in instructions {
        match instruction {
            Instruction::Noop => {
                current_cycle += 1;
                if PART_1_CYCLES.contains(&current_cycle) {
                    sum += x * current_cycle;
                }
            },
            Instruction::AddX(value) => {
                current_cycle += 1;
                if PART_1_CYCLES.contains(&current_cycle) {
                    sum += x * current_cycle;
                }
                current_cycle += 1;
                if PART_1_CYCLES.contains(&current_cycle) {
                    sum += x * current_cycle;
                }
                x += value;
            }
        }
    }
    sum
}