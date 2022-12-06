pub mod day_04 {
    struct Assignment {
        from: u32,
        to: u32,
    }

    impl Assignment {
        pub fn new(from: u32, to: u32) -> Self {
            Assignment { from: from, to: to }
        }

        pub fn does_fully_owerlap_with(&self, other: &Assignment) -> bool {
            self.from >= other.from && self.to <= other.to ||
            other.from >= self.from && other.to <= self.to
        }
    }

    pub fn count_fully_owerlapping_assingments(input: &str) -> u32 {
        input
          .lines()
          .map(|l| l.split(',').collect::<Vec<_>>())
          .map(|assignment_pair| (assignment_pair[0].split('-').collect::<Vec<_>>(),
                    assignment_pair[1].split('-').collect::<Vec<_>>()))
          .map(|assignment_pair| (Assignment::new(assignment_pair.0[0].parse().unwrap(), assignment_pair.0[1].parse().unwrap()),
                                 Assignment::new(assignment_pair.1[0].parse().unwrap(), assignment_pair.1[1].parse().unwrap())))
          .filter(|assignment_pair| assignment_pair.0.does_fully_owerlap_with(&assignment_pair.1))
          .count()
          .try_into()
          .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    #[test]
    fn test_part_1() {
        let input = fs::read_to_string("input/day_04.txt").unwrap_or(String::from(""));

        assert_eq!(super::day_04::count_fully_owerlapping_assingments(&input), 550);
    }
}