#![feature(test)]

extern crate test;

pub(crate) struct Solution;

#[path = "2.add-two-numbers.rs"]
mod add_two_numbers;
#[path = "1.two-sum.rs"]
mod two_sum;
