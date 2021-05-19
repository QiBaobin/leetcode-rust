// https://leetcode.com/problems/first-missing-positive/

struct Solution;
impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        for i in 0..n {
            while nums[i] > 0 && nums[i] <= n as i32 && nums[i] != nums[nums[i] as usize - 1] {
                let j = nums[i] as usize - 1;
                nums.swap(i, j);
            }
        }
        for i in 0..n {
            if nums[i] != i as i32 + 1 {
                return i as i32 + 1;
            }
        }
        n as i32 + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::first_missing_positive(vec![2, 3, 6, 7]), 1);
    }
}
