/// https://leetcode.com/problems/next-permutation/

struct Solution;
impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let n = nums.len();
        for i in 1..n {
            let num = nums[n - i - 1];
            if num < nums[n - i] {
                nums[n - i..].sort();
                for j in n - i.. {
                    if num < nums[j] {
                        nums.swap(n - i - 1, j);
                        return;
                    }
                }
            }
        }
        nums.reverse();
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut nums = vec![1, 2, 3];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![1, 3, 2]);
    }

    #[test]
    fn test_2() {
        let mut nums = vec![3, 2, 1];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![1, 2, 3]);
    }

    #[test]
    fn test_3() {
        let mut nums = vec![1, 1, 5];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![1, 5, 1]);
    }

    #[test]
    fn test_4() {
        let mut nums = vec![1, 3, 2];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![2, 1, 3]);
    }

    #[test]
    fn test_5() {
        let mut nums = vec![1, 3, 2, 2];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![2, 1, 2, 3]);
    }
}
