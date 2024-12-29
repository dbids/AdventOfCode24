advent_of_code::solution!(12);

// ##################### DEPENDENCIES ####################
use self::Direction::*;
use ndarray::Array2;
use std::slice::Iter;

// ##################### TYPE DEFS #####################
#[derive(Clone, Copy, Debug)]
struct MapElem {
  label: char,
  seen: bool,
}
impl MapElem {
  fn new() -> Self {
    MapElem {
      label: '0',
      seen: false,
    }
  }
}

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
    static DIRECTIONS: [Direction; 4] = [Left, Right, Up, Down];
    DIRECTIONS.iter()
  }
}

// ##################### HELPER FUNCTIONS ####################
// Get input map
fn parse_input(input: &str) -> (Array2<MapElem>, isize, isize) {
  // Determine array dimensions
  let mut height: isize = 0;
  let mut width: isize = 0;
  for (l_idx, line) in input.lines().enumerate() {
    width += 1;
    if l_idx == 0 {
      height = line.len() as isize;
    }
  }

  // Get input as a 2-d vector
  let mut input_array = Array2::from_elem((height as usize, width as usize), MapElem::new());
  for (l_idx, line) in input.lines().enumerate() {
    for (c_idx, c) in line.chars().enumerate() {
      input_array[[l_idx, c_idx]].label = c;
    }
  }

  return (input_array, height, width);
}

// Use DFS to determine the regions information, updating the visited bools in the array as needed
fn find_region_info(map: &mut Array2<MapElem>, x: isize, y: isize, label: char) -> (usize, usize) {
  // Get the current element on the map, if not found this is off the map
  let Some(start_elem) = map.get_mut((x as usize, y as usize)) else {
    return (0, 1);
  };

  // Add to perimeter, but not area if the label does not match the regions label
  if start_elem.label != label {
    return (0, 1);
  }

  // Count this out if its already been seen
  if start_elem.seen {
    return (0, 0);
  }
  start_elem.seen = true;

  // Branch for area and perimeters
  let mut area: usize = 1;
  let mut perimeter: usize = 0;
  for dir in Direction::iterator() {
    let (add_area, add_perimeter) =
      find_region_info(map, x + dir.get_x_inc(), y + dir.get_y_inc(), label);
    area += add_area;
    perimeter += add_perimeter;
  }
  return (area, perimeter);
}

// ##################### PART ONE #####################
pub fn part_one(input: &str) -> Option<usize> {
  // Get input as array of chars
  let (mut map, height, width) = parse_input(&input);

  let mut total_price: usize = 0;
  for start_x in 0..height {
    for start_y in 0..width {
      // Get start element of region and continue if its already been seen
      let start_elem = map[(start_x as usize, start_y as usize)];
      if start_elem.seen {
        continue;
      }

      // DFS the region, calculating the area and perimeter
      let (area, perimeter) = find_region_info(&mut map, start_x, start_y, start_elem.label);

      total_price += area * perimeter;
    }
  }

  Some(total_price)
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
    assert_eq!(result, Some(140));
  }

  #[test]
  fn test_part_one_second() {
    let result = part_one(&advent_of_code::template::read_file_part(
      "examples", DAY, 2,
    ));
    assert_eq!(result, Some(1930));
  }

  #[test]
  fn test_part_two() {
    let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    assert_eq!(result, None);
  }
}
