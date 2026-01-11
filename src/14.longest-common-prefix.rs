/*
 * @lc app=leetcode id=14 lang=rust
 *
 * [14] Longest Common Prefix
 */

pub struct Solution;

// @lc code=start
impl Solution {
    pub fn longest_common_prefix_no_alloc<'a, I, S>(strs: I) -> &'a str
    where
        I: IntoIterator<Item = &'a S>,
        S: ?Sized + AsRef<str> + 'a,
    {
        let mut iter = strs.into_iter();
        let Some(prefix) = iter.next() else {
            return "";
        };
        let mut prefix = prefix.as_ref().as_bytes();

        for s in iter {
            let s = s.as_ref().as_bytes();
            let len = prefix
                .iter()
                .zip(s)
                .enumerate()
                .find(|(_, (c1, c2))| c1 != c2)
                .map(|(i, _)| i)
                .unwrap_or_else(|| s.len().min(prefix.len()));

            prefix = &prefix[..len];
        }

        // SAFETY: prefix came from valid UTF-8 and we only cut at byte boundaries
        unsafe { core::str::from_utf8_unchecked(prefix) }
    }

    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        Self::longest_common_prefix_no_alloc(strs.as_slice()).to_string()
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;
    use test::Bencher;

    #[test]
    fn string_arguments() {
        let list = vec![
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string(),
        ];

        assert_eq!(Solution::longest_common_prefix(list), "fl");
    }

    #[bench]
    fn bench_string_arguments(b: &mut Bencher) {
        b.iter(|| {
            test::black_box(Solution::longest_common_prefix(test::black_box(vec![
                "flower".to_string(),
                "flow".to_string(),
                "flight".to_string(),
            ])));
        })
    }

    #[test]
    fn example1() {
        assert_eq!(
            Solution::longest_common_prefix_no_alloc(["flower", "flow", "flight"]),
            "fl"
        );
    }

    #[bench]
    fn bench_example1(b: &mut Bencher) {
        b.iter(|| {
            test::black_box(Solution::longest_common_prefix_no_alloc(test::black_box([
                "flower", "flow", "flight",
            ])));
        })
    }

    #[test]
    fn example2() {
        assert_eq!(
            Solution::longest_common_prefix_no_alloc(["dog", "racecar", "car"]),
            ""
        );
    }

    #[bench]
    fn bench_example2(b: &mut Bencher) {
        b.iter(|| {
            test::black_box(Solution::longest_common_prefix_no_alloc(test::black_box([
                "dog", "racecar", "car",
            ])));
        })
    }

    #[test]
    fn case109() {
        assert_eq!(Solution::longest_common_prefix_no_alloc(["ab", "a"]), "a");
    }

    #[bench]
    fn bench_case109(b: &mut Bencher) {
        b.iter(|| {
            test::black_box(Solution::longest_common_prefix_no_alloc(test::black_box([
                "ab", "a",
            ])));
        })
    }

    #[test]
    fn case117() {
        assert_eq!(
            Solution::longest_common_prefix_no_alloc(["aaa", "aa", "aaa"]),
            "aa"
        );
    }

    #[bench]
    fn bench_case117(b: &mut Bencher) {
        b.iter(|| {
            test::black_box(Solution::longest_common_prefix_no_alloc(test::black_box([
                "aaa", "aa", "aaa",
            ])));
        })
    }
}
