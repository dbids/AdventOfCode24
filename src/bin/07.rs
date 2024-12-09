advent_of_code::solution!(7);

// ##################### DEPENDENCIES ####################

// ##################### TYPE DEFS #####################

// ##################### HELPER FUNCTIONS ####################
fn concat_nums(a: u64, b: u64) -> u64 {
  a * 10u64.pow(b.ilog10() + 1) + b
}

fn test_input(input: &str, base: u64, loop_base:usize) -> u64 {
  let mut total_calibration: u64 = 0;
  // Iterate over lines
  for equation in input.lines() {
    // Get test value and operators
    let Some((test_val_str, operands_str)) = equation.split_once(':') else {
      panic!("No colon found")
    };
    let test_val: u64 = test_val_str.parse::<u64>().unwrap();
    let operands: Vec<u64> = operands_str
      .split_whitespace()
      .map(|n| n.parse::<u64>().unwrap())
      .collect();

    // Run through possible permutations, checking for equality with test value
    'permutations: for perm_idx in 0..(loop_base.pow((operands.len() as u32) - 1)) {
      // Apply operation
      let mut curr_test_val: u64 = operands[0];
      for o_idx in 0..(operands.len() - 1) {
        curr_test_val = match (perm_idx as u64 / (base.pow(o_idx as u32))) % base {
          0 => curr_test_val + operands[o_idx + 1],
          1 => curr_test_val * operands[o_idx + 1],
          2 => concat_nums(curr_test_val, operands[o_idx + 1]),
          _ => panic!("shouldn't be here"),
        };

        // Check for overflow and stop perm if reached
        if curr_test_val > test_val {
          continue 'permutations;
        }
      }

      // Check for equality
      if curr_test_val == test_val {
        total_calibration += test_val;
        break 'permutations;
      }
    }
  }
  total_calibration
}

// ##################### PART ONE #####################
pub fn part_one(input: &str) -> Option<u64> {
  let base: u64 = 2; // an explicit type is required
  let loop_base: usize = 2;

  return Some(test_input(&input, base, loop_base));
}

// ##################### PART TWO ####################
pub fn part_two(input: &str) -> Option<u64> {
  let base: u64 = 3; // an explicit type is required
  let loop_base: usize = 3;

  return Some(test_input(&input, base, loop_base));
}

// ##################### TESTS ###################
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_one() {
    let result = part_one(&advent_of_code::template::read_file("examples", DAY));
    assert_eq!(result, Some(3749));
  }

  #[test]
  fn test_part_two() {
    let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    assert_eq!(result, Some(11387));
  }
}
