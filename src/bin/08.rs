advent_of_code::solution!(8);

// ##################### DEPENDENCIES ####################
use std::collections::HashMap;

use advent_of_code::template::aoc_cli::check;

// ##################### TYPE DEFS #####################
#[derive(Debug, PartialEq, Eq, Clone)]
struct Point {
  x: usize,
  y: usize,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct SignedPoint {
  x: isize,
  y: isize,
}

// ##################### HELPER FUNCTIONS ####################
fn inside_bounds(x: isize, y: isize, x_max: usize, y_max: usize) -> bool {
  return !((x >= x_max as isize) || (y >= y_max as isize) || (y < 0) || (x < 0));
}

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
      if c != '.' && c != '\n' {
        //antennas.push(Antenna{x:l_idx, y:c_idx, freq:c,});
        antennas
          .entry(c)
          .or_insert_with(Vec::new)
          .push(Point { x: l_idx, y: c_idx });
      }
    }
  }
  //dbg!(antennas, height, width);

  // Determine antinodes
  let mut antinodes: Vec<Point> = Vec::new();
  for (_, point) in antennas.into_iter() {
    // Iterate through all pairs of points in the vector
    for (l, l_pt) in point.iter().enumerate() {
      for r_pt in point[l + 1..].iter() {
        // Find diff between points
        let diff_x: isize = r_pt.x as isize - l_pt.x as isize;
        let diff_y: isize = r_pt.y as isize - l_pt.y as isize;

        // Create check points
        let check_points: [SignedPoint; 2] = [
          SignedPoint {
            x: l_pt.x as isize - diff_x,
            y: l_pt.y as isize - diff_y,
          },
          SignedPoint {
            x: r_pt.x as isize + diff_x,
            y: r_pt.y as isize + diff_y,
          },
        ];
        check_points.into_iter().for_each(|cp| {
          if inside_bounds(cp.x, cp.y, height, width) {
            let antinode: Point = Point {
              x: cp.x as usize,
              y: cp.y as usize,
            };
            if !antinodes.contains(&antinode) {
              antinodes.push(antinode);
            }
          }
        })
      }
    }
  }

  Some(antinodes.len() as u32)
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
    assert_eq!(result, Some(14));
  }

  #[test]
  fn test_part_two() {
    let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    assert_eq!(result, None);
  }
}
