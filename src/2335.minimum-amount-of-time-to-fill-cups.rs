/*
 * @lc app=leetcode id=2335 lang=rust
 *
 * [2335] Minimum Amount of Time to Fill Cups
 */

pub struct Solution;

// @lc code=start
use core::cmp::Ordering;

impl Solution {
    pub fn fill_cups_fixed(mut amount: [i32; 3]) -> i32 {
        amount.sort_unstable();
        let sum = amount.iter().sum::<i32>();
        let [a, b, c] = amount;

        match (a + b).cmp(&c) {
            Ordering::Greater => sum / 2 + sum % 2,
            _ => c,
        }
    }

    pub fn fill_cups(amount: Vec<i32>) -> i32 {
        Self::fill_cups_fixed(unsafe { amount.as_slice().try_into().unwrap_unchecked() })
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;
    use test::Bencher;

    #[test]
    fn example1() {
        assert_eq!(Solution::fill_cups_fixed([1, 4, 2]), 4);
    }

    #[bench]
    fn bench_example1(b: &mut Bencher) {
        b.iter(|| {
            test::black_box(Solution::fill_cups_fixed(test::black_box([1, 4, 2])));
        })
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::fill_cups_fixed([5, 4, 4]), 7);
    }

    #[bench]
    fn bench_example2(b: &mut Bencher) {
        b.iter(|| {
            test::black_box(Solution::fill_cups_fixed(test::black_box([5, 4, 4])));
        })
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::fill_cups_fixed([5, 0, 0]), 5);
    }

    #[bench]
    fn bench_example3(b: &mut Bencher) {
        b.iter(|| {
            test::black_box(Solution::fill_cups_fixed(test::black_box([5, 0, 0])));
        })
    }
}
