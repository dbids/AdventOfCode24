advent_of_code::solution!(6);

// ##################### DEPENDENCIES ####################
use ndarray::Array2;
use std::slice::Iter;
use self::Direction::*;

// ##################### TYPE DEFS #####################
// Create struct to represent level and enum for position
#[derive(Clone,Copy,Debug,PartialEq, Eq)]
enum PositionType {
  Object,
  Empty,
  Person,
}
#[derive(Clone,Copy,Debug,PartialEq, Eq)]
struct LevelMap {
  position_type: PositionType,
  visited: bool,
}
impl LevelMap {
  fn from_char(position_char: char) -> LevelMap{
    let (position_type_temp, visited_temp) = match position_char {
      '#'             => (PositionType::Object, false),
      '.'             => (PositionType::Empty, false),
      '^'|'<'|'>'|'v' => (PositionType::Person, false),
      _               => {panic!("Unknown char!!");},
    };
    LevelMap {
      position_type: position_type_temp,
      visited: visited_temp,
    }
  }
}

// Create direction enum, with methods for x/y traversing directions
#[derive(Clone,Copy,Debug)]
enum Direction {
  Left,
  Right,
  Up,
  Down,
}
impl Direction {
  fn get_x_inc(&self) -> i32 {
    match &self {
      Direction::Down => 1,
      Direction::Up => -1,
      _ => 0,
    }
  }

  fn get_y_inc(&self) -> i32 {
    match &self {
      Direction::Left => -1,
      Direction::Right => 1,
      _ => 0,
    }
  }

  fn get_next_dir(&self) -> Direction {
    match &self {
      Direction::Up    => Direction::Right,
      Direction::Right => Direction::Down,
      Direction::Down  => Direction::Left,
      Direction::Left  => Direction::Up,
    }
  }
}

// Create person struct
#[derive(Clone,Copy,Debug)]
struct Person {
  x : usize,
  y : usize,
  direction : Direction,
}

// ##################### HELPER FUNCTIONS ####################
// Add usize to i32
fn add_usize_i32(u: usize, i: i32) -> Option<usize> {
  if i.is_negative() {
    u.checked_sub(i.wrapping_abs() as u32 as usize)
  } else {
    u.checked_add(i as usize)
  }
}

// ##################### PART ONE #####################
pub fn part_one(input: &str) -> Option<u32> {
  // Determine array dimensions
  let mut width: usize = 0;
  let mut height: usize = 0;
  for (l_idx, line) in input.lines().enumerate() {
    height += 1;
    if l_idx == 0 {
      width = line.len();
    }
  }

  // Get input as a 2-d array of LevelMap struct
  let mut person = Person {
    x: 0,
    y: 0,
    direction: Direction::Up,
  };
  let mut input_array = Array2::from_elem((width, height), LevelMap::from_char('.'));
  for (l_idx, line) in input.lines().enumerate() {
    for (c_idx, c) in line.chars().enumerate() {
      input_array[[l_idx, c_idx]] = LevelMap::from_char(c);
      if input_array[[l_idx, c_idx]].position_type == PositionType::Person {
        person = Person {
          x: l_idx,
          y: c_idx,
          direction: match c {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '>' => Direction::Right,
            '<' => Direction::Left,
            _ => panic!("One shouldn't be here"),
          },
        }
      }
    }
  }

  // Walk person through array, updating visited as they go
  let mut num_visited: u32 = 1;
  'walking : loop {
      // Get next x and y and check zero
      let x_inc: i32 = person.direction.get_x_inc();
      let y_inc: i32 = person.direction.get_y_inc();
      let Some(next_x) = add_usize_i32(person.x, x_inc) else {
        break 'walking;
      };
      let Some(next_y) = add_usize_i32(person.y, y_inc) else {
        break 'walking;
      };

      // Check array bounds and get next spot
      let Some(next_spot) = input_array.get((next_x, next_y)) else {
        break 'walking;
      };

      // Check for obstacle and change direction if so
      // Otherwise move
      if next_spot.position_type == PositionType::Object {
        person.direction = person.direction.get_next_dir();
      } else {
        person.x = next_x;
        person.y = next_y;
        if next_spot.visited == false {
          input_array[[next_x, next_y]].visited = true;
          num_visited += 1;
        }
      }

      // dbg!(person);
  }

  Some(num_visited)
}

// ##################### PART TWO #####################
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
    assert_eq!(result, Some(41));
  }

  #[test]
  fn test_part_two() {
    let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    assert_eq!(result, None);
  }
}
