advent_of_code::solution!(2);

use std::cmp::Ordering;

pub fn part_one(input: &str) -> Option<u32> {
  let mut num_safe = 0;

  // Iterate on lines
  'report: for report in input.lines() {
    // map line to vector of u32's
    let levels: Vec<i64> = report
      .split_whitespace()
      .map(|d| d.parse::<i64>().unwrap()) // Maps iter of strings to iter of u32
      .collect::<Vec<i64>>(); //Collects all elements of iter into vector

    // Setup initial check for level
    assert!(levels.len() > 1);
    let is_decreasing = match levels[0].cmp(&levels[1]) {
      Ordering::Less => true,
      Ordering::Greater => false,
      Ordering::Equal => continue 'report,
    };

    // Check if line is safe
    for l_idx in 0..levels.len()-1 {
      let diff = levels[l_idx] - levels[l_idx+1];
      if (diff == 0) || (is_decreasing != (diff < 0)) ||
        (diff.abs() > 3) || (diff.abs() < 1) {
        continue 'report;
      }
    }
    num_safe += 1;
  }
  Some(num_safe)
}

pub fn part_two(input: &str) -> Option<u32> {
  let mut num_safe = 0;

  // Iterate on lines
  'report: for report in input.lines() {
    // map line to vector of u32's
    let levels: Vec<i64> = report
      .split_whitespace()
      .map(|d| d.parse::<i64>().unwrap()) // Maps iter of strings to iter of u32
      .collect::<Vec<i64>>(); //Collects all elements of iter into vector

    // Find first broken index in line (if any)
    let mut is_safe: bool;
    let broken_left_idx: usize;
    (is_safe, broken_left_idx) = check_levels(&levels);

    // If no broken lines, increment and break
    if is_safe {
      // println!("Safe Initially\t\tlevels:{:?}", levels.clone());
      num_safe += 1;
      continue 'report;
    }

    // Try to fix a single broken level

    // Remove left of problem
    let mut levels_remove_left = levels.clone();
    levels_remove_left.remove(broken_left_idx);
    (is_safe, _ ) = check_levels(&levels_remove_left);
    // println!("\t...\tlevels_remove_left:{:?}", levels_remove_left.clone());
    if is_safe {
      // println!("Safe Remove Left\tlevels:{:?}\t\tlevels_remove_left:{:?}", levels.clone(), levels_remove_left);
      num_safe += 1;
      continue 'report;
    }

    // Remove right of problem
    let mut levels_remove_right = levels.clone();
    levels_remove_right.remove(broken_left_idx+1);
    (is_safe, _ ) = check_levels(&levels_remove_right);
    if is_safe {
      // println!("Safe Remove Right\tlevels:{:?}\t\tlevels_remove_right:{:?}", levels.clone(), levels_remove_right);
      num_safe += 1;
      continue 'report;
    }

    // Remove first element in case that was the problem
    let mut levels_remove_first = levels.clone();
    levels_remove_first.remove(0);
    (is_safe, _ ) = check_levels(&levels_remove_first);
    if is_safe {
      // println!("Safe Remove First\tlevels:{:?}\t\tlevels_remove_first:{:?}", levels.clone(), levels_remove_first);
      num_safe += 1;
      continue 'report;
    }


    // println!("Not Safe\t\tlevels:{:?}", levels);
  }
  Some(num_safe)
}

// Check if line is safe
fn check_levels(levels: &Vec<i64>) -> (bool, usize) {
  let is_decreasing: bool = match levels[0].cmp(&levels[1]) {
    Ordering::Less => true,
    Ordering::Greater => false,
    Ordering::Equal => return (false, 0),
  };
  for l_idx in 0..levels.len()-1 {
    let diff = levels[l_idx] - levels[l_idx+1];
    if (diff == 0) || (is_decreasing != (diff < 0)) ||
      (diff.abs() > 3) || (diff.abs() < 1) {
      return (false, l_idx);
    }
  }
  return (true, 0);
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
