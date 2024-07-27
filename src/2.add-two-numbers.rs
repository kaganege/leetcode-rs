/*
 * @lc app=leetcode id=2 lang=rust
 *
 * [2] Add Two Numbers
 */

#![cfg_attr(not(test), allow(unused))]

use super::Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>,
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode { next: None, val }
  }
}

// @lc code=start
const DEFAULT_NODE: ListNode = ListNode { val: 0, next: None };

impl Solution {
  pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
  ) -> Option<Box<ListNode>> {
    fn add_two_nodes(
      n1: Option<Box<ListNode>>,
      n2: Option<Box<ListNode>>,
      leading: i32,
    ) -> Option<Box<ListNode>> {
      match (n1, n2) {
        (None, None) if leading == 0 => None,
        (None, None) if leading > 0 => Some(Box::new(ListNode::new(leading))),
        (n1, n2) => {
          let n1 = n1.unwrap_or_else(|| Box::new(DEFAULT_NODE));
          let n2 = n2.unwrap_or_else(|| Box::new(DEFAULT_NODE));

          let sum = n1.val + n2.val + leading;
          let node = ListNode {
            val: sum % 10,
            next: add_two_nodes(n1.next, n2.next, sum / 10),
          };

          Some(Box::new(node))
        }
      }
    }

    add_two_nodes(l1, l2, 0)
  }
}
// @lc code=end

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;

  macro_rules! example {
    (
      l1 = [$($l1:expr),*];
      l2 = [$($l2:expr),*]$(;)?
    ) => {
      Solution::add_two_numbers(Some(list_to_node(vec![$($l1),*])), Some(list_to_node(vec![$($l2),*])))
    };
  }

  macro_rules! assert_example {
    (
      Input: l1 = $l1:tt, l2 = $l2:tt;
      Output: [$($o:expr),*]$(;)?
    ) => {
      assert_eq!(
        example! {
          l1 = $l1;
          l2 = $l2;
        },
        Some(list_to_node(vec![$($o),*]))
      )
    };
  }

  fn list_to_node(list: Vec<i32>) -> Box<ListNode> {
    let mut list = list.into_iter();
    let mut head_node = Box::new(ListNode::new(list.next().expect("List should be nonempty")));
    let mut current = &mut head_node;

    for val in list {
      current.next = Some(Box::new(ListNode::new(val)));
      current = current.next.as_mut().unwrap();
    }

    head_node
  }

  #[test]
  fn example1() {
    assert_example! {
      Input: l1 = [2,4,3], l2 = [5,6,4];
      Output: [7,0,8];
    }
  }

  #[bench]
  fn bench_example1(b: &mut Bencher) {
    b.iter(|| {
      example! {
        l1 = [2,4,3];
        l2 = [5,6,4];
      }
    })
  }

  #[test]
  fn example2() {
    assert_example! {
      Input: l1 = [0], l2 = [0];
      Output: [0];
    }
  }

  #[bench]
  fn bench_example2(b: &mut Bencher) {
    b.iter(|| {
      example! {
        l1 = [0];
        l2 = [0];
      }
    })
  }

  #[test]
  fn example3() {
    assert_example! {
      Input: l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9];
      Output: [8,9,9,9,0,0,0,1];
    }
  }

  #[bench]
  fn bench_example3(b: &mut Bencher) {
    b.iter(|| {
      example! {
        l1 = [9,9,9,9,9,9,9];
        l2 = [9,9,9,9];
      }
    })
  }
}
