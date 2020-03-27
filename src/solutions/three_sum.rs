/// https://leetcode.com/problems/3sum/

struct Solution;
impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        use std::cmp::Ordering::{Equal, Greater};
        let mut result = Vec::new();
        if nums.len() < 3 {
            return result;
        }

        nums.sort();
        let zero_index = nums.binary_search(&0).unwrap_or_else(|e| e);
        let end = std::cmp::min(zero_index + 1, nums.len() - 2);
        let mut i = 0;
        while i < end {
            let n = 0 - nums[i];
            let mut j = i + 1;
            let mut k = nums.len() - 1;

            while j < k {
                match (nums[j] + nums[k]).cmp(&n) {
                    Equal => {
                        result.push(vec![nums[i], nums[j], nums[k]]);
                        Self::advance(&nums, &mut j, k, true);
                        Self::advance(&nums, &mut k, j, false);
                    }
                    Greater => {
                        Self::advance(&nums, &mut k, j, false);
                    }
                    _ => {
                        Self::advance(&nums, &mut j, k, true);
                    }
                }
            }
            Self::advance(&nums, &mut i, end, true);
        }

        result
    }

    #[inline]
    fn advance(s: &[i32], i: &mut usize, end: usize, forward: bool) {
        let current = s[*i];
        while *i != end && s[*i] == current {
            if forward {
                *i += 1;
            } else {
                *i -= 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(vec![vec![0, 0, 0]], Solution::three_sum(vec![0, 0, 0]));
    }

    #[test]
    fn test_2() {
        assert_eq!(
            vec![vec![-1, -1, 2], vec![-1, 0, 1]],
            Solution::three_sum(vec![-1, 0, 1, 2, -1, -4])
        );
    }

    #[test]
    fn test_3() {
        assert!(Solution::three_sum(vec![1, 2, -1, -2]).is_empty());
    }

    #[test]
    fn test_4() {
        assert_eq!(
            vec![vec![-1, 0, 1]],
            Solution::three_sum(vec![-1, 1, 0, -1])
        );
    }
}
