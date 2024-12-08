advent_of_code::solution!(7);

// ##################### DEPENDENCIES ####################

// ##################### TYPE DEFS #####################

// ##################### HELPER FUNCTIONS ####################
fn concat_nums(a: u64, b: u64) -> u64 {
  a * 10u64.pow(b.ilog10() + 1) + b
}

// ##################### PART ONE #####################
pub fn part_one(input: &str) -> Option<u64> {
  let mut total_calibration: u64 = 0;
  let base: u64 = 2; // an explicit type is required
  let loop_base: usize = 2;

  // Iterate over lines
  'equations: for equation in input.lines() {
    // Get test value and operators
    let Some((test_val_str, operands_str)) = equation.split_once(':') else {
      panic!("No colon found")
    };
    let test_val: u64 = test_val_str.parse::<u64>().unwrap();
    let operands: Vec<u64> = operands_str
      .split_whitespace()
      .map(|n| n.parse::<u64>().unwrap())
      .collect();

    // Check for single operand
    if operands.len() == 1 {
      if operands[0] == test_val {
        total_calibration += test_val;
      }
      continue 'equations;
    }

    // Run through possible permutations, checking for equality with test value
    'permutations: for perm_idx in 0..(loop_base.pow((operands.len() as u32) - 1)) {
      // Deal with first operator explicitly
      let mut curr_test_val: u64;
      if (perm_idx as u64 % base) == 1 {
        curr_test_val = operands[0] * operands[1];
      } else {
        curr_test_val = operands[0] + operands[1];
      }

      // Apply rest of operators
      for o_idx in 1..(operands.len() - 1) {
        if (perm_idx as u64 / (base.pow(o_idx as u32))) % base == 1 {
          curr_test_val *= operands[o_idx + 1];
        } else {
          curr_test_val += operands[o_idx + 1];
        }

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

  Some(total_calibration)
}

// ##################### PART TWO ####################
pub fn part_two(input: &str) -> Option<u64> {
  let mut total_calibration: u64 = 0;
  let base: u64 = 3; // an explicit type is required
  let loop_base: usize = 3;

  // Iterate over lines
  'equations: for equation in input.lines() {
    // Get test value and operators
    let Some((test_val_str, operands_str)) = equation.split_once(':') else {
      panic!("No colon found")
    };
    let test_val: u64 = test_val_str.parse::<u64>().unwrap();
    let operands: Vec<u64> = operands_str
      .split_whitespace()
      .map(|n| n.parse::<u64>().unwrap())
      .collect();

    // Check for single operand
    if operands.len() == 1 {
      if operands[0] == test_val {
        total_calibration += test_val;
      }
      continue 'equations;
    }

    // Run through possible permutations, checking for equality with test value
    'permutations: for perm_idx in 0..(loop_base.pow((operands.len() as u32) - 1)) {
      // Deal with first operator explicitly
      let mut curr_test_val: u64 = match perm_idx as u64 % base {
        0 => operands[0] + operands[1],
        1 => operands[0] * operands[1],
        2 => concat_nums(operands[0], operands[1]),
        _ => panic!("shouldn't be here"),
      };

      // Apply rest of operators
      for o_idx in 1..(operands.len() - 1) {
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

  Some(total_calibration)
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
