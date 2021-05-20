// https://leetcode.com/problems/jump-game-ii/

struct Solution;
impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut current = 0;
        let mut steps = 0;
        while current + 1 < nums.len() {
            steps += 1;
            let scope = current + nums[current] as usize + 1;
            if scope >= nums.len() {
                break;
            }
            current = (current + 1..scope)
                .max_by_key(|&i| i + nums[i] as usize)
                .unwrap_or(current + 1);
        }
        steps
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::jump(vec![2, 3, 1, 1, 4]), 2);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::jump(vec![2, 3, 1, 1, 4]), 2);
    }
}
