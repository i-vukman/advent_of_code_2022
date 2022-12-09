pub mod day_06 {
    use std::collections::{VecDeque, HashSet, HashMap};

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
                let poped = self.chars.pop_front().unwrap();
                *self.char_count.entry(poped).or_insert(1) -= 1;
                if self.char_count[&poped] == 0 {
                    self.char_count.remove(&poped);
                }
            }
            self.chars.push_back(c);
            *self.char_count.entry(c).or_insert(0) += 1;
        }

        fn is_full(&self) -> bool {
            self.chars.len() == self.size
        }
    }

    pub fn solve(input: &str) -> usize {
        let iter = input.chars().enumerate();
        let mut buffer = Buffer::new(4);
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

        assert_eq!(super::day_06::solve(&input), 1238);
    }
}