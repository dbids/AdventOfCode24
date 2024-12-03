advent_of_code::solution!(3);

// Solution dependencies
use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
  let mut result: u32 = 0;

  // Iterate over matches on input string
  let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap(); // search for mul
  for (_, [l, r]) in re.captures_iter(input).map(|c| c.extract()) {
    // Check for 1-3 digits
    if (l.len() > 3) || (r.len() > 3) {
      //println!("mul({l},{r})");
      continue;
    }

    // Calculate mult
    let l_int = l.parse::<u32>().unwrap();
    let r_int = r.parse::<u32>().unwrap();
    result += l_int * r_int;
  }

  Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
  let mut result: u32 = 0;
  let mut is_do: bool = true;
  // Iterate over matches on input string
  let re = Regex::new(r"(mul\([0-9]+,[0-9]+\))|(do\(\))|(don't\(\))").unwrap(); // search for mul, do, don't
  for (_, [s]) in re.captures_iter(input).map(|c| c.extract()) {
    // println!("{s}");

    if s == "do()" {
      is_do = true;
      continue;
    }
    if s == "don't()" {
      is_do = false;
      continue;
    }

    if is_do {
      // Extract numbers
      let re_mult = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap(); // search for mul
      let Some((_, [l, r])) = re_mult.captures(s).map(|c| c.extract()) else {
        continue;
      };

      // Check for 1-3 digits
      if (l.len() > 3) || (r.len() > 3) {
        //println!("mul({l},{r})");
        continue;
      }

      // Calculate mult
      let l_int = l.parse::<u32>().unwrap();
      let r_int = r.parse::<u32>().unwrap();
      result += l_int * r_int;
    }
  }

  Some(result)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_one() {
    let result = part_one(&advent_of_code::template::read_file("examples", DAY));
    assert_eq!(result, Some(161));
  }

  #[test]
  fn test_part_two() {
    let result = part_two(&advent_of_code::template::read_file_part(
      "examples", DAY, 2,
    ));
    assert_eq!(result, Some(48));
  }
}
