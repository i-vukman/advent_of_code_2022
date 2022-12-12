pub fn sum_top_3(input: String) -> u32 {
  let mut elves: Vec<u32> = input
      .split("\n\n")
      .map(|elf_calories| elf_calories.lines()
                                      .map(|calories_item| calories_item.parse::<u32>().unwrap())
                                      .sum())
      .collect();

  elves.sort();
  elves.reverse();

  elves[..3].iter().sum::<u32>()
}

#[cfg(test)]
mod tests {
    use std::fs;

    #[test]
    fn test_from_input() {
        let input = fs::read_to_string("input/day_01.txt").unwrap();
        assert_eq!(super::sum_top_3(input), 193697);
    }
}