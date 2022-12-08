pub mod day_05 {
    use std::{collections::VecDeque, usize};

    pub struct Supplies {
        crate_stacks: Vec<VecDeque<char>>
    }

    impl Supplies {
        fn new(crate_stacks: Vec<VecDeque<char>>) -> Supplies {
            Supplies { crate_stacks }
        }

        fn move_crates_one_by_one(&mut self, amount: u32, from: usize, to: usize) {
            for _ in 0..amount {
                let crate_from = self.crate_stacks[from - 1].pop_front().unwrap();
                self.crate_stacks[to - 1].push_front(crate_from);
            }
        }

        fn move_crates_together(&mut self, amount: u32, from: usize, to: usize) {
            let mut temp = VecDeque::new();
            for _ in 0..amount {
                let crate_from = self.crate_stacks[from - 1].pop_front().unwrap();
                temp.push_front(crate_from);
            }
            
            while temp.front().is_some() {
                self.crate_stacks[to - 1].push_front(temp.pop_front().unwrap())
            }
        }

        fn get_top_line(&self) -> String {
            self.crate_stacks
                .iter()
                .map(|stack| stack.front().unwrap_or(&' '))
                .collect()
        }
    }

    pub fn solve_1(input: &str) -> String {
        let mut supplies = parse_supplies(input);

        parse_and_run_commands(input, |amount, from, to| supplies.move_crates_one_by_one(amount, from, to));

        supplies.get_top_line()
    }

    pub fn solve_2(input: &str) -> String {
        let mut supplies = parse_supplies(input);

        parse_and_run_commands(input, |amount, from, to| supplies.move_crates_together(amount, from, to));

        supplies.get_top_line()
    }

    fn parse_and_run_commands<F>(input: &str, mut command: F) 
        where F: FnMut(u32, usize, usize) -> ()  {
          
        let mut iterator = input.lines().peekable();
        
        while !iterator.next().unwrap().is_empty() {}

        while !iterator.peek().is_none() {
            let line = iterator.next().unwrap();
            let move_input = line.split(' ').collect::<Vec<_>>();
            command(move_input[1].parse().unwrap(), move_input[3].parse().unwrap(), move_input[5].parse().unwrap())
        }
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
        assert_eq!(super::day_05::solve_1(&input), String::from("FCVRLMVQP"));
    }

    #[test]
    pub fn test_part_2() {
        let input = fs::read_to_string("input/day_05.txt").unwrap_or(String::from(""));
        assert_eq!(super::day_05::solve_2(&input), String::from ("RWLWGJGFD"));
    }
}