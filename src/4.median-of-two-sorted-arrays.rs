/*
 * @lc app=leetcode id=4 lang=rust
 *
 * [4] Median of Two Sorted Arrays
 */

use super::Solution;

// @lc code=start
impl Solution {
  fn median(nums: Vec<i32>) -> f64 {
    let len = nums.len();
    let center = len / 2;

    if len % 2 == 0 {
      (nums[center - 1] + nums[center]) as f64 / 2.0
    } else {
      nums[center] as _
    }
  }

  fn merge_sort<T>(l1: Vec<T>, l2: Vec<T>) -> Vec<T>
  where
    T: Ord + Copy,
  {
    let mut merge = Vec::new();

    let mut p1 = 0;
    let mut p2 = 0;

    loop {
      let val = match (l1.get(p1), l2.get(p2)) {
        (Some(&val1), Some(&val2)) => {
          if val1 < val2 {
            p1 += 1;
            val1
          } else {
            p2 += 1;
            val2
          }
        }

        (Some(&num1), None) => {
          p1 += 1;
          num1
        }

        (None, Some(&num2)) => {
          p2 += 1;
          num2
        }

        // We reached end of the numbers
        (None, None) => break merge,
      };

      merge.push(val);
    }
  }

  #[allow(dead_code)]
  fn binary_search() {
    todo!();
  }

  pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    match nums1.len() + nums2.len() {
      1 => *nums1.first().unwrap_or_else(|| &nums2[0]) as _,
      2 => Self::median([nums1, nums2].concat()),
      _ => Self::median(Self::merge_sort(nums1, nums2)),
    }
  }
}
// @lc code=end

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;

  macro_rules! example {
    (
      nums1 = [$($l1:expr),*];
      nums2 = [$($l2:expr),*]$(;)?
    ) => {
      Solution::find_median_sorted_arrays(vec![$($l1),*], vec![$($l2),*])
    };
  }

  macro_rules! assert_example {
    (
      Input: nums1 = $l1:tt, nums2 = $l2:tt;
      Output: $o:expr$(;)?
    ) => {
      assert_eq!(
        example! {
          nums1 = $l1;
          nums2 = $l2;
        },
        $o
      )
    };
  }

  #[test]
  fn example1() {
    assert_example! {
      Input: nums1 = [1,3], nums2 = [2];
      Output: 2.00000;
    }
  }

  #[bench]
  fn bench_example1(b: &mut Bencher) {
    b.iter(|| {
      example! {
        nums1 = [1,3];
        nums2 = [2];
      }
    })
  }

  #[test]
  fn example2() {
    assert_example! {
      Input: nums1 = [1,2], nums2 = [3,4];
      Output: 2.50000
    }
  }

  #[bench]
  fn bench_example2(b: &mut Bencher) {
    b.iter(|| {
      example! {
        nums1 = [1,2];
        nums2 = [3,4];
      }
    })
  }
}
