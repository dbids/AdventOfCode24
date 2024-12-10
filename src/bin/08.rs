advent_of_code::solution!(8);

// ##################### DEPENDENCIES ####################
 use std::collections::HashMap;

 // ##################### TYPE DEFS #####################
#[derive(Debug, PartialEq, Eq)]
struct Point {
	x: usize,
	y: usize,
}

// ##################### HELPER FUNCTIONS ####################

// ##################### PART ONE #####################
pub fn part_one(input: &str) -> Option<u32> {
  // Get Antennas from input string
	let mut width: usize = 0;
	let mut height: usize = 0;
	let mut antennas: HashMap<char, Vec<Point>> = HashMap::new();
	for (l_idx, line) in input.lines().enumerate() {
		width = line.len();
		height += 1;
		for (c_idx, c) in line.chars().enumerate() {
			if c != '.' && c !='\n' {
				//antennas.push(Antenna{x:l_idx, y:c_idx, freq:c,});
        antennas.entry(c)
          .or_insert_with(Vec::new)
          .push(Point{x:l_idx, y:c_idx,});
			}
		}
	}
	dbg!(antennas, height, width);

	None
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
    assert_eq!(result, None);
  }

  #[test]
  fn test_part_two() {
    let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    assert_eq!(result, None);
  }
}
