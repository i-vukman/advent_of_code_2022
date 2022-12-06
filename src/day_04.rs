pub mod day_04 {
    struct Assignment {
        from: u32,
        to: u32,
    }

    impl Assignment {
        pub fn new(from: u32, to: u32) -> Self {
            Assignment { from: from, to: to }
        }

        pub fn is_fully_overlapping(&self, other: &Assignment) -> bool {
            self.from >= other.from && self.to <= other.to ||
            other.from >= self.from && other.to <= self.to
        }

        pub fn is_overlapping(&self, other: &Assignment) -> bool {
            self.from <= other.to && self.to >= other.from
        }
    }

    pub fn count_fully_overlapping(input: &str) -> u32 {
        count_filtered_assingments(input, |assignment1, assignment2| assignment1.is_fully_overlapping(assignment2))
    }

    pub fn count_overlapping(input: &str) -> u32 {
        count_filtered_assingments(input, |assignment1, assignment2| assignment1.is_overlapping(assignment2))
    }

    fn count_filtered_assingments<F>(input: &str, filter: F) -> u32
        where F: Fn(&Assignment, &Assignment) -> bool {
        input
          .lines()
          .map(|l| l.split(',').collect::<Vec<_>>())
          .map(|assignment_pair| (assignment_pair[0].split('-').collect::<Vec<_>>(),
                                  assignment_pair[1].split('-').collect::<Vec<_>>()))
          .map(|assignment_pair| (Assignment::new(assignment_pair.0[0].parse().unwrap(), assignment_pair.0[1].parse().unwrap()),
                                  Assignment::new(assignment_pair.1[0].parse().unwrap(), assignment_pair.1[1].parse().unwrap())))
          .filter(|assignment_pair| filter(&assignment_pair.0, &assignment_pair.1))
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

        assert_eq!(super::day_04::count_fully_overlapping(&input), 550);
    }

    #[test]
    fn test_part_2() {
        let input = fs::read_to_string("input/day_04.txt").unwrap_or(String::from(""));

        assert_eq!(super::day_04::count_overlapping(&input), 931);
    }
}