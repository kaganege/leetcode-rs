/*
 * @lc app=leetcode id=1 lang=rust
 *
 * [1] Two Sum
 */

// @lc code=start
use std::ops::ControlFlow;

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
