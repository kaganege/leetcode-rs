/*
 * @lc app=leetcode id=2 lang=rust
 *
 * [2] Add Two Numbers
 */

#![no_main]

struct Solution;

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
          let n1 = n1.unwrap_or(Box::new(DEFAULT_NODE));
          let n2 = n2.unwrap_or(Box::new(DEFAULT_NODE));

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
