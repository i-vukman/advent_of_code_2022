pub mod day_06 {
    use std::collections::{VecDeque, HashMap};

    struct Buffer {
        chars: VecDeque<char>,
        char_count: HashMap<char, u32>,
        size: usize,
    }

    impl Buffer {
        fn new(size: usize) -> Buffer {
            if size == 0 {
                panic!("Buffer size can't be 0");
            }
            Buffer { chars: VecDeque::new(), char_count: HashMap::new(), size }
        }

        fn has_unique_chars(&mut self) -> bool {
            self.chars.len() == self.char_count.len()
        }

        fn push(&mut self, c: char) {
            if self.chars.len() == self.size {
                let popped = self.chars.pop_front().unwrap();
                *self.char_count.entry(popped).or_insert(1) -= 1;
                if self.char_count[&popped] == 0 {
                    self.char_count.remove(&popped);
                }
            }
            self.chars.push_back(c);
            *self.char_count.entry(c).or_insert(0) += 1;
        }

        fn is_full(&self) -> bool {
            self.chars.len() == self.size
        }
    }

    pub fn solve_part_1(input: &str) -> usize {
        get_position_of_nth_distinct_char(input, 4)
    }

    pub fn solve_part_2(input: &str) -> usize {
        get_position_of_nth_distinct_char(input, 14)
    }

    fn get_position_of_nth_distinct_char(input: &str, distinct_char_count: usize) -> usize {
        let iter = input.chars().enumerate();
        let mut buffer = Buffer::new(distinct_char_count);
        for (i, c) in iter {
            buffer.push(c);
            if buffer.has_unique_chars() && buffer.is_full() {
                return i + 1;
            };
        };
        panic!("Invalid input");
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    #[test]
    fn test_part_1() {
        let input = fs::read_to_string("input/day_06.txt").unwrap_or(String::from(""));

        assert_eq!(super::day_06::solve_part_1(&input), 1238);
    }

    #[test]
    fn test_part_2() {
        let input = fs::read_to_string("input/day_06.txt").unwrap_or(String::from(""));

        assert_eq!(super::day_06::solve_part_2(&input), 3037);
    }
}