/// https://leetcode.com/problems/two-sum/
use std::cmp::Ordering;

pub struct Solution;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.len() < 2 {
            return vec![];
        }

        let mut sorted_with_indices: Vec<(usize, &i32)> = nums.iter().enumerate().collect();
        sorted_with_indices.sort_by(|a, b| (*a.1).cmp(b.1));
        let mut i = 0;
        let mut j = nums.len() - 1;

        loop {
            let (i1, n1) = sorted_with_indices[i];
            let (i2, n2) = sorted_with_indices[j];

            match (*n1 + *n2).cmp(&target) {
                Ordering::Equal => return vec![i1 as i32, i2 as i32],
                Ordering::Less => i += 1,
                _ => j -= 1,
            }

            if i == j {
                return vec![];
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(vec![0, 1], Solution::two_sum(vec![2, 7, 11, 15], 9));
    }

    #[test]
    fn test_2() {
        assert_eq!(vec![1, 2], Solution::two_sum(vec![3, 2, 4], 6));
    }
}
