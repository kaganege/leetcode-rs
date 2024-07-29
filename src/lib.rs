#![feature(test)]

extern crate test;

pub struct Solution;

#[path = "2.add-two-numbers.rs"]
mod add_two_numbers;
#[path = "3.longest-substring-without-repeating-characters.rs"]
mod longest_substring_without_repeating_characters;
#[path = "1.two-sum.rs"]
mod two_sum;
