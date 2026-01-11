/*
 * @lc app=leetcode id=9 lang=rust
 *
 * [9] Palindrome Number
 */

pub struct Solution;

impl Solution {
    pub fn is_palindrome_first_last_control(x: i32) -> bool {
        let Ok(mut x) = u32::try_from(x) else {
            return false;
        };

        let mut n = std::num::Saturating(x.checked_ilog10().unwrap_or(0));

        while n.0 > 0 {
            let r = 10u32.pow(n.0);
            let first_digit = x / r;
            let last_digit = x % 10;

            if first_digit != last_digit {
                return false;
            }

            x -= last_digit * r;
            x /= 10;
            n -= 2;
        }

        true
    }

    pub fn is_palindrome_rev(x: i32) -> bool {
        let Ok(x) = u32::try_from(x) else {
            return false;
        };

        let mut acc = x;
        let mut reversed = 0;

        while acc != 0 {
            reversed = reversed * 10 + acc % 10;
            acc /= 10;
        }

        x == reversed
    }
}

// @lc code=start
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let Ok(mut x) = u32::try_from(x) else {
            return false;
        };

        let mut n = std::num::Saturating(x.checked_ilog10().unwrap_or(0));

        while n.0 > 0 {
            let r = 10u32.pow(n.0);
            let first_digit = x / r;
            let last_digit = x % 10;

            if first_digit != last_digit {
                return false;
            }

            x -= last_digit * r;
            x /= 10;
            n -= 2;
        }

        true
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
        assert_eq!(Solution::is_palindrome(121), true);
    }

    #[bench]
    fn bench_example1(b: &mut Bencher) {
        b.iter(|| {
            test::black_box(Solution::is_palindrome(test::black_box(121)));
        })
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::is_palindrome(-121), false);
    }

    #[bench]
    fn bench_example2(b: &mut Bencher) {
        b.iter(|| {
            test::black_box(Solution::is_palindrome(test::black_box(-121)));
        })
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::is_palindrome(10), false);
    }

    #[bench]
    fn bench_example3(b: &mut Bencher) {
        b.iter(|| {
            test::black_box(Solution::is_palindrome(test::black_box(10)));
        })
    }

    #[test]
    fn case11() {
        assert_eq!(Solution::is_palindrome(11), true);
    }

    #[bench]
    fn bench_case11(b: &mut Bencher) {
        b.iter(|| {
            test::black_box(Solution::is_palindrome(test::black_box(11)));
        })
    }

    #[test]
    fn case34() {
        assert_eq!(Solution::is_palindrome(88888), true);
    }

    #[bench]
    fn bench_case34(b: &mut Bencher) {
        b.iter(|| {
            test::black_box(Solution::is_palindrome(test::black_box(88888)));
        })
    }

    #[test]
    fn example1_rev() {
        assert_eq!(Solution::is_palindrome_rev(121), true);
    }

    #[bench]
    fn bench_example1_rev(b: &mut Bencher) {
        b.iter(|| {
            test::black_box(Solution::is_palindrome_rev(test::black_box(121)));
        })
    }

    #[test]
    fn example2_rev() {
        assert_eq!(Solution::is_palindrome_rev(-121), false);
    }

    #[bench]
    fn bench_example2_rev(b: &mut Bencher) {
        b.iter(|| {
            test::black_box(Solution::is_palindrome_rev(test::black_box(-121)));
        })
    }

    #[test]
    fn example3_rev() {
        assert_eq!(Solution::is_palindrome_rev(10), false);
    }

    #[bench]
    fn bench_example3_rev(b: &mut Bencher) {
        b.iter(|| {
            test::black_box(Solution::is_palindrome_rev(test::black_box(10)));
        })
    }

    #[test]
    fn case11_rev() {
        assert_eq!(Solution::is_palindrome_rev(11), true);
    }

    #[bench]
    fn bench_case11_rev(b: &mut Bencher) {
        b.iter(|| {
            test::black_box(Solution::is_palindrome_rev(test::black_box(11)));
        })
    }

    #[test]
    fn case34_rev() {
        assert_eq!(Solution::is_palindrome_rev(88888), true);
    }

    #[bench]
    fn bench_case34_rev(b: &mut Bencher) {
        b.iter(|| {
            test::black_box(Solution::is_palindrome_rev(test::black_box(88888)));
        })
    }
}
