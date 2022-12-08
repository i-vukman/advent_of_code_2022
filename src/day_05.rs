pub mod day_05 {
    use std::{collections::VecDeque, usize};

    pub struct Supplies {
        crate_stacks: Vec<VecDeque<char>>
    }

    impl Supplies {
        fn new(crate_stacks: Vec<VecDeque<char>>) -> Supplies {
            Supplies { crate_stacks }
        }

        fn move_crates(&mut self, amount: u32, from: usize, to: usize) {
            for _ in 0..amount {
                let crate_from = self.crate_stacks[from - 1].pop_front().unwrap();
                self.crate_stacks[to - 1].push_front(crate_from);
            }
        }

        fn get_top_line(&self) -> String {
            self.crate_stacks
                .iter()
                .map(|stack| stack.front().unwrap_or(&' '))
                .collect()
        }
    }

    pub fn solve(input: &str) -> String {
        let mut supplies = parse_supplies(input);

        let mut iterator = input.lines().peekable();

        while !iterator.next().unwrap().is_empty() {}

        while !iterator.peek().is_none() {
            let line = iterator.next().unwrap();
            let move_input = line.split(' ').collect::<Vec<_>>();
            let move_input = (move_input[1], move_input[3], move_input[5]);
            supplies.move_crates(move_input.0.parse().unwrap(), move_input.1.parse().unwrap(), move_input.2.parse().unwrap());
        }

        supplies.get_top_line()
    }

    fn parse_supplies(input: &str) -> Supplies {
        let mut iterator = input.lines().peekable();
        let mut crate_stacks: Vec<VecDeque<char>> = Vec::new();

        while !iterator.peek().unwrap().is_empty() {
            for (i, c) in iterator.next().unwrap().chars().enumerate() {
                if c.is_alphabetic() {
                    let index = (i - 1) / 4;
                    if index >= crate_stacks.len() {
                        crate_stacks.resize(index + 1, VecDeque::new())
                    }
                    crate_stacks[index].push_back(c)
                }
            }
        }

        Supplies::new(crate_stacks)
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    #[test]
    pub fn test_part_1() {
        let input = fs::read_to_string("input/day_05.txt").unwrap_or(String::from(""));
        assert_eq!(super::day_05::solve(&input), String::from("FCVRLMVQP"));
    }
}