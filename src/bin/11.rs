advent_of_code::solution!(11);

// ##################### DEPENDENCIES ####################

// ##################### TYPE DEFS #####################

// ##################### HELPER FUNCTIONS ####################
// Blink specified number of times
fn blink (stones: &mut Vec<usize>, blinks: usize) -> Option<usize> {
  for _ in 0..blinks {
    let mut s_idx: usize = 0;
    for _ in 0..stones.len() {
      let s = stones.get_mut(s_idx).unwrap();
      let s_curr = *s;

      // If the stone is engraved with the number `0`,
      // it is replaced by a stone engraved with the number `1`.
      if s_curr == 0 {
        *s = 1;
      }
      // If the stone is engraved with a number that has an *even* number of digits,
      // it is replaced by *two stones*. The left half of the digits are engraved on
      // the new left stone, and the right half of the digits are engraved on the new
      // right stone. (Don't keep leading zeros).
      else if (s_curr.ilog10() + 1) % 2 == 0 {
        let s_str = s_curr.to_string();
        let (l_str, r_str) = s_str.split_at(s_str.len() / 2);
        *s = r_str.parse::<usize>().unwrap();
        stones.insert(s_idx, l_str.parse::<usize>().unwrap());
        s_idx += 1;
      }
      // If none of the other rules apply, the stone is replaced by a new stone;
      // the old stone's number *multiplied by 2024* is engraved on the new stone.
      else {
        *s = s_curr * 2024;
      }
      s_idx += 1;
    }
  }
  Some(stones.len())
}

// ##################### PART ONE #####################
pub fn part_one(input: &str) -> Option<usize> {
  // Get input stones as a vector
  let mut stones: Vec<usize> = input
    .split_whitespace()
    .map(|number| number.parse::<usize>().unwrap())
    .collect::<Vec<usize>>();

  // Blink
  blink(&mut stones, 25)

}

// ##################### PART TWO ####################
pub fn part_two(input: &str) -> Option<usize> {
  // Get input stones as a vector
  let mut stones: Vec<usize> = input
    .split_whitespace()
    .map(|number| number.parse::<usize>().unwrap())
    .collect::<Vec<usize>>();

  // Blink
  blink(&mut stones, 75)
}

// ##################### TESTS ###################
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_one() {
    let result = part_one(&advent_of_code::template::read_file("examples", DAY));
    assert_eq!(result, Some(55312));
  }

  #[test]
  fn test_part_two() {
    let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    assert_eq!(result, None);
  }
}
