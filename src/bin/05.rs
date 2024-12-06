advent_of_code::solution!(5);

// Solution dependencies
use regex::Regex;

pub fn part_one(input: &str) -> Option<u64> {

  // Split input into two at blank line
  let Some(split_idx) = input.find("\n\n") else {
    return None;
  };
  let (rules_str, updates_str) = input.split_at(split_idx);
  let updates_str = &updates_str[2..]; // Cutoff \n\n

  // Create array of vectors to hold rules
  const NEW_VEC: Vec<u8> = Vec::new();
  let mut rules = [NEW_VEC; 100];
  let rules_re = Regex::new(r"([0-9]+)\|([0-9]+)").unwrap(); // search for mul
  for (_, [bef, aft]) in rules_re.captures_iter(rules_str).map(|c| c.extract()) {
    let bef = bef.parse::<usize>().unwrap();
    let aft = aft.parse::<u8>().unwrap();
    rules[bef].push(aft);
  }

  // Check updates for validity
  let mut mid_sum: u64 = 0;
  'updates: for update in updates_str.split_whitespace() {
    // Get string as a vector
    let update_vec : Vec<u8> = update
      .split(',')
      .map(|d| d.parse::<u8>().unwrap()) // Maps iter of strings to iter of u8
      .collect::<Vec<u8>>(); //Collects all elements of iter into vector

    // iterate over vector, checking rules at each index
    for uv_idx in 0..update_vec.len() {
      let r_idx: usize = update_vec[uv_idx] as usize;
      for rule in rules[r_idx].clone().into_iter() {
        if (&update_vec[0..uv_idx].into_iter().find(|d| **d==rule)).is_some() {
          // This rule is broken
          continue 'updates;
        }
      }
    }
    mid_sum += update_vec[update_vec.len()/2] as u64;
  }
  return Some(mid_sum);
}

pub fn part_two(input: &str) -> Option<u32> {
  None
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_one() {
    let result = part_one(&advent_of_code::template::read_file("examples", DAY));
    assert_eq!(result, Some(143));
  }

  #[test]
  fn test_part_two() {
    let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    assert_eq!(result, None);
  }
}
