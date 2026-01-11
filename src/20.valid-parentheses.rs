/*
 * @lc app=leetcode id=20 lang=rust
 *
 * [20] Valid Parentheses
 */

pub struct Solution;

// @lc code=start
impl Solution {
    pub fn is_valid<S>(s: S) -> bool
    where
        S: AsRef<str>,
    {
        let mut stack = Vec::new();

        for &c in s.as_ref().as_bytes() {
            if matches!(c, b'(' | b'{' | b'[') {
                stack.push(c);
            } else {
                let open_char = match c {
                    b')' => b'(',
                    b'}' => b'{',
                    b']' => b'[',
                    _ => unsafe { core::hint::unreachable_unchecked() },
                };

                if stack.pop() != Some(open_char) {
                    return false;
                }
            }
        }

        stack.is_empty()
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
        assert!(Solution::is_valid("()"));
    }

    #[bench]
    fn bench_example1(b: &mut Bencher) {
        b.iter(|| {
            test::black_box(Solution::is_valid(test::black_box("()")));
        })
    }

    #[test]
    fn example2() {
        assert!(Solution::is_valid("()[]{}"));
    }

    #[bench]
    fn bench_example2(b: &mut Bencher) {
        b.iter(|| {
            test::black_box(Solution::is_valid(test::black_box("()[]{}")));
        })
    }

    #[test]
    fn example3() {
        assert!(!Solution::is_valid("(]"));
    }

    #[bench]
    fn bench_example3(b: &mut Bencher) {
        b.iter(|| {
            test::black_box(Solution::is_valid(test::black_box("(]")));
        })
    }

    #[test]
    fn example4() {
        assert!(Solution::is_valid("([])"));
    }

    #[bench]
    fn bench_example4(b: &mut Bencher) {
        b.iter(|| {
            test::black_box(Solution::is_valid(test::black_box("([])")));
        })
    }

    #[test]
    fn example5() {
        assert!(!Solution::is_valid("([)]"));
    }

    #[bench]
    fn bench_example5(b: &mut Bencher) {
        b.iter(|| {
            test::black_box(Solution::is_valid(test::black_box("([)]")));
        })
    }

    #[test]
    fn case91() {
        assert!(!Solution::is_valid("["));
    }

    #[bench]
    fn bench_case91(b: &mut Bencher) {
        b.iter(|| {
            test::black_box(Solution::is_valid(test::black_box("[")));
        })
    }
}
