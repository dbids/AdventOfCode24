advent_of_code::solution!(11);

// ##################### DEPENDENCIES ####################

// ##################### TYPE DEFS #####################

// ##################### HELPER FUNCTIONS ####################
// Blink specified number of times
fn blink(stone: usize, curr_blink: usize, max_blinks: usize) -> usize {
  let mut sum_blinks = 0;

  if curr_blink != (max_blinks) {
    // If the stone is engraved with the number `0`,
    // it is replaced by a stone engraved with the number `1`.
    if stone == 0 {
      sum_blinks += blink(1, curr_blink + 1, max_blinks);
    }
    // If the stone is engraved with a number that has an *even* number of digits,
    // it is replaced by *two stones*. The left half of the digits are engraved on
    // the new left stone, and the right half of the digits are engraved on the new
    // right stone. (Don't keep leading zeros).
    else if (stone.ilog10() + 1) % 2 == 0 {
      let s_str = stone.to_string();
      let (l_str, r_str) = s_str.split_at(s_str.len() / 2);
      sum_blinks += blink(l_str.parse::<usize>().unwrap(), curr_blink + 1, max_blinks);
      sum_blinks += blink(r_str.parse::<usize>().unwrap(), curr_blink + 1, max_blinks);
    }
    // If none of the other rules apply, the stone is replaced by a new stone;
    // the old stone's number *multiplied by 2024* is engraved on the new stone.
    else {
      sum_blinks += blink(stone * 2024, curr_blink + 1, max_blinks);
    }
  } else {
    return 1;
  }

  return sum_blinks;
}

// ##################### PART ONE #####################
pub fn part_one(input: &str) -> Option<usize> {
  // Get input stones as a vector
  let stones: Vec<usize> = input
    .split_whitespace()
    .map(|number| number.parse::<usize>().unwrap())
    .collect::<Vec<usize>>();

  // Blink
  Some(
    stones
      .into_iter()
      .map(|stone| blink(stone, 0, 25))
      .sum::<usize>(),
  )
}

// ##################### PART TWO ####################
pub fn part_two(input: &str) -> Option<usize> {
  // Get input stones as a vector
  let stones: Vec<usize> = input
    .split_whitespace()
    .map(|number| number.parse::<usize>().unwrap())
    .collect::<Vec<usize>>();

  // Blink
  Some(
    stones
      .into_iter()
      .map(|stone| blink(stone, 0, 75))
      .sum::<usize>(),
  )
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
