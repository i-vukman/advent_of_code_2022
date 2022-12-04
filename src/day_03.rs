pub mod day_03 {
    use std::collections::HashSet;

    pub fn sum_priorities(input: String) -> u32 {
        input
            .lines()
            .map(|line| {
                let line_split = line.split_at(line.len() / 2);
                let mut first_half_chars = HashSet::new();

                line_split.0.chars().for_each(|char| {
                    first_half_chars.insert(char);
                });

                for c in line_split.1.chars() {
                    if first_half_chars.contains(&c) {
                        return if c as u32 >= 97 { c as u32 - 96 } else { c as u32 - 38 };
                    }
                };
                
                0
            })
            .sum()
    }
}

#[cfg(test)]
pub mod tests {
    use std::fs;

    use super::day_03::sum_priorities;

    #[test]
    fn test_input() {
        let input = fs::read_to_string("input/day_03.txt").unwrap();
        assert_eq!(sum_priorities(input), 7917);
    }
}