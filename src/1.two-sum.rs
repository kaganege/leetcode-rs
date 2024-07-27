/*
 * @lc app=leetcode id=1 lang=rust
 *
 * [1] Two Sum
 */

#![cfg_attr(not(test), allow(unused))]

use super::Solution;

// @lc code=start
impl Solution {
  pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut out = Vec::new();

    'top: for (i, a) in nums.iter().enumerate() {
      for (j, b) in nums.iter().enumerate() {
        if i == j {
          continue;
        }

        if a + b == target {
          out.push(i as i32);
          out.push(j as i32);
          break 'top;
        }
      }
    }

    out
  }
}
// @lc code=end

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;

  #[test]
  fn example1() {
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
  }

  #[bench]
  fn bench_example1(b: &mut Bencher) {
    b.iter(|| {
      Solution::two_sum(vec![2, 7, 11, 15], 9);
    });
  }

  #[test]
  fn example2() {
    assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
  }

  #[bench]
  fn bench_example2(b: &mut Bencher) {
    b.iter(|| {
      Solution::two_sum(vec![3, 2, 4], 6);
    });
  }

  #[test]
  fn example3() {
    assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
  }

  #[bench]
  fn bench_example3(b: &mut Bencher) {
    b.iter(|| {
      Solution::two_sum(vec![3, 3], 6);
    });
  }
}
