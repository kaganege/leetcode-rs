/*
 * @lc app=leetcode id=13 lang=rust
 *
 * [13] Roman to Integer
 */

pub struct Solution;

// @lc code=start
impl Solution {
    fn str_windows2(s: &str) -> impl ExactSizeIterator<Item = [char; 2]> {
        s.as_bytes().windows(2).map(|b| {
            // SAFETY: The bytes are ASCII characters
            unsafe {
                [
                    char::from_u32_unchecked(b[0] as u32),
                    char::from_u32_unchecked(b[1] as u32),
                ]
            }
        })
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

    pub fn roman_to_int(s: String) -> i32 {
        if s.len() == 1 {
            return Self::roman_char_to_int(
                // SAFETY: The length is 1 and the first character is ASCII
                unsafe { s.chars().next().unwrap_unchecked() },
            );
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
}
// @lc code=end

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;
    use test::Bencher;

    #[test]
    fn example1() {
        assert_eq!(Solution::roman_to_int("III".to_string()), 3);
    }

    #[bench]
    fn bench_example1(b: &mut Bencher) {
        b.iter(|| {
            Solution::roman_to_int("III".to_string());
        })
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::roman_to_int("LVIII".to_string()), 58);
    }

    #[bench]
    fn bench_example2(b: &mut Bencher) {
        b.iter(|| {
            Solution::roman_to_int("LVIII".to_string());
        })
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994);
    }

    #[bench]
    fn bench_example3(b: &mut Bencher) {
        b.iter(|| {
            Solution::roman_to_int("MCMXCIV".to_string());
        })
    }

    #[test]
    fn case1497() {
        assert_eq!(Solution::roman_to_int("DCXXI".to_string()), 621);
    }

    #[bench]
    fn bench_case1497(b: &mut Bencher) {
        b.iter(|| {
            Solution::roman_to_int("DCXXI".to_string());
        })
    }

    #[test]
    fn case3800() {
        assert_eq!(Solution::roman_to_int("MDCXCV".to_string()), 1695);
    }

    #[bench]
    fn bench_case3800(b: &mut Bencher) {
        b.iter(|| {
            Solution::roman_to_int("MDCXCV".to_string());
        })
    }

    #[test]
    fn case3992() {
        assert_eq!(Solution::roman_to_int("D".to_string()), 500);
    }

    #[bench]
    fn bench_case3992(b: &mut Bencher) {
        b.iter(|| {
            Solution::roman_to_int("D".to_string());
        })
    }

    #[test]
    fn number_4() {
        assert_eq!(Solution::roman_to_int("IV".to_string()), 4);
    }

    #[bench]
    fn bench_number_4(b: &mut Bencher) {
        b.iter(|| {
            Solution::roman_to_int("IV".to_string());
        })
    }
}
