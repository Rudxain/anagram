//! A collection of anagram utility functions
//!
//! ## Installation
//! Add `anagram = 0.5.0` to your Cargo.toml
//!
//! ## Examples
//! ```
//! use anagram::{count, get_next, is_anagram, occurences};
//!
//! fn main() {
//!   // count how many anagrams can be formed from a given word
//!   let anagram_count = count("ordeals");
//!   assert_eq!(anagram_count, Some(5040));
//!
//!   // count the number of occurences of an anagram in a given word
//!   let occur = occurences("helloworldhello", "ll");
//!   assert_eq!(occur, 2);
//!
//!   // check if a word is an anagram of another word
//!   let ok = is_anagram("rustiscool", "oolcsistru");
//!   assert_eq!(ok, true);
//!
//!   // get the next lexicographically greater anagram
//!   let next = get_next("abcdefg");
//!   assert_eq!(next, "abcdegf");
//!
//!   // get all anagrams of a word
//!   let mut word = String::from("abc");
//!   for _ in 0..count(&word).unwrap() {
//!     // get next anagram
//!     word = get_next(&word);
//!     println!("{}", word);
//!   }
//! }
//! ```
use counter::Counter;
use std::{collections::HashSet, str::from_utf8};

#[must_use]
fn factorial(n: u8) -> Option<u128> {
  (1..=(n.into())).try_fold(1, u128::checked_mul)
}

/// Count the number of anagrams that can be formed from a word
#[must_use]
pub fn count(word: &str) -> Option<u128> {
  let mut unique = HashSet::new();

  let mut divisor: u128 = 1;

  let char_counts = word.chars().collect::<Counter<_>>();

  let mut char_count: usize = 0;
  for c in word.chars() {
    char_count += 1;
    if unique.insert(c) {
      divisor *= char_counts[&c] as u128;
    }
  }

  factorial(char_count as _).map(|c| {
    debug_assert!(c.is_multiple_of(divisor));
    c / divisor
  })
}

#[must_use]
const fn all_zero(s: &[i64]) -> bool {
  let mut i = 0;
  while i < s.len() {
    if s[i] != 0 {
      return false;
    }
    i += 1;
  }
  true
}

/// Count the number of occurences of an anagram in a word
#[must_use]
pub fn occurences(word: &str, input: &str) -> u128 {
  let len_word = word.chars().count();
  let len_input = input.chars().count();

  let mut count = [0_i64; 0x100];

  for val in 0..len_word {
    count[word.as_bytes()[val] as usize] += 1;
  }

  for val in 0..len_input {
    count[input.as_bytes()[val] as usize] -= 1;
  }

  let mut result: u128 = 0;
  result += u128::from(all_zero(&count));

  for i in len_input..len_word {
    // add last character
    count[word.as_bytes()[i] as usize] += 1;

    // remove first character
    count[word.as_bytes()[i - len_input] as usize] -= 1;

    result += u128::from(all_zero(&count));
  }
  result
}

/// Check if a word is an anagram of another word
#[must_use]
pub fn is_anagram(left: &str, right: &str) -> bool {
  if left.chars().count() != right.chars().count() {
    return false;
  }

  let mut count = [0_isize; 26];

  for c in left.chars() {
    let pos = if let Some(val) = c.to_digit(10) {
      val as usize
    } else {
      ('a'..='z')
        .position(|x| x == c.to_lowercase().next().unwrap())
        .unwrap()
    };

    count[pos] += 1;
  }

  for c in right.chars() {
    let pos = if let Some(val) = c.to_digit(10) {
      val as usize
    } else {
      ('a'..='z')
        .position(|x| x == c.to_lowercase().next().unwrap())
        .unwrap()
    };

    count[pos] -= 1;

    if count[pos] < 0 {
      return false;
    }
  }

  true
}

