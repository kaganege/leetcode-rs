/*
 * @lc app=leetcode id=21 lang=rust
 *
 * [21] Merge Two Sorted Lists
 */

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    pub fn from_slice(slice: &[i32]) -> Self {
        let mut head = Self::new(slice[0]);
        let mut node = &mut head;

        for val in &slice[1..] {
            node.next = Some(Self::new_boxed(*val));
            node = unsafe { node.next.as_mut().unwrap_unchecked() };
        }

        head
    }

    pub fn into_boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

pub struct Solution;

// @lc code=start
impl ListNode {
    #[inline]
    pub fn new_boxed(val: i32) -> Box<Self> {
        Box::new(Self::new(val))
    }
}

impl Solution {
    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new_boxed(0);
        let mut tail = &mut dummy;

        'main: loop {
            let take_l1 = 'comp: {
                let Some(l1) = list1.as_ref() else {
                    if list2.is_none() {
                        break 'main;
                    }

                    break 'comp false;
                };
                let Some(l2) = list2.as_ref() else {
                    break 'comp true;
                };

                l1.val <= l2.val
            };

            let next = if take_l1 {
                // SAFETY: We checked that `list1` is not `None` above
                let mut n = unsafe { list1.take().unwrap_unchecked() };
                list1 = n.next.take();
                n
            } else {
                // SAFETY: We checked that `list2` is not `None` above
                let mut n = unsafe { list2.take().unwrap_unchecked() };
                list2 = n.next.take();
                n
            };

            tail.next = Some(next);
            tail = tail.next.as_mut().unwrap();
        }

        dummy.next
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
        let list1 = ListNode::from_slice(&[1, 2, 4]).into_boxed();
        let list2 = ListNode::from_slice(&[1, 3, 4]).into_boxed();
        let expected = ListNode::from_slice(&[1, 1, 2, 3, 4, 4]).into_boxed();

        assert_eq!(
            Solution::merge_two_lists(Some(list1), Some(list2)),
            Some(expected)
        );
    }

    #[bench]
    fn bench_example1(b: &mut Bencher) {
        b.iter(|| {
            test::black_box(Solution::merge_two_lists(
                test::black_box(Some(ListNode::from_slice(&[1, 2, 4]).into_boxed())),
                test::black_box(Some(ListNode::from_slice(&[1, 3, 4]).into_boxed())),
            ))
        })
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::merge_two_lists(None, None), None);
    }

    #[bench]
    fn bench_example2(b: &mut Bencher) {
        b.iter(|| {
            test::black_box(Solution::merge_two_lists(
                test::black_box(None),
                test::black_box(None),
            ))
        })
    }

    #[test]
    fn example3() {
        let list = ListNode::from_slice(&[0]).into_boxed();

        assert_eq!(
            Solution::merge_two_lists(None, Some(list.clone())),
            Some(list)
        );
    }
}
