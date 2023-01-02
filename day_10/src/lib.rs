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

pub fn build_crt_output(instructions: &[Instruction]) -> Vec<Vec<char>> {
    let mut result = Vec::new();
    result.push(Vec::new());

    let mut current_pixel_position = -1;
    let mut sprite_position = 1;

    for instruction in instructions {
        match instruction {
            Instruction::Noop => {
                current_pixel_position += 1;

                if current_pixel_position >= sprite_position - 1 && current_pixel_position <= sprite_position + 1 {
                    result.last_mut().unwrap().push('#');
                } else {
                    result.last_mut().unwrap().push('.');
                }

                if current_pixel_position == 39 {
                    current_pixel_position = -1;
                    result.push(Vec::new());
                }
            },
            Instruction::AddX(value) => {
                current_pixel_position += 1;
                
                if current_pixel_position >= sprite_position - 1 && current_pixel_position <= sprite_position + 1 {
                    result.last_mut().unwrap().push('#');
                } else {
                    result.last_mut().unwrap().push('.');
                }

                if current_pixel_position == 39 {
                    current_pixel_position = -1;
                    result.push(Vec::new());
                }
                
                current_pixel_position += 1;
                
                if current_pixel_position >= sprite_position - 1 && current_pixel_position <= sprite_position + 1 {
                    result.last_mut().unwrap().push('#');
                } else {
                    result.last_mut().unwrap().push('.');
                }
                
                if current_pixel_position == 39 {
                    current_pixel_position = -1;
                    result.push(Vec::new());
                }

                sprite_position += value;
            }
        }
    }

    result
}