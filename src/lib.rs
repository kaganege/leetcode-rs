#![feature(test)]

extern crate test;

pub struct Solution;

#[path = "2.add-two-numbers.rs"]
pub mod add_two_numbers;
#[path = "3.longest-substring-without-repeating-characters.rs"]
pub mod longest_substring_without_repeating_characters;
#[path = "4.median-of-two-sorted-arrays.rs"]
pub mod median_of_two_sorted_arrays;
#[path = "1.two-sum.rs"]
pub mod two_sum;
