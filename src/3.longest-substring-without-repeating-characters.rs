/*
 * @lc app=leetcode id=3 lang=rust
 *
 * [3] Longest Substring Without Repeating Characters
 */

#![cfg_attr(not(test), allow(unused))]

use super::Solution;

// @lc code=start
impl Solution {
  pub fn length_of_longest_substring(s: String) -> i32 {
    let mut max_len = 0;
    let mut char_map: [usize; 128] = [0; 128];
    let mut start = 0;

    for (i, ch) in s.char_indices() {
      let ch = ch as usize;
      start = start.max(char_map[ch]);
      char_map[ch] = i + 1;
      max_len = max_len.max(i - start + 1);
    }

    max_len as _
  }
}
// @lc code=end

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;

  macro_rules! assert_example {
    (
      Input: s = $l1:expr;
      Output: $o:expr $(;)?
    ) => {
      assert_eq!(Solution::length_of_longest_substring(String::from($l1)), $o)
    };
  }

  #[test]
  fn example1() {
    assert_example! {
      Input: s = "abcabcbb";
      Output: 3
    }
  }

  #[bench]
  fn bench_example1(b: &mut Bencher) {
    b.iter(|| {
      Solution::length_of_longest_substring("abcabcbb".to_string());
    })
  }

  #[test]
  fn example2() {
    assert_example! {
      Input: s = "bbbbb";
      Output: 1
    }
  }

  #[bench]
  fn bench_example2(b: &mut Bencher) {
    b.iter(|| {
      Solution::length_of_longest_substring("bbbbb".to_string());
    })
  }

  #[test]
  fn example3() {
    assert_example! {
      Input: s = "pwwkew";
      Output: 3
    }
  }

  #[bench]
  fn bench_example3(b: &mut Bencher) {
    b.iter(|| {
      Solution::length_of_longest_substring("pwwkew".to_string());
    })
  }
}