/// Get the next lexicographically greater permutation
/// This function will return either the next greater permutation or the
/// lexicographically smallest permutation if the given word is the
/// lexicographically greatest permutation
/// Examples:
/// "abc" -> "acb"
/// "cba" -> "abc"
#[must_use]
pub fn get_next(word: &str) -> String {
  let mut i = word.chars().count() - 1;

  // find the first char smaller than the char next to it
  while i > 0 {
    if word.as_bytes()[i] > word.as_bytes()[i - 1] {
      break;
    }
    i -= 1;
  }

  // we are at the lexicographically greatest permutation so we
  // return the lexicographically smallest one
  if i == 0 {
    let mut chars: Vec<char> = word.chars().collect();
    chars.sort_unstable();
    return chars.into_iter().collect();
  }

  // find the smallest char on the right side of i-1'th char
  // that's greater than word[i - 1]
  let mut j = i + 1;
  let mut smallest = i;
  let mut word_as_bytes: Vec<u8> = word.to_string().into_bytes();
  while j < word.chars().count() {
    if word_as_bytes[j] > word_as_bytes[i - 1] && word_as_bytes[j] < word_as_bytes[smallest] {
      smallest = j;
    }
    j += 1;
  }

  // swap smallest with word[i - 1]
  word_as_bytes.swap(smallest, i - 1);

  // sort right half
  let mut right_half: Vec<u8> = word_as_bytes[i..word.chars().count()].to_vec();
  right_half.sort_unstable();

  // merge back and return as a String
  from_utf8(&[&word_as_bytes[0..i], &right_half].concat())
    .unwrap()
    .to_string()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_factorial() {
    assert_eq!(factorial(1), Some(1));
    assert_eq!(factorial(2 * 2), Some(24));
    assert_eq!(factorial(14), Some(87178291200));
    assert_eq!(factorial(12), Some(479001600));
  }

  #[test]
  fn test_anagram_count() {
    assert_eq!(count("at"), Some(2));
    assert_eq!(count("ordeals"), Some(5040));
    assert_eq!(
      count("abcdefghijklmnopqrstuvwxyz"),
      Some(403291461126605635584000000)
    );
    assert_eq!(
      count("abcdefghijklmabcdefghijklm"),
      Some(49229914688306352000000)
    );
    assert_eq!(count("abcdABCDabcd"), Some(29937600));
  }

  #[test]
  fn test_is_anagram() {
    assert_eq!(is_anagram("hello", "ooo"), false);
    assert_eq!(is_anagram("Hello", "olleH"), true);
    assert_eq!(is_anagram("hello", "olleh"), true);
    assert_eq!(is_anagram("helicopter", "copterheli"), true);
    assert_eq!(is_anagram("hacker", "hackes"), false);
    assert_eq!(is_anagram("HaCkER", "hacker"), true);
    assert_eq!(is_anagram("ac", "bb"), false);
    assert_eq!(is_anagram("123", "321"), true);
    assert_eq!(is_anagram("1110002293", "1101009322"), true);
    assert_eq!(is_anagram("1102eeaA", "20Svv00"), false);
    assert_eq!(is_anagram("1102eeaA", "0112EEaA"), true);
    assert_eq!(is_anagram("1102eeaA", "0112eaAe"), true);
  }

  #[test]
  fn test_occurences() {
    assert_eq!(occurences("forxxorfxdofr", "for"), 3);
    assert_eq!(occurences("hellohelloleh", "hel"), 3);
    assert_eq!(occurences("oofooflolhi", "oo"), 2);
    assert_eq!(occurences("rustiscool", "st"), 1);
    assert_eq!(occurences("thegrandopeningscenerywasgreat", "grand"), 1);
    assert_eq!(occurences("anagrams", "smargana"), 1);
  }

  #[test]
  fn test_get_next() {
    assert_eq!(get_next("abc"), "acb");
    assert_eq!(get_next("bac"), "bca");
    assert_eq!(get_next("aaa"), "aaa");
    assert_eq!(get_next("cba"), "abc");
    assert_eq!(get_next("218765"), "251678");
    assert_eq!(get_next("1234"), "1243");
    assert_eq!(get_next("4321"), "1234");
    assert_eq!(get_next("534976"), "536479");
  }
}
