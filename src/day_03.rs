pub mod day_03 {
    use std::collections::HashSet;

    pub fn sum_priorities_01(input: String) -> u32 {
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
                }

                0
            })
            .sum()
    }

    pub fn sum_badges_02(input: String) -> u32 {
        let mut lines_iterator = input.lines().into_iter().peekable();
        let mut total = 0;
        
        while lines_iterator.peek().is_some() {
            let first = lines_iterator.next().unwrap_or("");
            let second = lines_iterator.next().unwrap_or("");
            let third = lines_iterator.next().unwrap_or("");

            let mut first_char_set = HashSet::new();
            let mut second_char_set = HashSet::new();
            let mut third_char_set = HashSet::new();

            first.chars().for_each(|c| { first_char_set.insert(c); });
            second.chars().for_each(|c| { second_char_set.insert(c); });
            third.chars().for_each(|c| { third_char_set.insert(c); });

            let first_intersection = first_char_set.intersection(&second_char_set).copied().collect::<HashSet<_>>();
            let second_intersection = first_intersection.intersection(&third_char_set).copied().collect::<HashSet<_>>();

            match second_intersection.iter().next() {
                Some(c) => total += if *c as u32 >= 97 { *c as u32 - 96 } else { *c as u32 - 38 },
                None => ()
            }
        }

        total
    }
}

#[cfg(test)]
pub mod tests {
    use std::fs;

    use super::day_03::{sum_priorities_01, sum_badges_02};

    #[test]
    fn test_input_1() {
        let input = fs::read_to_string("input/day_03.txt").unwrap();
        assert_eq!(sum_priorities_01(input), 7917);
    }

    #[test]
    fn test_input_2() {
        let input = fs::read_to_string("input/day_03.txt").unwrap();
        assert_eq!(sum_badges_02(input), 2585);
    }
}