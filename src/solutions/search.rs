/// https://leetcode.com/problems/search-in-rotated-sorted-array/

struct Solution;
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() {
            return -1;
        }

        use std::cmp::Ordering;
        let mut start = 0;
        let mut end = nums.len() - 1;
        while start + 1 < end {
            let mut mid = start + (end - start) / 2;
            match target.cmp(&nums[mid]) {
                Ordering::Equal => {
                    return mid as i32;
                }
                Ordering::Greater => {
                    if nums[mid] > nums[start] || target <= nums[end] {
                        start = mid + 1;
                    } else {
                        end = mid - 1;
                    }
                }
                Ordering::Less => {
                    if nums[mid] < nums[end] || target >= nums[start] {
                        end = mid - 1;
                    } else {
                        start = mid + 1;
                    }
                }
            }
        }
        if target == nums[start] {
            start as i32
        } else if target == nums[end] {
            end as i32
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 1), 5);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 8, 1, 2], 8), 4);
    }
}
