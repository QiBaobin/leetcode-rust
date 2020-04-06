/// https://leetcode.com/problems/merge-k-sorted-lists/

pub struct Solution;
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
impl Solution {
    pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        Self::merge(&mut lists[..])
    }
    pub fn merge(lists: &mut [Option<Box<ListNode>>]) -> Option<Box<ListNode>> {
        match lists.len() {
            0 => None,
            1 => lists[0].take(),
            2 => Self::merge_two_lists(lists[0].take(), lists[1].take()),
            n => {
                let mid = n / 2;
                Self::merge_two_lists(
                    Self::merge(&mut lists[0..mid]),
                    Self::merge(&mut lists[mid..]),
                )
            }
        }
    }
    pub fn merge_two_lists(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head = None;
        let mut tail = &mut head;
        while l1.is_some() && l2.is_some() {
            if l1.as_ref().unwrap().val > l2.as_ref().unwrap().val {
                tail.replace(l2.take().unwrap());
                tail = &mut tail.as_mut().unwrap().next;
                l2 = tail.take();
            } else {
                tail.replace(l1.take().unwrap());
                tail = &mut tail.as_mut().unwrap().next;
                l1 = tail.take();
            }
        }

        if l1.is_some() {
            tail.replace(l1.take().unwrap());
        } else if l2.is_some() {
            tail.replace(l2.take().unwrap());
        }

        head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            create_nodes_from(&[1, 1, 2, 3, 4, 4]),
            Solution::merge_k_lists(vec![
                create_nodes_from(&[1, 2, 4]),
                create_nodes_from(&[1, 3, 4])
            ])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(None, Solution::merge_k_lists(vec![None]));
    }

    #[test]
    fn test_3() {
        assert_eq!(
            create_nodes_from(&[1, 2, 3]),
            Solution::merge_k_lists(vec![create_nodes_from(&[1, 2, 3])])
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(
            create_nodes_from(&[1, 1, 2, 3, 4, 4, 4, 5, 6]),
            Solution::merge_k_lists(vec![
                create_nodes_from(&[1, 2, 4]),
                create_nodes_from(&[4, 5, 6]),
                create_nodes_from(&[1, 3, 4])
            ])
        );
    }

    fn create_nodes_from(nums: &[i32]) -> Option<Box<ListNode>> {
        if nums.is_empty() {
            None
        } else {
            let mut root = Some(Box::new(ListNode::new(nums[0])));
            let mut current = root.as_mut().unwrap();
            for n in nums.get(1..).unwrap() {
                current.next = Some(Box::new(ListNode::new(*n)));
                current = current.next.as_mut().unwrap();
            }
            root
        }
    }
}
