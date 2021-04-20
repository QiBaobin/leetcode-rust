/// https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/

struct Solution;
impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if let Ok(i) = nums.binary_search(&target) {
            use std::cmp::Ordering;
            vec![
                nums[..i]
                    .binary_search_by(|&n| {
                        if n < target {
                            Ordering::Less
                        } else {
                            Ordering::Greater
                        }
                    })
                    .map_or_else(|e| e, |o| o + 1) as i32,
                nums[i + 1..]
                    .binary_search_by(|&n| {
                        if n > target {
                            Ordering::Greater
                        } else {
                            Ordering::Less
                        }
                    })
                    .map_or_else(|e| e + i, |o| o) as i32,
            ]
        } else {
            vec![-1, -1]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8),
            vec![3, 4]
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 6),
            vec![-1, -1]
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::search_range(vec![1, 2, 2], 1), vec![0, 0]);
    }
}
