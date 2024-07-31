/*
 * @lc app=leetcode id=4 lang=rust
 *
 * [4] Median of Two Sorted Arrays
 */

use super::Solution;

// @lc code=start
#[allow(unused)]
impl Solution {
  // O(m + n)
  fn two_pointer(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let mut p1 = 0;
    let mut p2 = 0;

    let total = nums1.len() + nums2.len();
    let is_even = total % 2 == 0;
    let center = total / 2;

    let mut previous = 0;

    macro_rules! check_index {
      ($val:expr) => {
        if p1 + p2 == center + 1 {
          break if is_even {
            (previous + $val) as f64 / 2.0
          } else {
            $val as f64
          };
        } else {
          previous = $val;
        }
      };
    }

    loop {
      match (nums1.get(p1), nums2.get(p2)) {
        (Some(&val1), Some(&val2)) => {
          if val1 < val2 {
            p1 += 1;
            check_index!(val1);
          } else {
            p2 += 1;
            check_index!(val2);
          }
        }

        (Some(&val1), None) => {
          p1 += 1;
          check_index!(val1);
        }

        (None, Some(&val2)) => {
          p2 += 1;
          check_index!(val2);
        }

        (None, None) => unreachable!(),
      };
    }
  }

  // O(log(m + n))
  fn binary_search(
    a: Vec<i32>,
    b: Vec<i32>,
    k: usize,
    a_start: usize,
    a_end: Option<usize>,
    b_start: usize,
    b_end: Option<usize>,
  ) -> f64 {
    todo!("Fix stack overflow");

    let Some(a_end) = a_end else {
      return b[k - a_start] as f64;
    };
    let Some(b_end) = b_end else {
      return a[k - b_start] as f64;
    };

    let a_index = (a_start + a_end) / 2;
    let b_index = (b_start + b_end) / 2;

    let a_value = a[a_index];
    let b_value = b[b_index];

    if a_index + b_index < k {
      if a_value < b_value {
        Self::binary_search(a, b, k, a_index + 1, Some(a_end), b_start, Some(b_end))
      } else {
        Self::binary_search(a, b, k, a_start, Some(a_end), b_index + 1, Some(b_end))
      }
    } else {
      if a_value > b_value {
        Self::binary_search(
          a,
          b,
          k,
          a_start,
          a_index.checked_sub(1),
          b_start,
          Some(b_end),
        )
      } else {
        Self::binary_search(
          a,
          b,
          k,
          a_start,
          Some(a_end),
          b_start,
          b_index.checked_sub(1),
        )
      }
    }
  }

  pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let n1 = nums1.len();
    let n2 = nums2.len();
    let total = n1 + n2;

    if total % 2 == 1 {
      Self::binary_search(
        nums1,
        nums2,
        total / 2,
        0,
        n1.checked_sub(1),
        0,
        n2.checked_sub(1),
      )
    } else {
      (Self::binary_search(
        nums1.clone(),
        nums2.clone(),
        total / 2 - 1,
        0,
        n1.checked_sub(1),
        0,
        n2.checked_sub(1),
      ) + Self::binary_search(
        nums1,
        nums2,
        total / 2,
        0,
        n1.checked_sub(1),
        0,
        n2.checked_sub(1),
      )) / 2.0
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
