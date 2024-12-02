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

    // Setup initial check for level
    let mut one_unsafe: bool = false;
    let mut change_direction: i32 = 0;
    assert!(levels.len() > 1);

    // Check if line is safe
    for l_idx in 0..levels.len()-1 {
      let diff = levels[l_idx] - levels[l_idx+1];

      // Special case first diff
      if l_idx == 0 {
        change_direction = match levels[l_idx].cmp(&levels[l_idx+1]) {
          Ordering::Less => -1,
          Ordering::Greater => 1,
          Ordering::Equal => {
            one_unsafe = true;
            0
          },
        };
        if ((diff.abs() > 3) || (diff.abs() < 1)) && (one_unsafe == false) {
          one_unsafe = true;
        }
      } else {

        // Special case second diff with zero first diff
        if (l_idx == 1) && (change_direction == 0) {
          change_direction = match levels[l_idx].cmp(&levels[l_idx+1]) {
            Ordering::Less => -1,
            Ordering::Greater => 1,
            Ordering::Equal => {
              one_unsafe = true;
              0
            },
          };
          if change_direction == 0 {
            continue 'report; // Unsafe
          }
        }

        // Do generalized check for every level
        if (diff == 0) || ((change_direction < 0) != (diff < 0)) ||
          (diff.abs() > 3) || (diff.abs() < 1) {
          if one_unsafe {
            continue 'report;
          } else {
            one_unsafe = true;
          }
        }
      }

      println!("report: {report}\tl_idx: {l_idx}\tone_unsafe: {one_unsafe}");
    }

    // If we made it through the whole level it is safe
    num_safe += 1;
  }
  Some(num_safe)
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
