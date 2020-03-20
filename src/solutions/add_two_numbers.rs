/// https://leetcode.com/problems/add-two-numbers/
//Definition for singly-linked list.

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
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head = ListNode::new(0);
        let mut l = l1.as_ref();
        let mut r = l2.as_ref();

        let mut carry = 0;
        let mut current = &mut head;
        let empty = &Box::new(ListNode::new(0));
        loop {
            match (l, r) {
                (None, None) => {
                    if carry > 0 {
                        current.next = Some(Box::new(ListNode::new(carry)));
                    }
                    return head.next;
                }
                _ => {
                    let n1 = l.unwrap_or(empty);
                    let n2 = r.unwrap_or(empty);
                    l = n1.next.as_ref();
                    r = n2.next.as_ref();

                    let v = carry + n1.val + n2.val;
                    current.next = Some(Box::new(ListNode::new(v % 10)));
                    current = current.next.as_mut().unwrap().as_mut();
                    carry = v / 10;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            create_nodes_from(&[7, 0, 8]),
            Solution::add_two_numbers(create_nodes_from(&[2, 4, 3]), create_nodes_from(&[5, 6, 4]),)
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            create_nodes_from(&[0, 8, 0, 8, 0, 1]),
            Solution::add_two_numbers(
                create_nodes_from(&[1, 2, 4, 3, 6]),
                create_nodes_from(&[9, 5, 6, 4, 4]),
            )
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            create_nodes_from(&[1, 8]),
            Solution::add_two_numbers(
                create_nodes_from(&[1, 8]),
                create_nodes_from(&[]),
            )
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
