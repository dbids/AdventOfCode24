advent_of_code::solution!(9);

// ##################### DEPENDENCIES ####################

// ##################### TYPE DEFS #####################
#[derive(Debug)]
struct Metadata {
  id_num: usize,
  start: usize,
  end: usize,
}
// ##################### HELPER FUNCTIONS ####################

// ##################### PART ONE #####################
pub fn part_one(input: &str) -> Option<u32> {
  let mut files: Vec<Metadata> = Vec::new();
  let mut spaces: Vec<Metadata> = Vec::new();
  let mut counter: usize = 0;
  input.chars().map(|c| c.to_digit(10).unwrap()).enumerate().for_each(|(c_idx, c)| {
    if c_idx % 2 {
      files.push(Metadata{id_num: c_idx/2, start:counter, end:counter+c-1});
    } else {
      spaces.push(Metadata{id_num: 0, start:counter, end:counter+c-1});
    }
    counter += c;
  })


}

// ##################### PART TWO ####################
pub fn part_two(input: &str) -> Option<u32> {
  None
}

// ##################### TESTS ###################
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_one() {
    let result = part_one(&advent_of_code::template::read_file("examples", DAY));
    assert_eq!(result, Some(1928));
  }

  #[test]
  fn test_part_two() {
    let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    assert_eq!(result, None);
  }
}
