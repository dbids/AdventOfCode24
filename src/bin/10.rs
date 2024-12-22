advent_of_code::solution!(10);

// ##################### DEPENDENCIES ####################
use self::Direction::*;
use ndarray::Array2;
use std::collections::HashSet;
use std::slice::Iter;

// ##################### TYPE DEFS #####################
// Create direction enum, with methods for x/y traversing directions
#[derive(Clone, Copy, Debug)]
enum Direction {
  Left,
  Right,
  Up,
  Down,
}
impl Direction {
  fn get_x_inc(&self) -> isize {
    match &self {
      Direction::Down => 1,
      Direction::Up => -1,
      _ => 0,
    }
  }

  fn get_y_inc(&self) -> isize {
    match &self {
      Direction::Left => -1,
      Direction::Right => 1,
      _ => 0,
    }
  }
  pub fn iterator() -> Iter<'static, Direction> {
    static DIRECTIONS: [Direction; 4] =
      [Left, Right, Up, Down];
    DIRECTIONS.iter()
  }
}

// ##################### HELPER FUNCTIONS ####################
// Get input map and trailheads
fn parse_input(input : &str) -> (Array2<u8>, Vec<(isize, isize)>){
  // Determine array dimensions
  let mut height: usize = 0;
  let mut width: usize = 0;
  for (l_idx, line) in input.lines().enumerate() {
    width += 1;
    if l_idx == 0 {
      height = line.len();
    }
  }

  // Get input as a 2-d vector
  let mut input_array = Array2::from_elem((height, width), 0);
  let mut trailheads: Vec<(isize, isize)> = Vec::new();
  for (l_idx, line) in input.lines().enumerate() {
    for (c_idx, c) in line.chars().enumerate() {
      let c_num: u8 = c.to_digit(10).unwrap() as u8;
      input_array[[l_idx, c_idx]] = c_num;
      if c_num == 0 {
        trailheads.push((l_idx as isize, c_idx as isize));
      }
    }
  }

  return (input_array, trailheads);
}

// Traverse map recursively
fn traverse_map(x: isize, y: isize, in_dir: Direction, map: &Array2<u8>, trail_ends: &mut HashSet<(isize, isize)>) {
  // Get next x and y and check zero
  let x_inc: isize = in_dir.get_x_inc();
  let y_inc: isize = in_dir.get_y_inc();
  let Some(next_x) = x.checked_add(x_inc) else {
    return;
  };
  let Some(next_y) = y.checked_add(y_inc) else {
    return;
  };

  // Check array bounds and get next spot
  let Some(&next_num) = map.get((next_x as usize, next_y as usize)) else {
    return;
  };

  // Check for strict increase
  let Some(curr_num) = map.get((x as usize, y as usize)) else {
    return;
  };
  if let Some(sub_num) = next_num.checked_sub(*curr_num) {
    if sub_num != 1 {
      return;
    }
  } else {
    return;
  }

  // If next num is 9 return x and y
  if next_num == 9 {
    trail_ends.insert((next_x, next_y));
    return;
  } else {
    for &direction in Direction::iterator() {
      traverse_map(next_x, next_y, direction, &map, trail_ends);
    }
  }
}


// ##################### PART ONE #####################
pub fn part_one(input: &str) -> Option<usize> {
  // Get input as an array of u8 and list of trailheads
  let (map, trailheads) = parse_input(&input);

  // Iterate over trailheads and calculate the score for each
  let mut sum_trailhead_scores: usize = 0;
  for th in trailheads.into_iter() {
    let mut trail_ends: HashSet<(isize, isize)> = HashSet::new();

    for &direction in Direction::iterator() {
      traverse_map(th.0, th.1, direction,  &map, &mut trail_ends);
    }

    sum_trailhead_scores += trail_ends.len();
  }

  Some(sum_trailhead_scores)
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
    assert_eq!(result, Some(36));
  }

  #[test]
  fn test_part_two() {
    let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    assert_eq!(result, None);
  }
}
