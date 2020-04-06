/// https://leetcode.com/problems/swap-nodes-in-pairs/

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
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0);
        dummy.next = head;
        let mut n = &mut dummy;
        while n.next.is_some() && n.next.as_ref().unwrap().next.is_some() {
            let mut n1 = n.next.take().unwrap();
            let mut n2 = n1.next.take().unwrap();
            let n3 = n2.next;

            n1.next = n3;
            n2.next = Some(n1);
            n.next = Some(n2);
            n = n.next.as_mut().unwrap().next.as_mut().unwrap();
        }
        dummy.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            create_nodes_from(&[2, 1, 4, 3]),
            Solution::swap_pairs(create_nodes_from(&[1, 2, 3, 4]))
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            create_nodes_from(&[2]),
            Solution::swap_pairs(create_nodes_from(&[2]))
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
