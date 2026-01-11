/*
 * @lc app=leetcode id=13 lang=rust
 *
 * [13] Roman to Integer
 */

pub struct Solution;

impl Solution {
    pub fn roman_to_int2(s: &str) -> i32 {
        let bytes = s.as_bytes();
        let mut sum = 0;

        for i in 0..bytes.len() - 1 {
            let curr = Self::roman_char_to_int(bytes[i] as char);
            let next = Self::roman_char_to_int(bytes[i + 1] as char);

            sum += if curr < next { -curr } else { curr };
        }

        // SAFETY: The length is 1 and the last character is ASCII
        sum + Self::roman_char_to_int(unsafe { *bytes.last().unwrap_unchecked() } as char)
    }
}

// @lc code=start
impl Solution {
    fn str_windows2(s: &str) -> impl ExactSizeIterator<Item = [char; 2]> {
        s.as_bytes()
            .windows(2)
            .map(|b| [b[0] as char, b[1] as char])
    }

    fn roman_char_to_int(c: char) -> i32 {
        match c {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            // SAFETY: The byte is can't be unknown character
            _ => unsafe { std::hint::unreachable_unchecked() },
        }
    }

    fn roman_array_to_int(a: [char; 2]) -> (i32, bool) {
        match a {
            ['C', 'M'] => (900, true),
            ['C', 'D'] => (400, true),
            ['X', 'C'] => (90, true),
            ['X', 'L'] => (40, true),
            ['I', 'X'] => (9, true),
            ['I', 'V'] => (4, true),
            [a, _] => (Self::roman_char_to_int(a), false),
        }
    }

    pub fn roman_to_int1(s: &str) -> i32 {
        if s.len() == 1 {
            return Self::roman_char_to_int(s.as_bytes()[0] as char);
        }

        let mut n = 0;
        let mut windows = Self::str_windows2(&s).enumerate();
        let last_i = windows.len().saturating_sub(1);

        while let Some((i, c)) = windows.next() {
            let (converted, skip) = Self::roman_array_to_int(c);
            n += converted;
            if skip {
                let w = windows.next();
                if i + 1 == last_i {
                    // SAFETY: There is at least one element in the iterator
                    n += Self::roman_char_to_int(unsafe { w.unwrap_unchecked() }.1[1]);
                }
            } else if i == last_i {
                n += Self::roman_char_to_int(c[1]);
            }
        }

        n
    }

    pub fn roman_to_int(s: String) -> i32 {
        Self::roman_to_int1(&s)
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
        assert_eq!(Solution::roman_to_int1("III"), 3);
    }

    #[bench]
    fn bench_example1(b: &mut Bencher) {
        b.iter(|| {
            test::black_box(Solution::roman_to_int1(test::black_box("III")));
        })
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::roman_to_int1("LVIII"), 58);
    }

    #[bench]
    fn bench_example2(b: &mut Bencher) {
        b.iter(|| {
            test::black_box(Solution::roman_to_int1(test::black_box("LVIII")));
        })
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::roman_to_int1("MCMXCIV"), 1994);
    }

    #[bench]
    fn bench_example3(b: &mut Bencher) {
        b.iter(|| {
            test::black_box(Solution::roman_to_int1(test::black_box("MCMXCIV")));
        })
    }

    #[test]
    fn case1497() {
        assert_eq!(Solution::roman_to_int1("DCXXI"), 621);
    }

    #[bench]
    fn bench_case1497(b: &mut Bencher) {
        b.iter(|| {
            test::black_box(Solution::roman_to_int1(test::black_box("DCXXI")));
        })
    }

    #[test]
    fn case3800() {
        assert_eq!(Solution::roman_to_int1("MDCXCV"), 1695);
    }

    #[bench]
    fn bench_case3800(b: &mut Bencher) {
        b.iter(|| {
            test::black_box(Solution::roman_to_int1(test::black_box("MDCXCV")));
        })
    }

    #[test]
    fn case3992() {
        assert_eq!(Solution::roman_to_int1("D"), 500);
    }

    #[bench]
    fn bench_case3992(b: &mut Bencher) {
        b.iter(|| {
            test::black_box(Solution::roman_to_int1(test::black_box("D")));
        })
    }

    #[test]
    fn number_4() {
        assert_eq!(Solution::roman_to_int1("IV"), 4);
    }

    #[bench]
    fn bench_number_4(b: &mut Bencher) {
        b.iter(|| {
            test::black_box(Solution::roman_to_int1(test::black_box("IV")));
        })
    }

    #[test]
    fn example1_alternative() {
        assert_eq!(Solution::roman_to_int2("III"), 3);
    }

    #[bench]
    fn bench_example1_alternative(b: &mut Bencher) {
        b.iter(|| {
            test::black_box(Solution::roman_to_int2(test::black_box("III")));
        })
    }

    #[test]
    fn example2_alternative() {
        assert_eq!(Solution::roman_to_int2("LVIII"), 58);
    }

    #[bench]
    fn bench_example2_alternative(b: &mut Bencher) {
        b.iter(|| {
            test::black_box(Solution::roman_to_int2(test::black_box("LVIII")));
        })
    }

    #[test]
    fn example3_alternative() {
        assert_eq!(Solution::roman_to_int2("MCMXCIV"), 1994);
    }

    #[bench]
    fn bench_example3_alternative(b: &mut Bencher) {
        b.iter(|| {
            test::black_box(Solution::roman_to_int2(test::black_box("MCMXCIV")));
        })
    }

    #[test]
    fn case1497_alternative() {
        assert_eq!(Solution::roman_to_int2("DCXXI"), 621);
    }

    #[bench]
    fn bench_case1497_alternative(b: &mut Bencher) {
        b.iter(|| {
            test::black_box(Solution::roman_to_int2(test::black_box("DCXXI")));
        })
    }

    #[test]
    fn case3800_alternative() {
        assert_eq!(Solution::roman_to_int2("MDCXCV"), 1695);
    }

    #[bench]
    fn bench_case3800_alternative(b: &mut Bencher) {
        b.iter(|| {
            test::black_box(Solution::roman_to_int2(test::black_box("MDCXCV")));
        })
    }

    #[test]
    fn case3992_alternative() {
        assert_eq!(Solution::roman_to_int2("D"), 500);
    }

    #[bench]
    fn bench_case3992_alternative(b: &mut Bencher) {
        b.iter(|| {
            test::black_box(Solution::roman_to_int2(test::black_box("D")));
        })
    }

    #[test]
    fn number_4_alternative() {
        assert_eq!(Solution::roman_to_int2("IV"), 4);
    }

    #[bench]
    fn bench_number_4_alternative(b: &mut Bencher) {
        b.iter(|| {
            test::black_box(Solution::roman_to_int2(test::black_box("IV")));
        })
    }
}
