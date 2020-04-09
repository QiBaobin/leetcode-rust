/// https://leetcode.com/problems/remove-duplicates-from-sorted-array/

pub struct Solution;
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let n = nums.len();
        if n < 2 {
            return n as i32;
        }

        let mut i = 0;
        let mut j = 1;
        while j < n {
            if nums[j] != nums[i] {
                i += 1;
                if j != i {
                    nums[i] = nums[j];
                }
            }
            j += 1;
        }
        (i + 1) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::remove_duplicates(&mut vec![1, 2, 3]), 3);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::remove_duplicates(&mut vec![1, 1, 2, 3]), 3);
    }
}
