/// https://leetcode.com/problems/search-insert-position/

struct Solution;
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut start = 0;
        let mut end = nums.len() - 1;
        while start <= end {
            let mid = (start + end) / 2;
            if target > nums[mid] {
                start = mid + 1;
            } else if mid == 0 {
                return 0;
            } else {
                end = mid - 1;
            }
        }
        start as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::search_insert(vec![5, 7, 8, 10], 8), 2);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::search_insert(vec![], 6), 0);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::search_insert(vec![1, 2, 2], 2), 1);
    }
}
