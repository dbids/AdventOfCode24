advent_of_code::solution!(1);

// Solution dependencies
use regex::Regex;

pub fn part_one(input: &str) -> Option<i32> {
  //println!("{input}");

  // Create vectors from the input string
  let mut llocs: Vec<i32> = Vec::new();
  let mut rlocs: Vec<i32> = Vec::new();
  let re = Regex::new(r"([0-9]+)\s+([0-9]+)").unwrap(); // () creates the matching group in the regex
  for (_, [l, r]) in re.captures_iter(input).map(|c| c.extract()) {
    let l_int = l.parse::<i32>().unwrap();
    let r_int = r.parse::<i32>().unwrap();
    llocs.push(l_int);
    rlocs.push(r_int);
  }

  // Check equal length of parsed vectors
  assert_eq!(llocs.len(), rlocs.len());
//  for i in 0..llocs.len() {
//    println!("{i}: {0}, {1}", llocs[i], rlocs[i]);
//  }

  // Sort the two vectors
  llocs.sort();
  rlocs.sort();
//  for i in 0..llocs.len() {
//    println!("{i}: {0}, {1}", llocs[i], rlocs[i]);
//  }

  // Find the difference
  let mut difference: i32 = 0;
  for i in 0..llocs.len() {
    difference += (llocs[i] - rlocs[i]).abs();
  }
//  println!("{difference}");

  Some(difference)
}

pub fn part_two(input: &str) -> Option<i32> {
  None
}

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
