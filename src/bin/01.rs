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
  //for i in 0..llocs.len() {
  //  println!("{i}: {0}, {1}", llocs[i], rlocs[i]);
  //}

  // Sort the two vectors
  llocs.sort();
  rlocs.sort();
  //for i in 0..llocs.len() {
  //  println!("{i}: {0}, {1}", llocs[i], rlocs[i]);
  //}

  // Find the difference
  let mut difference: i32 = 0;
  for i in 0..llocs.len() {
    difference += (llocs[i] - rlocs[i]).abs();
  }
  //println!("{difference}");

  Some(difference)
}

pub fn part_two(input: &str) -> Option<i32> {
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

  // Sort the two vectors
  llocs.sort();
  rlocs.sort();
  //for i in 0..llocs.len() {
  //  println!("{i}: {0}, {1}", llocs[i], rlocs[i]);
  //}

  // Convert vectors to tuple with frequency of occurance
  let llocs_freq: Vec<(i32, i32)> = calc_freq(&llocs);
  let rlocs_freq: Vec<(i32, i32)> = calc_freq(&rlocs);
  //for i in 0..llocs_freq.len() {
  //  println!("{i}: {0}, {1}", llocs_freq[i].0, llocs_freq[i].1);
  //}
  //for i in 0..rlocs_freq.len() {
  //  println!("{i}: {0}, {1}", rlocs_freq[i].0, rlocs_freq[i].1);
  //}

  // Calculate similarity score
  let mut similarity: i32 = 0;
  for lloc_freq in llocs_freq.into_iter() {
    'right_loc: for r_idx in 0..rlocs_freq.len() {
      if lloc_freq.0 == rlocs_freq[r_idx].0 {
        similarity += lloc_freq.0 * lloc_freq.1 * rlocs_freq[r_idx].1;
        break 'right_loc;
      }
    }
  }
  //println!("\n{similarity}");

  Some(similarity)
}

// Returns vectors of unique elements with frequencies counted
fn calc_freq(vec_in: &Vec<i32>) -> Vec<(i32, i32)> {
  let mut vec_out: Vec<(i32, i32)> = Vec::new();

  let mut curr_idx = 0;
  while curr_idx != vec_in.len() {
    if curr_idx == (vec_in.len() - 1) {
      vec_out.push((vec_in[curr_idx], 1));
      curr_idx += 1;
    } else if vec_in[curr_idx] != vec_in[curr_idx + 1] {
      vec_out.push((vec_in[curr_idx], 1));
      curr_idx += 1;
    } else {
      let mut freq: usize = 2;
      loop {
        if vec_in[curr_idx + freq - 1] != vec_in[curr_idx + freq] {
          vec_out.push((vec_in[curr_idx], (i32::try_from(freq).unwrap()))); // Had to type cast due to usize not converting to i32 cleanly
          curr_idx += freq;
          break;
        }
        freq += 1;
      }
    }
  }

  return vec_out;
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_one() {
    let result = part_one(&advent_of_code::template::read_file("examples", DAY));
    assert_eq!(result, Some(11));
  }

  #[test]
  fn test_part_two() {
    let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    assert_eq!(result, Some(31));
  }
}
