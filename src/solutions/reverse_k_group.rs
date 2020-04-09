/// https://leetcode.com/problems/reverse-nodes-in-k-group/

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
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if k < 2 {
            return head;
        }

        let mut head = head;
        // copy from below
        // https://leetcode.com/problems/reverse-nodes-in-k-group/discuss/373014/Rust-0-ms-non-recursive
        let mut p_head: Option<Box<ListNode>> = None;
        let mut prev_tail = &mut p_head;
        let mut k_group: Option<Box<ListNode>> = None;
        loop {
            for k_group_len in 0..k {
                if let Some(mut node) = head {
                    head = node.next.take();
                    node.next = k_group;
                    k_group = Some(node);
                } else {
                    let mut reverted_k_group: Option<Box<ListNode>> = None;
                    for _ in 0..k_group_len {
                        let mut node = k_group.unwrap();
                        k_group = node.next.take();
                        node.next = reverted_k_group;
                        reverted_k_group = Some(node);
                    }
                    *prev_tail = reverted_k_group;
                    return p_head
                }
            }
            *prev_tail = k_group;
            for _ in 0..k {
                prev_tail = &mut prev_tail.as_mut().unwrap().next;
            }
            k_group = None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            create_nodes_from(&[2, 1, 4, 3, 5]),
            Solution::reverse_k_group(create_nodes_from(&[1, 2, 3, 4, 5]), 2)
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            create_nodes_from(&[3, 2, 1, 4, 5]),
            Solution::reverse_k_group(create_nodes_from(&[1, 2, 3, 4, 5]), 3)
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
