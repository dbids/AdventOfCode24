advent_of_code::solution!(9);

// ##################### DEPENDENCIES ####################
use std::cmp::Ordering;
use std::collections::VecDeque;

// ##################### TYPE DEFS #####################
#[derive(Debug)]
struct Metadata {
  id_num: usize,
  start: usize,
  len: usize,
}

// ##################### HELPER FUNCTIONS ####################
fn format_input(input: &str, files: &mut VecDeque<Metadata>, spaces: &mut VecDeque<Metadata>) {
  let mut counter: usize = 0;
  input
    .chars()
    .filter(|c| c.is_numeric())
    .map(|c| c.to_digit(10).unwrap() as usize)
    .enumerate()
    .for_each(|(c_idx, c)| {
      if c != 0 {
        if (c_idx % 2) == 0 {
          files.push_back(Metadata {
            id_num: c_idx / 2,
            start: counter,
            len: c,
          });
        } else {
          spaces.push_back(Metadata {
            id_num: 0,
            start: counter,
            len: c,
          });
        }
        counter += c;
      }
    });
}

// ##################### PART ONE #####################
pub fn part_one(input: &str) -> Option<usize> {
  // Get input as metadata
  let mut files: VecDeque<Metadata> = VecDeque::new();
  let mut spaces: VecDeque<Metadata> = VecDeque::new();
  format_input(&input, &mut files, &mut spaces);

  // Compress data
  let Some(mut curr_file) = files.pop_back() else {
    panic!("No files")
  };
  let Some(mut curr_space) = spaces.pop_front() else {
    panic!("No spaces")
  };
  'Compress: loop {
    if curr_file.start < curr_space.start {
      files.push_back(curr_file);
      break 'Compress;
    }
    match curr_space.len.cmp(&curr_file.len) {
      Ordering::Greater => {
        files.push_front(Metadata {
          start: curr_space.start,
          ..curr_file
        });
        curr_space.start += curr_file.len;
        curr_space.len -= curr_file.len;
        curr_file = files
          .pop_back()
          .unwrap_or_else(|| panic!("Ran out of files"));
      }
      Ordering::Equal => {
        files.push_front(Metadata {
          start: curr_space.start,
          ..curr_file
        });
        curr_file = files
          .pop_back()
          .unwrap_or_else(|| panic!("Ran out of files"));
        if let Some(some_space) = spaces.pop_front() {
          curr_space = some_space;
        } else {
          break 'Compress;
        };
      }
      Ordering::Less => {
        files.push_front(Metadata {
          start: curr_space.start,
          id_num: curr_file.id_num,
          len: curr_space.len,
        });
        curr_file.len -= curr_space.len;
        if let Some(some_space) = spaces.pop_front() {
          curr_space = some_space;
        } else {
          break 'Compress;
        };
      }
    }
  }

  // Calculate checksum
  let mut checksum: usize = 0;
  for f in files.into_iter() {
    for offst in 0..f.len {
      checksum += f.id_num * (f.start + offst);
    }
  }

  Some(checksum)
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
    assert_eq!(result, Some(1928));
  }

  #[test]
  fn test_part_two() {
    let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    assert_eq!(result, None);
  }
}
