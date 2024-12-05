advent_of_code::solution!(4);

// Solution dependencies
use self::Direction::*;
use ndarray::Array2;
use std::slice::Iter;

// Create direction enum, with methods for x/y traversing directions
#[derive(Debug)]
enum Direction {
  Left,
  Right,
  Up,
  Down,
  UpLeft,
  UpRight,
  DownLeft,
  DownRight,
}
impl Direction {
  fn get_x_inc(&self) -> i32 {
    match &self {
      Direction::Down | Direction::DownLeft | Direction::DownRight => 1,
      Direction::Up | Direction::UpLeft | Direction::UpRight => -1,
      _ => 0,
    }
  }

  fn get_y_inc(&self) -> i32 {
    match &self {
      Direction::Left | Direction::DownLeft | Direction::UpLeft => -1,
      Direction::Right | Direction::DownRight | Direction::UpRight => 1,
      _ => 0,
    }
  }

  pub fn iterator() -> Iter<'static, Direction> {
    static DIRECTIONS: [Direction; 8] =
      [Left, Right, Up, Down, UpLeft, UpRight, DownLeft, DownRight];
    DIRECTIONS.iter()
  }

  pub fn iterator_diag() -> Iter<'static, Direction> {
    static DIRECTIONS: [Direction; 4] = [UpLeft, UpRight, DownLeft, DownRight];
    DIRECTIONS.iter()
  }
}

// Get next letter
fn next_letter(curr_letter: char) -> Option<char> {
  match curr_letter {
    'X' => Some('M'),
    'M' => Some('A'),
    'A' => Some('S'),
    _ => None,
  }
}

// Add usize to i32
fn add_usize_i32(u: usize, i: i32) -> Option<usize> {
  if i.is_negative() {
    u.checked_sub(i.wrapping_abs() as u32 as usize)
  } else {
    u.checked_add(i as usize)
  }
}

// Test XMAS for part 1
fn test_xmas(x: usize, y: usize, ia: &Array2<char>) -> u32 {
  // Check all directions
  let mut num_xmas: u32 = 0;
  'for_direction: for direction in Direction::iterator() {
    // Setup direction vars
    let mut next_x: usize = x;
    let mut next_y: usize = y;
    let x_inc: i32 = direction.get_x_inc();
    let y_inc: i32 = direction.get_y_inc();

    // Loop until s
    for _ in 0..3 {
      // Get next letter in sequence based on current letter
      let Some(next_letter) = next_letter(ia[[next_x, next_y]]) else {
        continue 'for_direction;
      };

      // Get next x and y
      if let Some(next_x_inc) = add_usize_i32(next_x, x_inc) {
        next_x = next_x_inc;
      } else {
        continue 'for_direction;
      }
      if let Some(next_y_inc) = add_usize_i32(next_y, y_inc) {
        next_y = next_y_inc;
      } else {
        continue 'for_direction;
      }

      // Check array bounds
      if let Some(ia_next_letter) = ia.get((next_x, next_y)) {
        // Check next letter does not match expectation
        if next_letter != *ia_next_letter {
          continue 'for_direction;
        }
      } else {
        continue 'for_direction;
      }
    }
    num_xmas += 1;
  }
  return num_xmas;
}

// Test X of MAS for part 2
fn test_x_mas(x: usize, y: usize, ia: &Array2<char>) -> u32 {
  // Check all directions
  let mut num_mas: u32 = 0;
  'for_direction: for direction in Direction::iterator_diag() {
    if num_mas == 2 {
      break 'for_direction;
    }

    // Setup direction vars
    let mut next_x: usize;
    let mut next_y: usize;
    let x_inc: i32 = direction.get_x_inc();
    let y_inc: i32 = direction.get_y_inc();

    // Get next x and y
    if let Some(next_x_inc) = add_usize_i32(x, x_inc) {
      next_x = next_x_inc;
    } else {
      continue 'for_direction;
    }
    if let Some(next_y_inc) = add_usize_i32(y, y_inc) {
      next_y = next_y_inc;
    } else {
      continue 'for_direction;
    }

    // Check array bounds and check if M
    if let Some(ia_next_letter) = ia.get((next_x, next_y)) {
      if *ia_next_letter != 'M' {
        continue 'for_direction;
      }
    } else {
      continue 'for_direction;
    }

    // If M check for S in opposite direction
    let x_inc = x_inc * -1;
    let y_inc = y_inc * -1;
    if let Some(next_x_inc) = add_usize_i32(x, x_inc) {
      next_x = next_x_inc;
    } else {
      continue 'for_direction;
    }
    if let Some(next_y_inc) = add_usize_i32(y, y_inc) {
      next_y = next_y_inc;
    } else {
      continue 'for_direction;
    }
    if let Some(ia_next_letter) = ia.get((next_x, next_y)) {
      if *ia_next_letter != 'S' {
        continue 'for_direction;
      }
    } else {
      continue 'for_direction;
    }

    // If both pass increment num_mas
    num_mas += 1;
  }

  return (num_mas == 2) as u32;
}

pub fn part_one(input: &str) -> Option<u32> {
  // Determine array dimensions
  let mut x: usize = 0;
  let mut y: usize = 0;
  for (l_idx, line) in input.lines().enumerate() {
    y += 1;
    if l_idx == 0 {
      x = line.len();
    }
  }

  // Get input as a 2-d vector
  let mut input_array = Array2::from_elem((x, y), ' ');
  for (l_idx, line) in input.lines().enumerate() {
    for (c_idx, c) in line.chars().enumerate() {
      input_array[[l_idx, c_idx]] = c;
    }
  }

  // Loop through array and recur on X's
  let mut num_xmas: u32 = 0;
  for ((x, y), value) in input_array.indexed_iter() {
    if *value == 'X' {
      num_xmas += test_xmas(x, y, &input_array);
    }
  }

  Some(num_xmas)
}

pub fn part_two(input: &str) -> Option<u32> {
  // Determine array dimensions
  let mut x: usize = 0;
  let mut y: usize = 0;
  for (l_idx, line) in input.lines().enumerate() {
    y += 1;
    if l_idx == 0 {
      x = line.len();
    }
  }

  // Get input as a 2-d vector
  let mut input_array = Array2::from_elem((x, y), ' ');
  for (l_idx, line) in input.lines().enumerate() {
    for (c_idx, c) in line.chars().enumerate() {
      input_array[[l_idx, c_idx]] = c;
    }
  }

  // Loop through array and recur on A's
  let mut num_xmas: u32 = 0;
  for ((x, y), value) in input_array.indexed_iter() {
    if *value == 'A' {
      num_xmas += test_x_mas(x, y, &input_array);
    }
  }
  Some(num_xmas)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_one() {
    let result = part_one(&advent_of_code::template::read_file("examples", DAY));
    assert_eq!(result, Some(18));
  }

  #[test]
  fn test_part_two() {
    let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    assert_eq!(result, Some(9));
  }
}
