advent_of_code::solution!(2);

use std::cmp::Ordering;

pub fn part_one(input: &str) -> Option<u32> {
  let mut numSafe = 0;

  // Iterate on lines
  'report: for report in input.lines() {
    // map line to vector of u32's
    let levels: Vec<i64> = report
      .split_whitespace()
      .map(|d| d.parse::<i64>().unwrap()) // Maps iter of strings to iter of u32
      .collect::<Vec<i64>>(); //Collects all elements of iter into vector

    // Check if line is safe
    assert!(levels.len() > 1);
    let is_decreasing = match levels[0].cmp(&levels[1]) {
      Ordering::Less => true,
      Ordering::Greater => false,
      Ordering::Equal => continue 'report,
    };
    for l_idx in 0..levels.len()-1 {
      let diff = levels[l_idx] - levels[l_idx+1];
      if (diff == 0) || (is_decreasing != (diff < 0)) ||
        (diff.abs() > 3) || (diff.abs() < 1) {
        continue 'report;
      }
    }
    numSafe += 1;
  }
  Some(numSafe)
}

pub fn part_two(input: &str) -> Option<u32> {
  None
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_one() {
    let result = part_one(&advent_of_code::template::read_file("examples", DAY));
    assert_eq!(result, Some(2));
  }

  #[test]
  fn test_part_two() {
    let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    assert_eq!(result, Some(4));
  }
}
