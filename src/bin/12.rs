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
    static DIRECTIONS: [Direction; 4] = [Left, Up, Right, Down];
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

// Check if inside region
fn inside_region(map: &mut Array2<MapElem>, x: isize, y: isize, label: char) -> bool {
  // Get the current element on the map, if not found this is off the map
  if let Some(curr_elem) = map.get((x as usize, y as usize)) {
    // Check if the label does not match the regions label
    if curr_elem.label != label {
      return false;
    }
  } else {
    return false;
  };
  return true;
}

// ##################### PART ONE #####################
// Use DFS to determine the regions information, updating the visited bools in the array as needed
fn find_region_info(map: &mut Array2<MapElem>, x: isize, y: isize, label: char) -> (usize, usize) {
  // Get the current element on the map, if not found this is off the map
  let Some(curr_elem) = map.get_mut((x as usize, y as usize)) else {
    return (0, 1);
  };

  // Add to perimeter, but not area if the label does not match the regions label
  if curr_elem.label != label {
    return (0, 1);
  }

  // Count this out if its already been seen
  if curr_elem.seen {
    return (0, 0);
  }
  curr_elem.seen = true;

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
// DFT but this time count corners to determine the number of sides
fn find_region_info_sides(
  map: &mut Array2<MapElem>,
  x: isize,
  y: isize,
  label: char,
) -> (usize, usize) {
  // Get the current element on the map, if not found this is off the map
  let Some(curr_elem) = map.get_mut((x as usize, y as usize)) else {
    return (0, 0);
  };

  // Add to perimeter, but not area if the label does not match the regions label
  if curr_elem.label != label {
    return (0, 0);
  }

  // Count this out if its already been seen
  if curr_elem.seen {
    return (0, 0);
  }
  curr_elem.seen = true;

  // Check for corners
  let dir_vec: Vec<Direction> = vec![
    Direction::Left,
    Direction::Up,
    Direction::Right,
    Direction::Down,
  ];
  let mut corners: usize = 0;
  let mut area: usize = 1;
  for d1_idx in 0..dir_vec.len() {
    let d2_idx: usize = if d1_idx != dir_vec.len() - 1 {
      d1_idx + 1
    } else {
      0
    };
    let d1 = dir_vec[d1_idx].clone();
    let d2 = dir_vec[d2_idx].clone();

    // Exterior corner
    if !inside_region(map, x + d1.get_x_inc(), y + d1.get_y_inc(), label)
      && !inside_region(map, x + d2.get_x_inc(), y + d2.get_y_inc(), label)
    {
      corners += 1;
    }

    // Interior corner
    if inside_region(map, x + d1.get_x_inc(), y + d1.get_y_inc(), label)
      && inside_region(map, x + d2.get_x_inc(), y + d2.get_y_inc(), label)
      && !inside_region(
        map,
        x + d1.get_x_inc() + d2.get_x_inc(),
        y + d1.get_y_inc() + d2.get_y_inc(),
        label,
      )
    {
      corners += 1;
    }
  }

  // Branch in all directions for area and corners
  for dir in Direction::iterator() {
    let (add_area, add_corners) =
      find_region_info_sides(map, x + dir.get_x_inc(), y + dir.get_y_inc(), label);
    area += add_area;
    corners += add_corners;
  }
  return (area, corners);
}

pub fn part_two(input: &str) -> Option<usize> {
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
      let (area, corners) = find_region_info_sides(&mut map, start_x, start_y, start_elem.label);
      total_price += area * corners;
    }
  }

  Some(total_price)
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
    assert_eq!(result, Some(80));
  }
  #[test]
  fn test_part_two_second() {
    let result = part_two(&advent_of_code::template::read_file_part(
      "examples", DAY, 2,
    ));
    assert_eq!(result, Some(1206));
  }
}
