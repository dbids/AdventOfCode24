advent_of_code::solution!(8);

// ##################### DEPENDENCIES ####################
use std::collections::HashMap;

use advent_of_code::template::aoc_cli::check;

// ##################### TYPE DEFS #####################
#[derive(Debug, PartialEq, Eq, Clone)]
struct Point {
  x: isize,
  y: isize,
}
impl Point {
  fn inside_bounds(&self, x_max: isize, y_max: isize) -> bool {
    return !((self.x >= x_max) || (self.y >= y_max) || (self.y < 0) || (self.x < 0));
  }
}

// ##################### HELPER FUNCTIONS ####################
fn format_input(
  input: &str,
  width: &mut isize,
  height: &mut isize,
  antennas: &mut HashMap<char, Vec<Point>>,
) {
  for (l_idx, line) in input.lines().enumerate() {
    *width = line.len() as isize;
    *height += 1;
    for (c_idx, c) in line.chars().enumerate() {
      if c != '.' && c != '\n' {
        antennas.entry(c).or_insert_with(Vec::new).push(Point {
          x: l_idx as isize,
          y: c_idx as isize,
        });
      }
    }
  }
}

// ##################### PART ONE #####################
pub fn part_one(input: &str) -> Option<usize> {
  // Get Antennas from input string
  let mut width: isize = 0;
  let mut height: isize = 0;
  let mut antennas: HashMap<char, Vec<Point>> = HashMap::new();
  format_input(input, &mut width, &mut height, &mut antennas);

  // Determine antinodes
  let mut antinodes: Vec<Point> = Vec::new();
  for (_, point) in antennas.into_iter() {
    // Iterate through all pairs of points in the vector
    for (l, l_pt) in point.iter().enumerate() {
      for r_pt in point[l + 1..].iter() {
        // Find diff between points
        let diff_x: isize = r_pt.x - l_pt.x;
        let diff_y: isize = r_pt.y - l_pt.y;

        // Create check points
        let check_points: [Point; 2] = [
          Point {
            x: l_pt.x - diff_x,
            y: l_pt.y - diff_y,
          },
          Point {
            x: r_pt.x + diff_x,
            y: r_pt.y + diff_y,
          },
        ];
        check_points.into_iter().for_each(|cp| {
          if cp.inside_bounds(height, width) {
            let antinode: Point = Point { x: cp.x, y: cp.y };
            if !antinodes.contains(&antinode) {
              antinodes.push(antinode);
            }
          }
        })
      }
    }
  }

  Some(antinodes.len())
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
